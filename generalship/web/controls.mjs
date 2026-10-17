// Decisions here use only the authorized player view. They never simulate movement.
export const troopRoles = ['line','ranged','light','guard'];
export const groupNames = ['Line infantry','Ranged infantry','Light infantry','Couriers','Supply carriers'];
export function selectedPeople(view, ids) {
  return view.people.filter(p => ids.includes(p.id) && p.side === view.side && p.hp > 0);
}
export function centre(people) {
  if (!people.length) return null;
  return {x:Math.round(people.reduce((s,p)=>s+p.pos.x,0)/people.length),y:Math.round(people.reduce((s,p)=>s+p.pos.y,0)/people.length)};
}
export function distance(a,b) { return Math.hypot(a.x-b.x,a.y-b.y); }
export function sourcePoint(view, source) {
  if(source.kind==='hq') return view.terrain.headquarters[view.side];
  return (source.kind==='general'?view.people:view.posts).find(p=>p.id===source.id)?.pos;
}
function localReach(view, from, to) {
  if(distance(from,to)>view.config.exchange_radius) return false;
  if(distance(from,to)<=(view.config.short_sight??1800)) return true;
  const steps=Math.max(1,Math.floor(Math.max(Math.abs(to.x-from.x),Math.abs(to.y-from.y))/400));
  for(let i=1;i<=steps;i++) {
    const p={x:from.x+(to.x-from.x)*i/steps,y:from.y+(to.y-from.y)*i/steps};
    if(view.terrain.cells[Math.floor(p.y/1000)*90+Math.floor(p.x/1000)]===2 && distance(from,p)>(view.config.woods_visibility??1200)) return false;
  }
  return true;
}
export function chooseSource(view, ids) {
  const target=centre(selectedPeople(view,ids));
  if(!target) return {kind:'hq'};
  const sources=[{kind:'hq'},...view.people.filter(p=>p.side===view.side&&p.role==='general'&&p.hp>0).map(p=>({kind:'general',id:p.id})),...view.posts.filter(p=>p.side===view.side&&p.hp>0&&p.complete&&view.tick-p.observed<30).map(p=>({kind:'outpost',id:p.id}))];
  const local=sources.filter(s=>selectedPeople(view,ids).some(p=>localReach(view,sourcePoint(view,s),p.pos)));
  if(local.length) return local.sort((a,b)=>distance(sourcePoint(view,a),target)-distance(sourcePoint(view,b),target))[0];
  // A remote source needs a recently observed courier nearby; availability is not guaranteed.
  const supported=sources.filter(s=>view.people.some(p=>p.side===view.side&&p.role==='courier'&&p.hp>0&&view.tick-p.observed<30&&distance(p.pos,sourcePoint(view,s))<=view.config.exchange_radius));
  return supported.sort((a,b)=>distance(sourcePoint(view,a),target)-distance(sourcePoint(view,b),target))[0]||{kind:'hq'};
}
export function advice(view, ids, source) {
  const people=selectedPeople(view,ids), point=sourcePoint(view,source), target=centre(people);
  if(!target||!point) return 'Select a formation or general to give an order.';
  if(people.every(p=>p.role==='general')) return 'Direct command · MOVE or right-click moves the selected general.';
  const age=Math.floor(Math.max(...people.map(p=>view.tick-p.observed))/10);
  const name=source.kind==='hq'?'HQ':`${source.kind==='general'?'General':'Outpost'} ${source.id}`;
  if(people.some(p=>localReach(view,point,p.pos))) return `From ${name} · local handoff${people.every(p=>troopRoles.includes(p.role))?' → nearby formation members':''} · ${age?`positions ${age}s old`:'positions current'}`;
  const seconds=Math.ceil(distance(point,target)/(view.config.speeds[4]*view.config.road_speed_percent/100)/10);
  const knownCourier=view.people.some(p=>p.side===view.side&&p.role==='courier'&&p.hp>0&&view.tick-p.observed<30&&distance(p.pos,point)<=view.config.exchange_radius);
  return `From ${name} · courier needed · at least ~${seconds}s travel${knownCourier?'':' · no courier recently seen here'}${age?` · recipient report ${age}s old`:''}`;
}
export function buildInputs(view, ids, preset, target, source, options={}) {
  const people=selectedPeople(view,ids);
  if(!people.length) throw Error('Select living friendly people first.');
  if(target.x<0||target.x>=90000||target.y<0||target.y>=60000||view.terrain.cells[Math.floor(target.y/1000)*90+Math.floor(target.x/1000)]===3) throw Error('That is deep water. Choose the bridge, ford, or dry ground.');
  if(people.some(p=>p.role==='general')) {
    if(!people.every(p=>p.role==='general')) throw Error('Select generals separately from troops. Double-click a troop to select its formation.');
    if(!['move','withdraw','hold'].includes(preset.verb)) throw Error('Generals provide command and vision. Use MOVE, HOLD, or WITHDRAW.');
    if(options.route?.length) throw Error('Generals take a direct destination. Clear route waypoints with Escape.');
    return people.map(p=>({type:'move_general',id:p.id,target:preset.verb==='hold'?p.pos:target}));
  }
  if(people.some(p=>!troopRoles.includes(p.role))) {
    if(!people.every(p=>['carrier','courier','scout'].includes(p.role))||!['move','observe'].includes(preset.verb)||preset.verb==='observe'&&people.some(p=>p.role!=='scout')) throw Error('These people have support duties. Use the Standing duties controls, or MOVE for scouts, couriers, and carriers.');
    if(options.route?.length) throw Error('Support duties use one rendezvous. Clear route waypoints with Escape.');
    return [...new Set(people.map(p=>p.role))].map(role=>({type:'assign',source,people:people.filter(p=>p.role===role).map(p=>p.id),role,target,post:null,escort:null}));
  }
  return [{type:'order',source,recipients:people.map(p=>p.id),rendezvous:options.rendezvous??null,plan:{verb:preset.verb,manner:preset.manner,target,area_radius:options.area||0,route:options.route||[],then:['hold','observe'].includes(preset.after)?preset.after:null,fallback:preset.after==='fallback'?(options.fallback||view.terrain.headquarters[view.side]):null}}];
}
export function orderSummary(order) {
  const count=order.recipients.length, status=order.status;
  const acknowledged=Number(status.match(/Acknowledged locally by (\d+)/)?.[1]||0);
  const received=Math.max(acknowledged,order.confirmed?.length||0);
  if(received) return {kind:received<count?'partial':'delivered',text:`Received by ${received}/${count} · ${received<count?'others not yet confirmed':'following their local situation'}`};
  if(status.includes('undelivered')) return {kind:'failed',text:'Not delivered · courier could not find the recipients'};
  if(status.includes('Acknowledged locally by 0')) return {kind:'failed',text:'No new recipients · a newer order or duty may already apply'};
  return {kind:'pending',text:'Awaiting a report · distant orders travel by courier'};
}
