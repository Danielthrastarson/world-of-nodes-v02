#!/usr/bin/env python3
"""W0 renderer-only scale matrix. Does not expand the live simulation population."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--binary', type=Path, default=Path('client/target/release/world_render_probe'))
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    report = {'kind': 'synthetic renderer-only benchmark, not population scaling',
              'binary_sha256': hashlib.sha256(args.binary.read_bytes()).hexdigest(),
              'source_revision': subprocess.check_output(['git','rev-parse','HEAD'], text=True).strip(),
              'cases': [], 'status': 'RUNNING'}
    env = dict(os.environ, LIBGL_ALWAYS_SOFTWARE='1')
    env.pop('WAYLAND_DISPLAY', None)
    try:
        for count in (1000, 10000, 50000):
            for lod, zoom in [('close',1.05), ('medium',0.7), ('far',0.3)]:
                name=f'{count}-{lod}'
                command=['xvfb-run','-a','-s','-screen 0 1440x900x24',str(args.binary.resolve()),str((args.output/name).resolve()),str(count),str(zoom)]
                started=time.monotonic()
                with (args.output/(name+'.log')).open('x') as log:
                    process=subprocess.Popen(command,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
                    try:
                        code=process.wait(timeout=180)
                    except subprocess.TimeoutExpired:
                        os.killpg(process.pid,signal.SIGTERM)
                        process.wait(timeout=10)
                        raise RuntimeError(f'{name}: exceeded bounded 180-second budget')
                if code: raise RuntimeError(f'{name}: exit {code}; retained log')
                data=json.loads((args.output/name/'result.json').read_text())
                assert data['kernel_unchanged'] and data['kernel_persons']==18
                assert data['visible_records']==count
                report['cases'].append({'name':name,'command':command,'wall_seconds':time.monotonic()-started,'result':data})
                print(name, 'CPU ms', round(data['cpu_draw_mean_ms'],3), 'batches', data['captured_scene_batches'], flush=True)
                (args.output/'summary.json').write_text(json.dumps(report,indent=2)+'\n')
        report['status']='MEASURED'
    except Exception as error:
        report['status']='INCOMPLETE';report['error']=str(error)
        raise
    finally:
        (args.output/'summary.json').write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__': main()
