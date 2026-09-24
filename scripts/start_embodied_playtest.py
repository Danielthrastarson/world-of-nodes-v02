#!/usr/bin/env python3
"""Launch E0 with local records; never overwrite an earlier session."""
from datetime import datetime, timezone
import hashlib, json, os, subprocess
from pathlib import Path
root = Path(__file__).resolve().parents[1]
binary = root / 'client/target/release/embodied_client'
if not binary.exists():
    raise SystemExit('Build first: cargo build --release --manifest-path client/Cargo.toml --bin embodied_client')
folder = root / 'research/human-playtest' / datetime.now(timezone.utc).strftime('e0-%Y%m%dT%H%M%S.%fZ')
folder.mkdir(parents=True, exist_ok=False)
command = [str(binary), '--log-dir', str(folder / 'session')]
manifest = {'revision': subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),
    'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'command': command,
    'seed': 0, 'scenario': 'E0 controlled 40-person fixture', 'debug_truth': False,
    'starts_paused': True, 'automated_inputs': False, 'display': os.environ.get('DISPLAY'),
    'recording': 'Local commands, UI state and performance; no screen, microphone or external telemetry'}
(folder/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
with (folder/'process.log').open('x') as log:
    p = subprocess.Popen(command,cwd=root,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
print(f'E0 started (PID {p.pid}). Records: {folder}')
print('Close normally or Escape to finalize independent replay.')
