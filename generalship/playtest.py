#!/usr/bin/env python3
"""Run/stop the temporary Internet playtest; serves only the game on localhost:8080."""
import json
import os
from pathlib import Path
import re
import secrets
import signal
import socket
import subprocess
import sys
import time
import urllib.request

ROOT = Path(__file__).resolve().parent.parent
RUNTIME = ROOT / 'generalship/runtime'
STATE = RUNTIME / 'playtest.json'
CODE = RUNTIME / 'host-code.txt'


def process_stamp(pid):
    try:
        # Linux start time prevents a stale PID file from targeting a reused PID.
        return Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()[19]
    except (OSError, IndexError):
        return None


def main():
    RUNTIME.mkdir(exist_ok=True)
    state = json.loads(STATE.read_text()) if STATE.exists() else {}
    alive = bool(state.get('pid') and process_stamp(state['pid']) == state.get('stamp'))
    action = sys.argv[1] if len(sys.argv) > 1 else 'start'
    if action == 'status':
        print(json.dumps({**state, 'running': alive}, indent=2))
        return
    if action == 'stop':
        if alive:
            os.kill(state['pid'], signal.SIGTERM)
            print('Stopping this playtest server and its public link.')
        else:
            print('No running playtest supervisor.')
        return
    if action != 'start':
        raise SystemExit('Usage: python3 generalship/playtest.py [start|status|stop]')
    if alive:
        raise SystemExit('Playtest is already running. Use status or stop.')
    tunnel_binary = RUNTIME / 'cloudflared'
    if not tunnel_binary.is_file():
        raise SystemExit('Place the official cloudflared Linux binary at generalship/runtime/cloudflared. See GENERALSHIP_F0.md.')
    with socket.socket() as check:
        if check.connect_ex(('127.0.0.1', 8080)) == 0:
            raise SystemExit('Port 8080 is in use. Stop the existing local server first.')
    subprocess.run(['cargo', 'build', '--locked', '--release', '--manifest-path', 'generalship/Cargo.toml', '--bin', 'generalship-server'], cwd=ROOT, check=True)
    if not CODE.exists():
        with os.fdopen(os.open(CODE, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), 'w') as f:
            f.write(secrets.token_urlsafe(24) + '\n')
    code = CODE.read_text().strip()
    if len(code) < 12:
        raise SystemExit('The private host code is too short.')
    env = dict(os.environ, G0_HOST_CODE=code, G0_BIND='127.0.0.1', PORT='8080', G0_DATA_DIR=str(RUNTIME))
    children = []
    log_files = []
    state = {'pid': os.getpid(), 'stamp': process_stamp(os.getpid()), 'url': None, 'started': time.time()}
    STATE.write_text(json.dumps(state))
    os.chmod(STATE, 0o600)

    def stop(_signum, _frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, stop)
    try:
        server_log = (RUNTIME / 'server.log').open('w')
        log_files.append(server_log)
        server = subprocess.Popen([str(ROOT / 'generalship/target/release/generalship-server')], cwd=ROOT, env=env, stdout=server_log, stderr=subprocess.STDOUT)
        children.append(server)
        for attempt in range(50):
            if server.poll() is not None:
                raise RuntimeError('Game server stopped. Check generalship/runtime/server.log.')
            try:
                with urllib.request.urlopen('http://127.0.0.1:8080/health', timeout=1) as response:
                    if json.load(response).get('ok'):
                        break
            except OSError:
                time.sleep(0.1)
        else:
            raise RuntimeError('Local server did not become ready.')
        log_path = RUNTIME / 'cloudflare.log'
        tunnel_log = log_path.open('w')
        log_files.append(tunnel_log)
        tunnel_env = {k: v for k, v in os.environ.items() if not k.startswith('TUNNEL_')}
        tunnel = subprocess.Popen([str(tunnel_binary), 'tunnel', '--config', '/dev/null', '--no-autoupdate', '--metrics', '127.0.0.1:0', '--url', 'http://127.0.0.1:8080'], cwd=ROOT, env=tunnel_env, stdout=tunnel_log, stderr=subprocess.STDOUT)
        children.append(tunnel)
        print(f'Host code file: {CODE}', flush=True)
        print('Temporary playtest: keep this process and computer running. Ctrl+C stops both services.', flush=True)
        while all(child.poll() is None for child in children):
            if state['url'] is None:
                match = re.search(r'https://[a-z0-9-]+\.trycloudflare\.com', log_path.read_text())
                if match:
                    state['url'] = match.group(0)
                    STATE.write_text(json.dumps(state))
                    print(f"PLAYTEST URL: {state['url']}", flush=True)
            time.sleep(1)
        raise RuntimeError('A playtest process stopped; check the runtime logs.')
    except KeyboardInterrupt:
        print('Playtest stopped; existing matches are aborted.', flush=True)
    finally:
        for child in reversed(children):
            if child.poll() is None:
                child.terminate()
        for child in reversed(children):
            try:
                child.wait(timeout=5)
            except subprocess.TimeoutExpired:
                child.kill()
                child.wait()
        for file in log_files:
            file.close()


if __name__ == '__main__':
    main()
