import test from 'node:test';
import assert from 'node:assert/strict';
import {buildInputs,chooseSource,orderSummary,selectedPeople} from '../web/controls.mjs';
const point=(x,y=28000)=>({x,y});
function view(){return {side:0,tick:0,config:{exchange_radius:3000,speeds:[45,50,65,60,80],road_speed_percent:125},terrain:{headquarters:[point(8000),point(82000)],cells:Array(5400).fill(0)},posts:[],people:[
{id:0,side:0,role:'general',hp:100,pos:point(8000),observed:0},
{id:2,side:0,role:'line',hp:100,pos:point(10000),observed:0},
{id:32,side:0,role:'carrier',hp:100,pos:point(9000),observed:0},
{id:36,side:0,role:'courier',hp:100,pos:point(8000),observed:0},
{id:40,side:0,role:'scout',hp:100,pos:point(30000),observed:0},
{id:50,side:1,role:'line',hp:100,pos:point(70000),observed:0}]};}
const preset={verb:'move',manner:'together',after:'hold'},source={kind:'hq'};
test('ordinary MOVE becomes direct general movement, never a courier order',()=>{
 assert.deepEqual(buildInputs(view(),[0],preset,point(25000),source),[{type:'move_general',id:0,target:point(25000)}]);
 assert.throws(()=>buildInputs(view(),[0],{...preset,verb:'engage'},point(25000),source),/Generals/);
 assert.throws(()=>buildInputs(view(),[0,2],preset,point(25000),source),/separately/);
});
test('support MOVE changes the actual duty destination and preserves its role',()=>{
 for(const [id,role] of [[32,'carrier'],[36,'courier'],[40,'scout']]){
  const [input]=buildInputs(view(),[id],preset,point(25000),source);
  assert.equal(input.type,'assign');assert.equal(input.role,role);assert.deepEqual(input.target,point(25000));
 }
 assert.throws(()=>buildInputs(view(),[32],{...preset,verb:'engage'},point(25000),source),/Standing duties/);
});
test('troop plans preserve routes, fallbacks, rendezvous and seat ownership',()=>{
 const inputs=buildInputs(view(),[2,50],{...preset,after:'fallback'},point(30000),source,{route:[point(22000)],fallback:point(12000),rendezvous:point(10000)});
 assert.deepEqual(inputs[0].recipients,[2]);assert.deepEqual(inputs[0].plan.fallback,point(12000));assert.equal(inputs[0].plan.then,null);assert.equal(inputs[0].plan.route.length,1);assert.deepEqual(inputs[0].rendezvous,point(10000));
 const v=view();v.people[1].hp=0;assert.deepEqual(selectedPeople(v,[2,50]),[]);
});
test('water destinations are rejected before consuming a courier',()=>{
 const v=view();v.terrain.cells[28*90+44]=3;
 assert.throws(()=>buildInputs(v,[2],preset,point(44000),source),/deep water/);
});
test('auto source chooses nearby command using only received friendly observations',()=>{
 const v=view();v.people[0].pos=point(30000);v.people[1].pos=point(31000);
 assert.deepEqual(chooseSource(v,[2]),{kind:'general',id:0});
 v.people[1].pos=point(50000);assert.deepEqual(chooseSource(v,[2]),{kind:'hq'});
 v.people[5].role='courier';v.people[5].pos=point(30000);assert.deepEqual(chooseSource(v,[2]),{kind:'hq'});
});
test('delivery feedback distinguishes confirmed partial delivery from pending or failed',()=>{
 const order={recipients:[2,3,4],status:'Acknowledged locally by 1 people; execution is not confirmed complete'};
 assert.match(orderSummary(order).text,/1\/3/);
 assert.equal(orderSummary({...order,status:'Dispatched; awaiting acknowledgement'}).kind,'pending');
 assert.equal(orderSummary({...order,status:'Returned undelivered: recipient not found at rendezvous'}).kind,'failed');
});
