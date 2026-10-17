// Real HTTPS/WSS acceptance check against the supplied G0 service; Node.js 22+.
import {readFileSync,writeFileSync} from 'node:fs';
import assert from 'node:assert/strict';
const url=process.argv[2];
if(!url?.startsWith('https://'))throw new Error('Supply the public HTTPS service URL');
const hostCode=process.env.G0_HOST_CODE||readFileSync(new URL('./runtime/host-code.txt',import.meta.url),'utf8').trim();
async function post(path,body){const r=await fetch(url+path,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});const text=await r.text();let value;try{value=JSON.parse(text);}catch{value=text;}return {status:r.status,value};}
let maxBytes=0;
function connect(auth){
  const socket=new WebSocket(url.replace('https:','wss:')+'/ws');const inbox=[];let error;
  socket.addEventListener('open',()=>socket.send(JSON.stringify(auth)));
  socket.addEventListener('message',e=>{maxBytes=Math.max(maxBytes,e.data.length);inbox.push(JSON.parse(e.data));});
  socket.addEventListener('error',()=>{error=new Error('WebSocket transport error');});
  return {socket,send:value=>socket.send(JSON.stringify(value)),wait:async predicate=>{
    const deadline=Date.now()+15000;
    while(Date.now()<deadline){if(error)throw error;const index=inbox.findIndex(predicate);if(index>=0)return inbox.splice(index,1)[0];await new Promise(r=>setTimeout(r,20));}
    throw new Error('Timed out waiting for expected protocol response');
  }};
}
let west,east;
try{
  assert.equal((await fetch(url+'/health')).status,200);
  assert.equal((await fetch(url+'/.git/config')).status,404);
  assert.equal((await post('/api/create',{host_code:'invalid'})).status,401);
  const a=await post('/api/create',{host_code:hostCode,seed:42});assert.equal(a.status,200);
  const b=await post('/api/join',{invite:a.value.invite});assert.equal(b.status,200);
  assert.equal((await post('/api/join',{invite:a.value.invite})).status,401);
  west=connect(a.value);east=connect(b.value);
  for(const [seat,side] of [[west,0],[east,1]]){
    const view=(await seat.wait(m=>m.type==='view')).view;
    assert.equal(view.side,side);assert(view.people.every(p=>p.side===side));
    seat.send({type:'input',sequence:1,generation:0,input:{type:'ready'}});
    await seat.wait(m=>m.type==='receipt'&&m.sequence===1);
  }
  await west.wait(m=>m.type==='view'&&m.view.status.state==='running');
  const start=performance.now();
  west.send({type:'input',sequence:2,generation:0,input:{type:'move_general',id:0,target:{x:20000,y:28000}}});
  await west.wait(m=>m.type==='receipt'&&m.sequence===2);
  const rtt=performance.now()-start;
  await west.wait(m=>m.type==='view'&&m.view.tick>=10&&m.view.people.find(p=>p.id===0).pos.x>7000);
  west.socket.close();await new Promise(r=>setTimeout(r,800));
  west=connect(a.value);const restored=await west.wait(m=>m.type==='view');assert.equal(restored.view.side,0);assert.equal(restored.last_sequence,2);
  west.send({type:'input',sequence:3,generation:0,input:{type:'surrender'}});
  await east.wait(m=>m.type==='view'&&m.view.status.state==='finished'&&m.view.status.winner===1);
  west.send({type:'replay'});const replay=(await west.wait(m=>m.type==='replay')).replay;
  writeFileSync(new URL('./runtime/internet-replay.json',import.meta.url),JSON.stringify(replay));
  west.send({type:'rematch'});east.send({type:'rematch'});
  await west.wait(m=>m.type==='view'&&m.generation===1&&m.view.status.state==='lobby');
  console.log(JSON.stringify({result:'PASS',https:true,wss:true,seat_isolation:true,one_use_invite:true,reconnect:true,surrender:true,rematch:true,round_trip_ms:Math.round(rtt),max_snapshot_bytes:maxBytes,replay_tick:replay.end_tick}));
}finally{west?.socket.close();east?.socket.close();}
