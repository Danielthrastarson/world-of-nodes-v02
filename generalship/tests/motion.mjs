import test from 'node:test';
import assert from 'node:assert/strict';
import {SnapshotMotion} from '../web/motion.mjs';

function snapshot(tick, changes={}) {
    return {tick, people:[{id:1, pos:{x:tick*100,y:0}, hp:100,
        observed:tick, received:tick, source:'general:0', absent_since:null, ...changes}]};
}
const x = (motion, view, now) => motion.sample(view, now).get(1).x;

test('jitter and duplicate snapshots do not reverse or restart motion', () => {
    const motion=new SnapshotMotion();
    const packets=Array.from({length:31},(_,tick)=>({tick,at:tick*100+[0,60,5,30,10][tick%5]}));
    let current=snapshot(0), next=0, previous=0;
    for(let now=0;now<=3000;now+=10){
        while(next<packets.length&&packets[next].at<=now){current=snapshot(packets[next++].tick);motion.push(current,now);}
        if(now%70===0)motion.push(current,now);
        const value=x(motion,current,now);
        assert.ok(value>=previous,`reversed at ${now}: ${previous} -> ${value}`);
        assert.ok(value<=current.people[0].pos.x,'never predict beyond received position');
        if(now>400)assert.ok(value-previous>=9&&value-previous<=11,`uneven frame at ${now}: ${value-previous}`);
        previous=value;
    }
});

test('stalled network holds the final authorized position', () => {
    const motion=new SnapshotMotion();let view;
    for(let tick=0;tick<=5;tick++){view=snapshot(tick);motion.push(view,tick*100);x(motion,view,tick*100);}
    assert.equal(x(motion,view,2000),500);
    assert.equal(x(motion,view,5000),500);
});

test('continuously delayed relay observations interpolate without removing signal delay', () => {
    const motion=new SnapshotMotion();let view;
    for(let tick=10;tick<=16;tick++){
        view=snapshot(tick,{source:'outpost:0',observed:tick-10,pos:{x:(tick-10)*100,y:0}});
        motion.push(view,tick*100);x(motion,view,tick*100);
    }
    assert.equal(x(motion,view,1650),450);
    assert.equal(view.people[0].observed,6);
});

test('dated courier reports, absent contacts and casualties do not acquire invented motion', () => {
    for(const changes of [
        {source:'courier:36',observed:0},
        {received:0,observed:0},
        {absent_since:2},
        {hp:0},
    ]){
        const motion=new SnapshotMotion();let view;
        for(let tick=0;tick<=5;tick++){view=snapshot(tick,changes);motion.push(view,tick*100);}
        assert.deepEqual(motion.sample(view,550).get(1),view.people[0].pos);
    }
});

test('changed relay delay or a gap in observations snaps to the new report', () => {
    for(const changes of [{observed:1},{received:1}]){
        const motion=new SnapshotMotion();
        for(let tick=0;tick<=3;tick++){const view=snapshot(tick);motion.push(view,tick*100);x(motion,view,tick*100);}
        const view=snapshot(4,changes);motion.push(view,400);
        assert.deepEqual(motion.sample(view,450).get(1),view.people[0].pos);
    }
});

test('new contacts appear only after receipt and reset clears previous round positions', () => {
    const motion=new SnapshotMotion();
    motion.push(snapshot(50),0);
    const current={tick:51,people:[]};motion.push(current,100);
    assert.equal(motion.sample(current,150).has(1),false);
    motion.reset();const next=snapshot(0,{pos:{x:42,y:0}});motion.push(next,200);
    assert.equal(x(motion,next,200),42);
});
