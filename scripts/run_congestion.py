#!/usr/bin/env python3
"""Reproduce the bounded congestion study into a new directory, never overwrite."""
import gzip
from pathlib import Path
import subprocess
import sys

root=Path(sys.argv[1])
assert not root.exists(),'Use a new output directory'
subprocess.run(['target/release/congestion_probe',str(root)],check=True)
subprocess.run(['target/release/congestion_probe',str(root),'selected-traces'],check=True)
for arm in ['reactive','forecast_corrected']:
    subprocess.run([sys.executable,'scripts/run_persistence.py',str(root),arm,'30','forecast-audit'],check=True)
subprocess.run([sys.executable,'scripts/analyze_congestion.py',str(root)],check=True)
for p in list(root.glob('*.trace'))+list(root.glob('*-forecasts.tsv')):
    with p.open('rb') as source,p.with_suffix(p.suffix+'.gz').open('wb') as output:
        with gzip.GzipFile(fileobj=output,mode='wb',filename='',mtime=0) as target:
            for chunk in iter(lambda:source.read(1024*1024),b''):target.write(chunk)
    p.unlink()
