import {FieldRenderer,mapTransform} from './field.mjs';
import {SnapshotMotion} from './motion.mjs';
import {selectedPeople,distance,chooseSource,advice,buildInputs,orderSummary,groupNames} from './controls.mjs';
// The canvas consumes authorized snapshots only. No world simulation runs here.
const $=id=>document.getElementById(id), canvas=$('field'),renderer=new FieldRenderer(canvas);
let view=null, previous=null,ws=null,session=null,seq=0,generation=0,selected=new Set(),postId=null,siteId=null,pending=null,drag=null,waypoints=[],fallback=null,rendezvous=null,reconnectTimer=null,reconnectDelay=1800,terminal=false;
const defaults=[{verb:'move',manner:'together',after:'hold',key:'1'},{verb:'hold',manner:'no_pursuit',after:'none',key:'2'},{verb:'engage',manner:'together',after:'none',key:'3'},{verb:'observe',manner:'avoid',after:'none',key:'4'},{verb:'withdraw',manner:'avoid',after:'none',key:'5'}];
defaults.forEach((p,i)=>p.groupKey=String(i+1));
let fieldAlert='',fieldAlertUntil=0,feedbackSelection=new Set();
let connectionReady=false,botMatch=false,readyPending=false;
let zoom=1,pan={x:0,y:0},hover=null,markers=[],groupHits=[],sent=new Map(),feedback='Select a formation, choose a command, then click the map.',damageFlashes=new Map();
const motion=new SnapshotMotion();
let displayPeople=[];
let presets=defaults,groups=Array.from({length:5},()=>[]);
try{const saved=JSON.parse(localStorage.getItem('g0-presets'));if(Array.isArray(saved)&&saved.length===5)presets=saved.map((p,i)=>({...p,groupKey:p.groupKey||String(i+1)}));}catch{}
function toast(text){$('toast').textContent=text;clearTimeout(toast.timer);toast.timer=setTimeout(()=>{$('toast').textContent='';},6500);}
function textNode(tag,text){const e=document.createElement(tag);e.textContent=text;return e;}
function time(tick){const s=Math.floor(tick/10);return `${Math.floor(s/60).toString().padStart(2,'0')}:${(s%60).toString().padStart(2,'0')}`;}
function source(){const value=$('source').value||'auto';if(value==='auto')return chooseSource(view,[...selected]);if(value==='hq')return {kind:'hq'};const[k,id]=value.split(':');return {kind:k,id:Number(id)};}
function canCommand(){return connectionReady&&view?.status.state==='running';}
function renderAvailability(){
    const lobby=view?.status.state==='lobby',blocked=!connectionReady||lobby;
    const wasBlocked=!$('startGate').hidden,hadFocus=$('startGate').contains(document.activeElement);
    $('startGate').hidden=!blocked;$('game').dataset.awaiting=String(blocked);
    for(const id of ['field','forceBar','mapTools'])$(id).inert=blocked;
    $('deskButton').disabled=blocked;$('addBot').disabled=!connectionReady;$('rematch').disabled=!connectionReady;
    if(blocked){setDesk();pending=null;drag=null;waypoints=[];updateHint();}
    $('game').dataset.connection=connectionReady?'connected':'reconnecting';$('startMode').textContent=connectionReady?(botMatch?'BOT PRACTICE':'PRIVATE MATCH'):'COMMAND CONNECTION';
    $('startTitle').textContent=!connectionReady?(view?'Reconnecting…':'Connecting to the field…'):view.ready[view.side]?'Waiting for your opponent':botMatch?'Ready to take command?':'Ready to deploy?';
    $('startMessage').textContent=!connectionReady?(view?.status.state==='running'?'Standing orders continue. Controls return when your connection is restored.':'Waiting for your command connection.'):view.ready[view.side]?'You are ready. The clock starts when your opponent is ready too.':botMatch?'The field is paused. Start practice to begin moving and issuing orders.':'The field is paused until both commanders are ready.';
    $('ready').hidden=!connectionReady||!lobby;$('ready').disabled=!connectionReady||readyPending||!!view?.ready[view.side];
    $('ready').textContent=readyPending?'Starting…':view?.ready[view.side]?'Ready · waiting':botMatch?'Start practice':'I’m ready';
    $('startGuide').hidden=!connectionReady||!lobby;
    if(blocked&&!wasBlocked&&connectionReady&&!$('ready').disabled)$('ready').focus();
    else if(!blocked&&wasBlocked&&hadFocus)canvas.focus();
    updateHint();
}
function send(input,label='Instruction',target=null){
    if(!connectionReady||!ws||ws.readyState!==WebSocket.OPEN){toast('Connection unavailable; no order was sent.');return false;}
    if(view?.status.state!=='running'&&input.type!=='ready'){toast('Start the match before issuing orders.');return false;}
    const sequence=++seq;sent.set(sequence,{label,target});feedbackSelection=new Set(input.recipients||input.people||(input.id!==undefined?[input.id]:[]));
    ws.send(JSON.stringify({type:'input',sequence,generation,input}));
    if(target){markers.push({target,label,until:performance.now()+2200});}
    feedback=`${label} · sending…`;return true;
}
function dispatch(preset,target,options={}){
    if(!canCommand()){toast(connectionReady?'The round is not running.':'Connection unavailable; no order was sent.');return false;}
    try{
        const inputs=buildInputs(view,[...selected],preset,target,source(),options);
        for(const input of inputs)if(!send(input,input.type==='move_general'?`General ${input.id} moving`:input.type==='assign'?`${input.role} rendezvous`:`${preset.verb.toUpperCase()} · ${input.recipients.length} people`,target))return false;
        return true;
    }catch(error){toast(error.message);feedback=error.message;return false;}
}
let deskSection='orders';
function setDesk(section=null){
    $('commandDesk').hidden=!section;$('deskButton').setAttribute('aria-expanded',String(!!section));
    if(!section)return;deskSection=section;
    document.querySelectorAll('[data-desk]').forEach(b=>b.setAttribute('aria-pressed',String(b.dataset.desk===section)));
    document.querySelectorAll('[data-desk-panel]').forEach(p=>p.hidden=p.dataset.deskPanel!==section);
}
function cancelTarget(){pending=null;waypoints=[];updateHint();}
function clearSelection(){cancelTarget();selected.clear();renderSelection();}
function commandIssue(verb){
    if(!canCommand())return 'Commands become available when the round is running.';
    const people=view?selectedPeople(view,[...selected]):[];
    if(!people.length)return 'Select people first.';
    if(people.some(p=>p.role==='general'))return !people.every(p=>p.role==='general')?'Select generals separately from troops.':['move','hold','withdraw'].includes(verb)?'':'Generals use Move, Hold, or Withdraw.';
    if(people.every(p=>['line','ranged','light','guard'].includes(p.role)))return '';
    if(people.every(p=>['carrier','courier','scout'].includes(p.role))&&(verb==='move'||verb==='observe'&&people.every(p=>p.role==='scout')))return '';
    return 'Use Details → Standing duties for these people.';
}
function command(index){if(!view)return;if(!canCommand()){toast(connectionReady?'Start the round first.':'Reconnecting · no order was sent.');return;}const issue=commandIssue(presets[index].verb);if(issue){toast(issue);return;}pending={type:'order',preset:structuredClone(presets[index]),index};waypoints=[];updateHint();canvas.focus();}
function updateHint(){
    document.querySelectorAll('#commands button').forEach((e,i)=>{e.classList.toggle('active',pending?.index===i);e.setAttribute('aria-pressed',String(pending?.index===i));});
    $('targetBar').hidden=!canCommand()||!pending;$('commandDock').hidden=!canCommand()||!!pending||!selected.size;canvas.classList.toggle('targeting',!!pending);
    if(pending){setDesk();$('mapLegend').hidden=true;$('legendButton').setAttribute('aria-expanded','false');}
    $('hint').textContent=pending?pending.type==='rendezvous'?'Choose a courier rendezvous':pending.type==='fallback'?'Choose a fallback position':pending.type==='general'?'Move general · choose destination':pending.type==='assign'?'Assign duty · choose rendezvous':`${pending.preset.verb.toUpperCase()} · choose target${pending.preset.verb==='move'?' · Shift-click for route':''}`:'';
}
$('deskButton').onclick=()=>setDesk($('commandDesk').hidden?deskSection:null);
$('closeDesk').onclick=()=>{setDesk();$('deskButton').focus();};
document.querySelectorAll('[data-desk]').forEach(b=>b.onclick=()=>setDesk(b.dataset.desk));
$('selectionDetails').onclick=()=>setDesk('people');$('orderDetails').onclick=()=>setDesk('orders');$('intelButton').onclick=()=>setDesk('reports');
$('clearSelection').onclick=clearSelection;$('cancelCommand').onclick=()=>{cancelTarget();canvas.focus();};
async function api(path,body){const r=await fetch(path,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});const text=await r.text();let data;try{data=JSON.parse(text);}catch{throw new Error(text);}if(!r.ok)throw new Error(data.error||text);return data;}
function enter(data){session={room:data.room,token:data.token,side:data.side,boot:data.boot};sessionStorage.setItem('g0-session',JSON.stringify(session));const base=data.side*48;groups=[[...Array(10)].map((_,i)=>base+2+i),[...Array(10)].map((_,i)=>base+12+i),[...Array(10)].map((_,i)=>base+22+i),[...Array(4)].map((_,i)=>base+36+i),[...Array(4)].map((_,i)=>base+32+i)];groupButtons();$('lobby').hidden=true;$('game').hidden=false;if(data.invite){$('inviteArea').hidden=false;$('invite').value=`${location.origin}/#invite=${data.invite}`;}connect();}
function connect(){terminal=false;connectionReady=false;readyPending=false;renderAvailability();motion.reset();ws=new WebSocket(`${location.protocol==='https:'?'wss:':'ws:'}//${location.host}/ws`);ws.onopen=()=>ws.send(JSON.stringify(session));ws.onmessage=event=>{let m;try{m=JSON.parse(event.data);}catch{return;}
    if(m.type==='view'){reconnectDelay=1800;previous=view;view=m.view;connectionReady=true;if(view.ready[view.side]||view.status.state!=='lobby')readyPending=false;for(const id of selected){if(view.people.some(p=>p.id===id&&p.hp<=0))selected.delete(id);}for(const p of view.people){const old=previous?.people.find(q=>q.id===p.id);if(old&&p.observed>old.observed&&p.hp<old.hp){damageFlashes.set(p.id,performance.now()+700);if(p.side===view.side&&p.role==='general'&&p.hp<=0&&old.hp>0){fieldAlert=`General ${p.id} reported lost · train a replacement at HQ.`;fieldAlertUntil=performance.now()+20000;}}}seq=Math.max(seq,m.last_sequence);if(generation!==m.generation){selected.clear();postId=null;siteId=null;pending=null;waypoints=[];markers=[];sent.clear();fieldAlert='';fieldAlertUntil=0;feedback='Fresh round · choose a formation and start when ready.';previous=null;renderer.reset();updateHint();}if(!previous||previous.status.state!==view.status.state)motion.reset();motion.push(view,performance.now());generation=m.generation;session.boot=m.boot;sessionStorage.setItem('g0-session',JSON.stringify(session));renderPanels(m.rematch,m.opponent==='bot',m.can_add_bot);}
    else if(m.type==='error'){readyPending=false;renderAvailability();const pendingSend=sent.get(m.sequence);if(pendingSend)sent.delete(m.sequence);feedback=m.message;toast(m.message);if(m.message.includes('credential')){terminal=true;}}
    else if(m.type==='gone'){terminal=true;sessionStorage.removeItem('g0-session');$('matchStatus').textContent=m.reason;$('lobbyMessage').textContent=m.reason;$('game').hidden=true;$('lobby').hidden=false;view=null;toast(m.reason);}
    else if(m.type==='receipt'){const pendingSend=sent.get(m.sequence);if(pendingSend){sent.delete(m.sequence);feedback=pendingSend.label==='Instruction'?'Ready to command · select a formation, then right-click a destination.':`${pendingSend.label} · ${pendingSend.label.startsWith('General')?'direct command accepted':'accepted; awaiting a delivery report'}`;}else if(m.message!=='Duplicate ignored')feedback=m.message;}
    else if(m.type==='replay'){const blob=new Blob([JSON.stringify(m.replay,null,2)],{type:'application/json'});const link=document.createElement('a');link.href=URL.createObjectURL(blob);link.download=`generalship-${session.room.slice(0,8)}-${generation}.json`;link.click();setTimeout(()=>URL.revokeObjectURL(link.href),1000);}
};ws.onclose=()=>{connectionReady=false;renderAvailability();if(!terminal){$('matchStatus').textContent='Connection lost · standing orders continue';clearTimeout(reconnectTimer);reconnectTimer=setTimeout(connect,reconnectDelay);reconnectDelay=Math.min(15000,reconnectDelay*2);}};ws.onerror=()=>{};}
setInterval(()=>{if(ws?.readyState===WebSocket.OPEN)ws.send(JSON.stringify({type:'ping'}));},5000);
$('createForm').onsubmit=async e=>{e.preventDefault();try{const data=await api('/api/create',{host_code:$('hostCode').value,seed:Number($('seed').value),bot:e.submitter?.value==='bot'});$('hostCode').value='';enter(data);}catch(e){$('lobbyMessage').textContent=e.message;}};
$('addBot').onclick=()=>ws?.send(JSON.stringify({type:'add_bot'}));
$('copyInvite').onclick=async()=>{try{await navigator.clipboard.writeText($('invite').value);toast('Invitation copied.');}catch{$('invite').select();toast('Copy the selected invitation.');}};
$('ready').onclick=()=>{if(send({type:'ready'},'Ready')){readyPending=true;renderAvailability();}};$('surrender').onclick=()=>{if($('surrender').dataset.confirm==='yes'){send({type:'surrender'});$('surrender').dataset.confirm='';$('surrender').textContent='Surrender';}else{$('surrender').dataset.confirm='yes';$('surrender').textContent='Confirm surrender';setTimeout(()=>{$('surrender').dataset.confirm='';$('surrender').textContent='Surrender';},6000);}};
$('rematch').onclick=()=>ws?.send(JSON.stringify({type:'rematch'}));$('replay').onclick=()=>ws?.send(JSON.stringify({type:'replay'}));
$('moveGeneral').onclick=()=>{if(!view?.people.some(p=>selected.has(p.id)&&p.side===view.side&&p.role==='general'&&p.hp>0)){toast('Select a living general.');return;}pending={type:'general'};updateHint();};
$('setRendezvous').onclick=()=>{pending={type:'rendezvous'};updateHint();};
$('assign').onclick=()=>{if(!selected.size){toast('Select people to reassign.');return;}pending={type:'assign'};updateHint();};
$('build').onclick=()=>{if(siteId===null){toast('Select an empty hill site on the map.');return;}send({type:'build',site:siteId,people:[...selected]});};
$('upgrade').onclick=()=>{if(postId===null){toast('Select a known friendly outpost.');return;}send({type:'upgrade',post:postId,people:[...selected]});};
$('train').onclick=()=>{if(selected.size!==1){toast('Select one available headquarters resident.');return;}send({type:'train',person:[...selected][0]});};
$('helpButton').onclick=()=>$('help').showModal();$('closeHelp').onclick=()=>$('help').close();
const commandPaths={move:'M4 12h15m-6-6 6 6-6 6',hold:'M6 4v16M18 4v16M6 12h12',observe:'M2 12s3-6 10-6 10 6 10 6-3 6-10 6S2 12 2 12 M9 12a3 3 0 1 0 6 0a3 3 0 1 0-6 0',engage:'m5 4 15 16M19 4 4 19M3 15l6 6M15 3l6 6',withdraw:'M20 7H10v-4L3 10l7 7v-4h10v7'};
const commandDescriptions={move:'Travel to a destination. Shift-click to add route points.',hold:'Defend a position without pursuit.',observe:'Watch a position and avoid closing with enemies.',engage:'Approach and fight locally observed enemies.',withdraw:'Disengage toward a chosen fallback.'};
function commandButtons(){const area=$('commands');area.replaceChildren();presets.forEach((p,i)=>{
    const b=document.createElement('button'),svg=document.createElementNS('http://www.w3.org/2000/svg','svg'),path=document.createElementNS('http://www.w3.org/2000/svg','path');
    svg.setAttribute('viewBox','0 0 24 24');svg.setAttribute('aria-hidden','true');path.setAttribute('d',commandPaths[p.verb]||commandPaths.move);svg.append(path);
    b.append(svg,textNode('span',p.verb[0].toUpperCase()+p.verb.slice(1)),textNode('kbd',p.key));b.setAttribute('aria-label',`${p.key} ${p.verb.toUpperCase()}`);b.setAttribute('aria-keyshortcuts',p.key);b.setAttribute('aria-pressed','false');
    b.dataset.description=`${commandDescriptions[p.verb]} ${p.manner.replaceAll('_',' ')}${p.after!=='none'?' → '+p.after:''}.`;b.title=b.dataset.description;b.onclick=()=>command(i);area.append(b);
});}
function selectGroup(index){if(!canCommand())return;cancelTarget();selected=new Set(groups[index].filter(id=>view?.people.some(p=>p.id===id&&p.side===view.side&&p.hp>0)));renderSelection();}
function groupButtons(){const area=$('groups');area.replaceChildren();groups.forEach((g,i)=>{const b=document.createElement('button');b.dataset.group=i;b.append(textNode('span',['Line','Ranged','Light','Couriers','Supply'][i]),textNode('span',''),textNode('kbd',''));b.children[1].className='unitCount';b.onclick=e=>{if(e.ctrlKey){groups[i]=[...selected];groupButtons();toast(`${groupNames[i]} slot assigned.`);}else selectGroup(i);};area.append(b);});refreshGroups();}
function refreshGroups(){for(const b of $('groups').children){const i=Number(b.dataset.group),people=view?selectedPeople(view,groups[i]):[],count=view?people.length:groups[i].length;const name=`${groupNames[i]} · ${count} · Alt+${presets[i].groupKey}`;b.style.setProperty('--strength',`${Math.min(100,count/Math.max(1,groups[i].length)*100)}%`);b.children[1].textContent=count;b.children[2].textContent=`⌥${presets[i].groupKey}`;b.title=`${name}. Ctrl-click assigns your selection.`;b.setAttribute('aria-label',name);const active=people.length>0&&people.every(p=>selected.has(p.id))&&selected.size===people.length;b.classList.toggle('active',active);b.setAttribute('aria-pressed',String(active));}}
function openEditor(){if($('help').open)$('help').close();const area=$('presetEditor');area.replaceChildren();presets.forEach((p,i)=>{const row=document.createElement('div');row.append(textNode('b',String(i+1)));for(const [field,values]of [['verb',['move','hold','observe','engage','withdraw']],['manner',['together','no_pursuit','avoid']],['after',['none','hold','observe','fallback']]]){const s=document.createElement('select');s.dataset.field=field;s.setAttribute('aria-label',`${field} slot ${i+1}`);for(const v of values){const o=textNode('option',v.replaceAll('_',' '));o.value=v;s.append(o);}s.value=p[field];row.append(s);}const key=document.createElement('input');key.maxLength=1;key.value=p.key;key.dataset.field='key';key.title='Command key';key.setAttribute('aria-label',`Command key ${i+1}`);row.append(key);const groupKey=document.createElement('input');groupKey.maxLength=1;groupKey.value=p.groupKey;groupKey.dataset.field='groupKey';groupKey.title='Group key (with Alt; Ctrl+Alt assigns)';groupKey.setAttribute('aria-label',`Group key ${i+1}`);row.append(groupKey);area.append(row);});$('editor').showModal();}
$('editPresets').onclick=openEditor;$('editPresetsDesk').onclick=openEditor;$('closeEditor').onclick=()=>$('editor').close();
$('setFallback').onclick=()=>{$('editor').close();pending={type:'fallback'};updateHint();};
$('savePresets').onclick=()=>{const next=[...$('presetEditor').children].map(row=>Object.fromEntries([...row.querySelectorAll('[data-field]')].map(e=>[e.dataset.field,e.value])));if(new Set(next.map(p=>p.key)).size!==5||new Set(next.map(p=>p.groupKey)).size!==5||next.some(p=>!p.groupKey)||next.some(p=>!p.key||p.key==='Escape'||((p.after==='hold'||p.after==='observe')&&p.verb!=='move')||(p.manner==='avoid'&&['hold','engage'].includes(p.verb)))){toast('Use unique keys. Follow-ups require MOVE; HOLD/ENGAGE cannot avoid contact.');return;}presets=next;localStorage.setItem('g0-presets',JSON.stringify(presets));commandButtons();groupButtons();$('editor').close();};
window.addEventListener('keydown',e=>{if($('editor').open||$('help').open)return;
    if(e.key==='Escape'){e.preventDefault();if(pending)cancelTarget();else if(!$('commandDesk').hidden){setDesk();$('deskButton').focus();}else clearSelection();return;}
    if(!canCommand()||['INPUT','SELECT','TEXTAREA'].includes(document.activeElement.tagName))return;
    const digit=presets.findIndex(p=>p.groupKey.toLowerCase()===e.key.toLowerCase());if(e.altKey&&digit>=0&&digit<5){e.preventDefault();if(e.ctrlKey){groups[digit]=[...selected];groupButtons();toast(`Group ${digit+1} assigned.`);}else{selectGroup(digit);}return;}
    const i=presets.findIndex(p=>p.key.toLowerCase()===e.key.toLowerCase());if(i>=0&&!e.ctrlKey&&!e.metaKey&&!e.altKey){e.preventDefault();command(i);}
});
function transform(){const rect=canvas.getBoundingClientRect();return mapTransform(rect.width,rect.height,zoom,pan);}
function worldPoint(e){const r=canvas.getBoundingClientRect(),t=transform();return {x:Math.round((e.clientX-r.left-t.ox)/t.scale),y:Math.round((e.clientY-r.top-t.oy)/t.scale)};}
canvas.addEventListener('pointerdown',e=>{canvas.focus();if(!canCommand()||e.button!==0)return;drag={start:worldPoint(e),current:worldPoint(e),shift:e.shiftKey};canvas.setPointerCapture(e.pointerId);});
canvas.addEventListener('pointermove',e=>{hover=worldPoint(e);if(drag)drag.current=hover;});
canvas.addEventListener('pointerup',e=>{if(!drag||!canCommand())return;const p=worldPoint(e),start=drag.start;drag=null;
    if(p.x<0||p.x>=90000||p.y<0||p.y>=60000)return;
    if(pending){if(pending.type==='rendezvous'){rendezvous=p;$('rendezvousInfo').textContent=`Next delivery: ${Math.round(p.x/100)}, ${Math.round(p.y/100)} m`;toast('Delivery rendezvous set for your next order.');}
        else if(pending.type==='fallback'){fallback=p;toast('Fallback position set.');}
        else if(pending.type==='general'){if(!dispatch({verb:'move',manner:'no_pursuit',after:'hold'},p))return;}
        else if(pending.type==='assign'){const s=source();send({type:'assign',source:s,people:[...selected],role:$('duty').value,target:p,post:postId,escort:s.kind==='general'?s.id:null});}
        else{const preset=pending.preset;const area=e.ctrlKey&&['hold','observe','engage'].includes(preset.verb)?Math.min(12000,Math.round(Math.hypot(p.x-start.x,p.y-start.y))):0;if(e.shiftKey&&preset.verb==='move'){if(waypoints.length<8){waypoints.push(p);toast(`Waypoint ${waypoints.length} added; click without Shift to finish.`);}return;}if(!dispatch(preset,area?start:p,{area,route:waypoints,rendezvous,fallback}))return;rendezvous=null;$('rendezvousInfo').textContent='Delivery: last reported position';}
        pending=null;waypoints=[];updateHint();return;
    }
    const distance=Math.hypot(p.x-start.x,p.y-start.y);
    if(distance<=700){const badge=groupHits.find(b=>Math.abs(p.x-b.x)<b.w/2&&Math.abs(p.y-b.y)<b.h/2);if(badge){selectGroup(badge.index);return;}}
    // Selecting infrastructure preserves the workers selected for the job.
    if(distance<=700){const post=view.posts.find(q=>Math.hypot(q.pos.x-p.x,q.pos.y-p.y)<1100);const site=view.terrain.sites.findIndex(q=>Math.hypot(q.x-p.x,q.y-p.y)<1100);if(post||site>=0){postId=post?.id??null;siteId=post?null:site;renderPost();setDesk('works');return;}}
    if(!e.shiftKey)selected.clear();
    if(distance>700){for(const q of displayPeople){if(q.side===view.side&&q.hp>0&&q.pos.x>=Math.min(p.x,start.x)&&q.pos.x<=Math.max(p.x,start.x)&&q.pos.y>=Math.min(p.y,start.y)&&q.pos.y<=Math.max(p.y,start.y))selected.add(q.id);}}
    else{const nearby=displayPeople.filter(q=>q.side===view.side&&q.hp>0&&Math.hypot(q.pos.x-p.x,q.pos.y-p.y)<1400).sort((a,b)=>Math.hypot(a.pos.x-p.x,a.pos.y-p.y)-Math.hypot(b.pos.x-p.x,b.pos.y-p.y));if(nearby[0]){if(e.shiftKey&&selected.has(nearby[0].id))selected.delete(nearby[0].id);else selected.add(nearby[0].id);}else{postId=null;siteId=null;const post=view.posts.find(q=>Math.hypot(q.pos.x-p.x,q.pos.y-p.y)<2000);if(post)postId=post.id;else{const index=view.terrain.sites.findIndex(q=>Math.hypot(q.x-p.x,q.y-p.y)<2200);if(index>=0)siteId=index;}}}
    renderSelection();renderPost();
});
canvas.addEventListener('contextmenu',e=>{e.preventDefault();if(dispatch({verb:'move',manner:'together',after:'hold'},worldPoint(e))){pending=null;waypoints=[];updateHint();}});
canvas.addEventListener('dblclick',e=>{if(!canCommand()||pending)return;const point=worldPoint(e),p=displayPeople.filter(p=>p.side===view.side&&p.hp>0&&distance(p.pos,point)<1800).sort((a,b)=>distance(a.pos,point)-distance(b.pos,point))[0];if(!p)return;const i=groups.findIndex(g=>g.includes(p.id));if(i>=0)selectGroup(i);else{selected=new Set([p.id]);renderSelection();}});
canvas.addEventListener('pointerleave',()=>{hover=null;});
canvas.addEventListener('pointercancel',()=>{drag=null;});
canvas.addEventListener('wheel',e=>{e.preventDefault();const before=worldPoint(e),r=canvas.getBoundingClientRect();zoom=Math.max(1,Math.min(3,zoom*(e.deltaY<0?1.2:1/1.2)));if(zoom===1)pan={x:0,y:0};else{const t=transform();pan.x+=e.clientX-r.left-(t.ox+before.x*t.scale);pan.y+=e.clientY-r.top-(t.oy+before.y*t.scale);}},{passive:false});
$('fitMap').onclick=()=>{zoom=1;pan={x:0,y:0};};
$('source').onchange=()=>renderSelection();
function renderSelection(){if(!view)return;refreshGroups();renderDock();$('deliveryHint').textContent=advice(view,[...selected],source());$('count').textContent=selected.size;const people=view.people.filter(p=>selected.has(p.id));const counts={};for(const p of people)counts[p.role]=(counts[p.role]||0)+1;const ages=people.map(p=>Math.max(0,(view.tick-p.observed)/10));$('selectionInfo').textContent=people.length?`${Object.entries(counts).map(([k,v])=>`${v} ${k}`).join(', ')} · oldest observation ${Math.floor(Math.max(...ages))}s ago · ${people.filter(p=>p.hp>0).length} known alive · ${people.reduce((n,p)=>n+(p.cargo?.[0]||0),0)} carried food · ${Math.max(...people.map(p=>p.missed_meals||0))} max missed meals`:'Click or drag across your people. Shift adds.';}
function renderDock(){
    const people=selectedPeople(view,[...selected]),active=people.length>0;
    $('commandDock').hidden=!canCommand()||!active||!!pending;$('selectionPrompt').hidden=!canCommand()||active;$('game').classList.toggle('hasSelection',active);
    const group=groups.findIndex(g=>g.filter(id=>view.people.some(p=>p.id===id&&p.hp>0)).length===people.length&&people.every(p=>g.includes(p.id)));
    const roles=[...new Set(people.map(p=>p.role))];
    $('selectionName').textContent=group>=0?groupNames[group]:roles.length===1?`${roles[0][0].toUpperCase()+roles[0].slice(1)}${people.length>1?'s':''}`:'Mixed selection';
    $('dockCount').textContent=`${people.length} selected`;
    document.querySelectorAll('#commands button').forEach((b,i)=>{const issue=commandIssue(presets[i].verb);b.setAttribute('aria-disabled',String(!!issue));b.title=issue||b.dataset.description;});
    document.querySelectorAll('#generals button').forEach(b=>b.classList.toggle('active',selected.has(Number(b.dataset.person))));
    $('moveGeneral').hidden=!people.some(p=>p.role==='general');
    $('commandFeedback').title=$('commandFeedback').textContent;$('deliveryHint').title=advice(view,[...selected],source());
}
function renderPost(){if(!view)return;const p=view.posts.find(q=>q.id===postId);$('postInfo').textContent=p?`Outpost ${p.id} · ${p.side===view.side?'friendly':'enemy'} · ${p.hp} health · ${p.complete?'built':'construction'} · ${p.staff} observed staff · ${p.upgraded?'upgraded relay':'base relay'}${p.work!==null?' · work '+(p.work/10).toFixed(1)+'s':''}${p.stock?' · '+p.stock[1]+' materials':''} · observed ${Math.floor((view.tick-p.observed)/10)}s ago`:siteId!==null?`Hill site ${siteId+1} selected · 20 materials, 60 worker-seconds`:'Click an outpost or an empty hill site.';}
function renderPanels(rematch,bot,canAddBot){
    botMatch=bot;renderAvailability();
    $('botInfo').hidden=!bot;$('addBot').hidden=!canAddBot;if(bot)$('inviteArea').hidden=true;
    $('clock').textContent=time(view.tick);$('foodCount').textContent=view.stock[0];$('materialCount').textContent=view.stock[1];$('matchPhase').textContent=view.status.state==='running'?'IN THE FIELD':view.status.state==='finished'?'ROUND COMPLETE':'DEPLOYMENT';
    const status=view.status;$('matchStatus').textContent=status.state==='lobby'?`You are ${view.side===0?'West':'East'} · ${bot?'practice bot ready':view.connected[1-view.side]?'opponent connected':'waiting for opponent'}`:status.state==='finished'?`${status.winner===null?'Draw':status.winner===view.side?'Victory':'Defeat'} · ${status.reason}`:`${view.side===0?'West':'East'} command · ${bot?'DEV · vs bot':view.connected[1-view.side]?'opponent connected':'opponent reconnecting'}`;
    $('surrender').disabled=status.state!=='running';$('rematch').hidden=status.state!=='finished';$('replay').hidden=status.state!=='finished';$('rematch').textContent=rematch?.[view.side]?'Rematch requested':bot?'Play again vs bot':'Request rematch';
    const old=$('source').value;const sources=[{value:'auto',label:'Auto · nearest usable command'},{value:'hq',label:'Headquarters'}];for(const p of view.people.filter(p=>p.side===view.side&&p.role==='general'&&p.hp>0))sources.push({value:`general:${p.id}`,label:`General ${p.id} · local command`});for(const p of view.posts.filter(p=>p.side===view.side&&p.complete&&p.hp>0))sources.push({value:`outpost:${p.id}`,label:`Outpost ${p.id} · last heard ${Math.floor((view.tick-p.observed)/10)}s`});
    const sourceKey=sources.map(s=>s.value).join(",");if($('source').dataset.entries!==sourceKey){$('source').dataset.entries=sourceKey;$('source').replaceChildren(...sources.map(s=>{const o=textNode('option',s.label);o.value=s.value;return o;}));if(sources.some(s=>s.value===old))$('source').value=old;}sources.forEach((s,i)=>{if($('source').options[i].textContent!==s.label)$('source').options[i].textContent=s.label;});
    const generals=view.people.filter(p=>p.side===view.side&&p.role==='general'&&p.hp>0),generalKey=generals.map(p=>p.id).join(',');if($('generals').dataset.entries!==generalKey){$('generals').dataset.entries=generalKey;$('generals').replaceChildren(...generals.map(p=>{const b=textNode('button',`◇ General ${p.id}`);b.dataset.person=p.id;b.setAttribute('aria-label',`Select general ${p.id}`);b.title='Select this general · right-click to move';b.onclick=()=>{cancelTarget();selected=new Set([p.id]);$('source').value='auto';renderSelection();};return b;}));}
    for(const p of view.people.filter(p=>p.side===view.side)){let b=document.getElementById(`person-${p.id}`);if(!b){b=textNode('button','');b.id=`person-${p.id}`;b.onclick=e=>{if(!e.shiftKey)selected.clear();selected.add(p.id);renderSelection();};$('roster').append(b);}b.textContent=`${p.id} ${p.role} · ${p.hp} HP · ${p.cargo?.[0]??'?'} food · ${Math.floor((view.tick-p.observed)/10)}s old`;b.classList.toggle('active',selected.has(p.id));}
    $('orders').replaceChildren(...view.orders.slice(0,8).map(o=>{const info=orderSummary(o),e=textNode('div',`#${o.id} ${o.plan.verb.toUpperCase()} · ${info.text}`);e.dataset.state=info.kind;return e;}));
    const latest=view.orders.find(o=>o.recipients.some(id=>selected.has(id)));$('commandFeedback').textContent=latest?`${latest.plan.verb.toUpperCase()} · ${orderSummary(latest).text} · sent ${Math.floor((view.tick-latest.issued)/10)}s ago`:selected.size&&!Array.from(selected).some(id=>feedbackSelection.has(id))?'Selection ready · right-click to MOVE, or choose 1–5.':feedback;
    const enemies=view.people.filter(p=>p.side!==view.side&&p.hp>0&&!p.absent_since);const current=enemies.filter(p=>view.tick-p.observed<30);$('fieldAlert').textContent=fieldAlertUntil>performance.now()?fieldAlert:canCommand()&&view.tick<250&&!selected.size?'Move line + ranged infantry. Keep a general nearby for reports.':'';$('fieldAlert').hidden=!$('fieldAlert').textContent;
    $('intelStatus').textContent=current.length?`${current.length} enemies in recent reports`:enemies.length?`${enemies.length} remembered enemy contacts`:'No contacts reported';
    const meaningful=view.reports.filter(r=>r.source.startsWith('scout:')||r.source.startsWith('courier:')||r.people.some(p=>p.side!==view.side));const unique=new Map();for(const r of meaningful)unique.set(r.source,r);$('reports').replaceChildren(...[...unique.values()].reverse().slice(0,8).map(r=>textNode('div',`${r.source} · observed ${Math.floor((view.tick-r.observed)/10)}s ago · ${r.people.filter(p=>p.side!==view.side&&p.hp>0).length} enemy people`)));
    $('log').replaceChildren(...view.log.slice(-8).reverse().map(n=>textNode('div',`${time(n.tick)} ${n.text}`)));renderSelection();renderPost();
}
function draw(now=performance.now()){
    requestAnimationFrame(draw);const t=transform();
    if(view){const positions=motion.sample(view,now);displayPeople=view.people.map(p=>({...p,pos:positions.get(p.id)||p.pos}));}
    markers=markers.filter(m=>m.until>now);
    for(const [id,until] of damageFlashes)if(until<=now)damageFlashes.delete(id);
    groupHits=renderer.draw({view,people:displayPeople,t,selected,groups,postId,siteId,drag,pending,waypoints,fallback,rendezvous,hover,markers,damageFlashes,now});
    $('mapScaleLine').style.width=`${10000*t.scale}px`;
    $('zoomLevel').textContent=`${Math.round(zoom*100)}%`;
}
$('legendButton').onclick=()=>{const open=$('mapLegend').hidden;$('mapLegend').hidden=!open;$('legendButton').setAttribute('aria-expanded',String(open));};
commandButtons();groupButtons();updateHint();draw();
const invite=new URLSearchParams(location.hash.slice(1)).get('invite');if(invite){history.replaceState(null,'',location.pathname);try{enter(await api('/api/join',{invite}));}catch(e){$('lobbyMessage').textContent=e.message;}}
else{try{const saved=JSON.parse(sessionStorage.getItem('g0-session'));if(saved?.token)enter(saved);}catch{}}
