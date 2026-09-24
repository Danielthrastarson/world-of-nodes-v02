#!/usr/bin/env python3
"""Start a fresh local recorded game; never replace an earlier human session."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--directory', type=Path, help='fresh session directory')
    parser.add_argument('--seed', type=int, default=0)
    parser.add_argument('--remote', action='store_true')
    args = parser.parse_args()
    timestamp = datetime.now(timezone.utc)
    session = (args.directory or ROOT / 'research/human-playtest' /
               timestamp.strftime('recorded-%Y%m%dT%H%M%S.%fZ')).resolve()
    binary = ROOT / 'client/target/release/world-of-nodes-client'
    replay = ROOT / 'client/target/release/playtest_replay'
    if not binary.is_file() or not replay.is_file():
        raise SystemExit('Build first: cargo build --release --manifest-path client/Cargo.toml --bins')
    if not 0 <= args.seed <= 2**64 - 1:
        raise SystemExit('Seed must be an unsigned 64-bit integer.')
    session.parent.mkdir(parents=True, exist_ok=True)
    session.mkdir(mode=0o700)  # Fail rather than replace an earlier session.
    journal = session / 'journal'
    command = [str(binary), '--seed', str(args.seed), '--journal', str(journal),
               '--evidence', str(session / 'manual-checkpoint.txt'),
               '--capture', str(session / 'manual-screenshot.png')]
    if args.remote:
        command.append('--remote')
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    sources = list((ROOT / 'client/src').rglob('*.rs')) + [ROOT / 'client/Cargo.toml', ROOT / 'client/Cargo.lock']
    manifest = {
        'schema': 1, 'started_at_utc': timestamp.isoformat(), 'base_revision': revision,
        'accepted_implementation': '243d07b40deddfddf94c95842086412e0cbc7a6d',
        'accepted_evidence': 'd3958d5cc8eb0a711351d59d8ae036f0aa476271',
        'scenario': 'Remote' if args.remote else 'Local', 'seed': args.seed,
        'executable_sha256': sha(binary), 'replay_executable_sha256': sha(replay),
        'source_sha256': {str(p.relative_to(ROOT)): sha(p) for p in sorted(sources)},
        'debug_starts_on': False, 'starts_paused': True, 'automated_game_inputs': False,
        'recording_scope': 'game-window events and ruler-visible state; local files only',
        'command': command,
    }
    (session / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    with (session / 'process.log').open('x') as log:
        process = subprocess.Popen(command, cwd=ROOT, stdin=subprocess.DEVNULL,
                                   stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
    (session / 'started.json').write_text(json.dumps({'pid': process.pid, 'display': os.environ.get('DISPLAY')}, indent=2) + '\n')
    deadline = time.monotonic() + 12
    while time.monotonic() < deadline:
        if (journal / 'current.json').exists():
            print(json.dumps({'session': str(session), 'journal': str(journal), 'pid': process.pid,
                              'window': 'World of Nodes · Recorded playtest', 'recording_ready': True}, indent=2))
            return
        if process.poll() is not None:
            raise SystemExit(f'Client exited ({process.returncode}); see {session / "process.log"}')
        time.sleep(0.1)
    raise SystemExit(f'Client started but recording readiness not confirmed; see {session / "process.log"}')


if __name__ == '__main__':
    main()
