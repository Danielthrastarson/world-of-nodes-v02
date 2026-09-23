#!/usr/bin/env python3
"""Actual X11 input to an isolated client window; run inside xvfb-run.

No game-state mutations or direct input API calls. Output is screenshots and the
client's typed-command transcript with independent headless replay comparison.
Requires system X11/XTest and Python stdlib, not a desktop automation service.
"""
import ctypes as c
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

root=Path(__file__).resolve().parents[1]
out=Path(sys.argv[1]).resolve()
out.mkdir(parents=True,exist_ok=True)
mode=sys.argv[2] if len(sys.argv)>2 else 'commission'
x=c.CDLL('libX11.so.6');t=c.CDLL('libXtst.so.6')
x.XOpenDisplay.argtypes=[c.c_char_p];x.XOpenDisplay.restype=c.c_void_p
x.XStringToKeysym.argtypes=[c.c_char_p];x.XStringToKeysym.restype=c.c_ulong
x.XKeysymToKeycode.argtypes=[c.c_void_p,c.c_ulong];x.XKeysymToKeycode.restype=c.c_uint
x.XFlush.argtypes=[c.c_void_p]
t.XTestFakeKeyEvent.argtypes=[c.c_void_p,c.c_uint,c.c_int,c.c_ulong]
t.XTestFakeButtonEvent.argtypes=[c.c_void_p,c.c_uint,c.c_int,c.c_ulong]
t.XTestFakeMotionEvent.argtypes=[c.c_void_p,c.c_int,c.c_int,c.c_int,c.c_ulong]
d=x.XOpenDisplay(None)
assert d, 'X display unavailable'
events=[]
def key(name):
    code=x.XKeysymToKeycode(d,x.XStringToKeysym(name.encode()))
    assert code,name
    for state in [1,0]:
        t.XTestFakeKeyEvent(d,code,state,0);x.XFlush(d);time.sleep(.08)
    events.append({'key':name})
def move(px,py):
    t.XTestFakeMotionEvent(d,-1,px,py,0);x.XFlush(d);time.sleep(.10)
def click(px,py,button=1):
    move(px,py)
    for state in [1,0]:
        t.XTestFakeButtonEvent(d,button,state,0);x.XFlush(d);time.sleep(.08)
    events.append({'click':[px,py,button]})
def capture(name):
    key('F10');time.sleep(.3)
    shutil.copyfile(out/'current.png',out/f'{name}.png')
def evidence(name):
    key('F12');time.sleep(.4)
    shutil.copyfile(out/'current.txt',out/f'{name}.txt')
    assert 'replay_equal=true' in (out/f'{name}.txt').read_text()
command=[str(root/'client/target/release/world-of-nodes-client'),'--capture',str(out/'current.png'),'--evidence',str(out/'current.txt')]
with (out/'client.log').open('w') as log:
    p=subprocess.Popen(command,stdout=log,stderr=subprocess.STDOUT)
    try:
        time.sleep(2);assert p.poll() is None,'client exited'
        capture('initial')
        key('F1');key('w');key('1');key('Return');evidence('help-no-command')
        assert 'commands=[]' in (out/'help-no-command.txt').read_text()
        key('Escape')
        if mode=='commission':
            key('w');key('1');click(620,419);capture('confirmation');key('Return')
            evidence('issued');assert 'Commission' in (out/'issued.txt').read_text()
            click(1250,136);capture('orders')
        else:
            click(407,349);key('w');key('3');key('Return');evidence('hold')
            assert 'Hold' in (out/'hold.txt').read_text()
            click(820,419,3);evidence('context-move')
            assert 'Move' in (out/'context-move.txt').read_text()
            key('w');key('2');click(820,419);key('Return');evidence('attack')
            assert 'Attack' in (out/'attack.txt').read_text()
        click(960,58);key('space');time.sleep(12);key('space')
        evidence('after-running');capture('play-running')
        key('F2');capture('debug-running');key('F2')
        evidence('after-debug');assert (out/'after-running.txt').read_text().split('state_hash=')[1].splitlines()[0]==(out/'after-debug.txt').read_text().split('state_hash=')[1].splitlines()[0]
        if mode=='military':
            # W/4 uses the same local movement path, keeping incoming vulnerability.
            key('w');key('4');click(150,419);key('Return');evidence('withdraw-input')
            assert 'Withdraw' in (out/'withdraw-input.txt').read_text()
        evidence('before-camera')
        # Pan and zoom must not affect the paused authoritative state.
        move(600,500);click(600,500,4);key('Right');key('Down');capture('camera')
        evidence('after-camera')
        assert (out/'before-camera.txt').read_text().split('state_hash=')[1].splitlines()[0]==(out/'after-camera.txt').read_text().split('state_hash=')[1].splitlines()[0]
        (out/'input.json').write_text(json.dumps({'mode':mode,'display':os.environ.get('DISPLAY'),'events':events,'result':'PASS'},indent=2)+'\n')
    finally:
        p.terminate();p.wait(timeout=10)
print(f'{mode}: actual window inputs and headless replay PASS')
