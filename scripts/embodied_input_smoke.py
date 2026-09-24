#!/usr/bin/env python3
"""Actual E0 mouse/key test in its own Xvfb; never target a human window."""
import csv,json,os,subprocess,sys,time
from pathlib import Path
root=Path(__file__).resolve().parents[1]
if '--inside-xvfb' not in sys.argv:
    env=dict(os.environ,LIBGL_ALWAYS_SOFTWARE='1');env.pop('WAYLAND_DISPLAY',None)
    raise SystemExit(subprocess.call(['xvfb-run','-a','-s','-screen 0 1440x900x24',sys.executable,__file__,sys.argv[1],'--inside-xvfb'],env=env))
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
session=out/'session'
from playtest_journal_smoke import WindowInput
device=WindowInput()
def xd(*args):
    action=args[0]
    if action=='search':
        w=device.find_client();return str(w) if w else ''
    if action=='windowfocus':device.focus(int(args[1]))
    elif action=='key':device.key(args[1])
    elif action=='mousemove':device.move(int(args[1]),int(args[2]))
    elif action in ('keydown','keyup'):
        code=device.x.XKeysymToKeycode(device.display,device.x.XStringToKeysym(args[1].encode()))
        device.t.XTestFakeKeyEvent(device.display,code,action=='keydown',0)
        device.x.XFlush(device.display)
    elif action in ('mousedown','mouseup','click'):
        for state in ((1,0) if action=='click' else (int(action=='mousedown'),)):
            device.t.XTestFakeButtonEvent(device.display,int(args[1]),state,0)
            device.x.XFlush(device.display);time.sleep(.09)
    return '' 
def wait(fn):
    end=time.monotonic()+30
    while time.monotonic()<end:
        try:
            result=fn()
            if result:return result
        except (OSError,subprocess.CalledProcessError,IndexError):pass
        time.sleep(.1)
    raise RuntimeError('timed out')
def row():
    if not (session/'ui.tsv').exists():return None
    with (session/'ui.tsv').open() as f: rows=list(csv.DictReader(f,delimiter='\t'))
    rows=[r for r in rows if None not in r.values()]
    return rows[-1] if rows else None
def state(third):
    def ready():
        r=row()
        return r if r and r['third_person']==str(third).lower() else None
    return wait(ready)
def key(k):xd('key',k)
stages=[]
with (out/'client.log').open('x') as log:
    p=subprocess.Popen([str(root/'client/target/release/embodied_client'),'--log-dir',str(session)],cwd=root,stdout=log,stderr=subprocess.STDOUT)
    try:
        window=wait(lambda:xd('search','--name','E0 embodied ruler prototype').splitlines()[0]);xd('windowfocus',window)
        initial=wait(row);stages.append({'name':'initial','state':initial})
        assert initial['paused']=='true'
        key('v');entered=state(True);stages.append({'name':'entered','state':entered})
        assert entered['state_hash']==initial['state_hash'] and entered['event_hash']==initial['event_hash']
        # Camera orbit and zoom while paused may not advance the world.
        xd('mousemove',900,450);xd('mousedown',3);xd('mousemove',930,460);xd('mouseup',3);xd('click',4)
        time.sleep(.3);assert row()['state_hash']==initial['state_hash']
        key('space');xd('keydown','d');time.sleep(.6);xd('keyup','d');key('space');time.sleep(.3)
        moved=row();stages.append({'name':'walked','state':moved});assert moved['pose']!=initial['pose']
        # Inspect a local person; record discovery rather than assert a specific ID.
        xd('mousemove',720,445);xd('click',1);time.sleep(.3);stages.append({'name':'inspect','state':row()})
        key('q');time.sleep(.2);key('v');state(False);key('w');time.sleep(.2)
        xd('mousemove',500,813);xd('click',1);time.sleep(.3)
        key('equal');time.sleep(.2);stages.append({'name':'order-and-speed','state':row()})
        # Untimed UI performance sample with no screenshots/readback.
        time.sleep(4)
        key('Escape');p.wait(timeout=30)
        assert p.returncode==0
        result=json.loads((session/'result.json').read_text());assert result['replay_equal']
        inputs=(session/'inputs.txt').read_text();assert 'Walk' in inputs and 'Commission' in inputs and 'ObserveReport' in inputs
        (out/'result.json').write_text(json.dumps({'status':'PASS','isolated_display':os.environ['DISPLAY'],'stages':stages,'replay':result},indent=2)+'\n')
    finally:
        if p.poll() is None:p.terminate();p.wait(timeout=10)
