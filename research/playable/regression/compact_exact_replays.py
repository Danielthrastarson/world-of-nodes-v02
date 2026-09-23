#!/usr/bin/env python3
"""Discard only this new audit's proved duplicate arm output; retain hashes."""
from pathlib import Path
import gzip,hashlib,json,shutil

root=Path(__file__).resolve().parent
manifest=[]
def remove(path,reason):
    assert path.is_relative_to(root) and path.is_dir() and not path.is_symlink()
    for p in sorted(path.rglob('*')):
        if p.is_file():
            raw=p.read_bytes()
            decoded=gzip.decompress(raw) if p.suffix=='.gz' else raw
            manifest.append({'path':str(p.relative_to(root)), 'sha256':hashlib.sha256(raw).hexdigest(),
                             'content_sha256':hashlib.sha256(decoded).hexdigest(), 'bytes':len(raw),
                             'reason':reason})
    shutil.rmtree(path)

commands=json.loads((root/'command-chain-validation.json').read_text())
assert commands['status']=='PASS' and commands['runs']==600
for p in sorted((root/'command-chain').iterdir()):
    if p.is_dir(): remove(p,'All5artifacts exactly matched preserved F13B same baseline source; command-chain-validation.json links original paths/hashes')
combat=json.loads((root/'combat-baseline-validation.json').read_text())
changed={d['scope'].split('/')[0] for d in combat['differences']}
assert changed=={'bottleneck_defended','crossing_defended'} and combat['runs']==1440
for panel in ('combat','baseline-combat'):
    for p in sorted((root/panel).iterdir()):
        if p.is_dir() and p.name not in changed:
            remove(p,'Exact current vs983b5b5 baseline; historical semantic bytes also match; only already-existing disabledConfig fields differ from originalF11C parameters')
for p in root.glob('*-seed0-events.diff'):
    p.with_suffix(p.suffix+'.gz').write_bytes(gzip.compress(p.read_bytes(),mtime=0))
    p.unlink()
(root/'compaction.json').write_text(json.dumps({'policy':'Original historical data untouched; retain all changed current/baseline arms and strictfailurearchive',
    'removed_exact_duplicate_files':len(manifest),'removed':manifest},indent=2)+'\n')
print(len(manifest),'proved duplicate files discarded; all hashes retained')
