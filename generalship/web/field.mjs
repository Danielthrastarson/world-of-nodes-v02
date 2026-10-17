// A map of received facts. This module has no socket, world state, or gameplay clock.
// Positions come from the authorized observation buffer; decoration never moves people.
import {centre, distance, selectedPeople} from './controls.mjs';

export const palette = {ground:'#536451',road:'#8b8969',woods:'#304a3d',water:'#2b4c58',bridge:'#b3a787',ford:'#607f7e',hill:'#717657',ally:'#e1efbd',enemy:'#f0a38e',selected:'#bff8df',ink:'#12221e'};
const colors=[palette.ground,palette.road,palette.woods,palette.water,palette.bridge,palette.ford,palette.hill];
const W=90,H=60,CELL=1000;

export function mapTransform(width,height,zoom=1,pan={x:0,y:0}) {
    const scale=Math.max(.0001,Math.min((width-48)/90000,(height-64)/60000))*zoom;
    return {scale,ox:(width-90000*scale)/2+pan.x,oy:(height-60000*scale)/2+pan.y,width,height};
}

// Border segments stay on the actual navigation cells. No softened bank or woodland
// silhouette suggests that impassable ground is walkable or that cover exists elsewhere.
export function terrainEdges(cells,kind,width=W,height=H) {
    const edges=[],at=(x,y)=>x>=0&&y>=0&&x<width&&y<height?cells[y*width+x]:-1;
    for(let y=0;y<height;y++)for(let x=0;x<width;x++)if(at(x,y)===kind){
        if(at(x,y-1)!==kind)edges.push([x,y,x+1,y]);
        if(at(x+1,y)!==kind)edges.push([x+1,y,x+1,y+1]);
        if(at(x,y+1)!==kind)edges.push([x+1,y+1,x,y+1]);
        if(at(x-1,y)!==kind)edges.push([x,y+1,x,y]);
    }
    return edges;
}

export function terrainRegions(cells,kind,width=W,height=H) {
    const seen=new Set(),regions=[];
    for(let i=0;i<cells.length;i++)if(cells[i]===kind&&!seen.has(i)){
        const todo=[i],points=[];seen.add(i);
        for(let n=0;n<todo.length;n++){
            const id=todo[n],x=id%width,y=Math.floor(id/width);points.push({x:(x+.5)*CELL,y:(y+.5)*CELL});
            for(const [dx,dy] of [[0,-1],[1,0],[0,1],[-1,0]]){
                const nx=x+dx,ny=y+dy,j=ny*width+nx;
                if(nx>=0&&ny>=0&&nx<width&&ny<height&&cells[j]===kind&&!seen.has(j)){seen.add(j);todo.push(j);}
            }
        }
        regions.push({pos:centre(points.map(pos=>({pos}))),count:points.length});
    }
    return regions;
}

export function contactStyle(contact,tick) {
    const age=Math.max(0,tick-contact.observed),absent=contact.absent_since!=null;
    return {age,remembered:age>=30||absent,absent,alpha:absent?.36:age>=30?.62:1};
}

export class FieldRenderer {
    constructor(canvas){this.canvas=canvas;this.ctx=canvas.getContext('2d');this.reset();}
    reset(){this.terrain=null;this.layer=null;this.landmarks=[];}

    makeTerrain(terrain){
        this.terrain=terrain;
        const layer=document.createElement('canvas');layer.width=1800;layer.height=1200;
        const c=layer.getContext('2d'),s=20;
        for(let y=0;y<H;y++)for(let x=0;x<W;x++){
            const kind=terrain.cells[y*W+x];c.fillStyle=colors[kind];c.fillRect(x*s,y*s,s,s);
            // Static map hatching encodes terrain type, not vegetation or water motion.
            if(kind===2){c.strokeStyle='#a8c3a51b';c.lineWidth=1;c.beginPath();c.moveTo(x*s,y*s+14);c.lineTo(x*s+14,y*s);c.moveTo(x*s+6,y*s+20);c.lineTo(x*s+20,y*s+6);c.stroke();}
            if(kind===3||kind===5){c.strokeStyle=kind===3?'#b8d6d41d':'#d0e5d64d';c.lineWidth=1;c.beginPath();c.moveTo(x*s+4,y*s+6);c.lineTo(x*s+16,y*s+6);c.moveTo(x*s+4,y*s+16);c.lineTo(x*s+16,y*s+16);c.stroke();}
            if(kind===4){c.strokeStyle='#423e2f66';c.lineWidth=1;c.beginPath();c.moveTo(x*s,y*s+5);c.lineTo((x+1)*s,y*s+5);c.moveTo(x*s,y*s+15);c.lineTo((x+1)*s,y*s+15);c.stroke();}
            if(kind===6){c.fillStyle='#d6d9ac30';c.fillRect(x*s+9,y*s+9,2,2);}
        }
        for(const kind of [1,2,3,4,5,6]){
            c.strokeStyle=kind===3?'#a1b8a76b':kind===2?'#162f2499':'#dce4bd42';c.lineWidth=kind===2?2:1.5;c.beginPath();
            for(const [x,y,x2,y2] of terrainEdges(terrain.cells,kind)){c.moveTo(x*s,y*s);c.lineTo(x2*s,y2*s);}c.stroke();
        }
        // 100 m survey grid; it has no effect on sensing, navigation, or concealment.
        c.strokeStyle='#e5ecd417';c.lineWidth=1;c.beginPath();
        for(let x=0;x<=W;x+=10){c.moveTo(x*s,0);c.lineTo(x*s,H*s);}
        for(let y=0;y<=H;y+=10){c.moveTo(0,y*s);c.lineTo(W*s,y*s);}c.stroke();
        this.layer=layer;
        this.landmarks=[...[2,4,5].flatMap(kind=>terrainRegions(terrain.cells,kind).map(r=>({...r,kind})))];
    }

    label(text,x,y,{color='#e4ebd5',size=10,plate=true,selected=false,muted=false}={}){
        const c=this.ctx,s=this.t.scale,px=1/s;c.save();c.font=`${selected?650:500} ${size*px}px system-ui`;c.textAlign='center';c.textBaseline='middle';
        const w=c.measureText(text).width+16*px,h=23*px;
        if(plate){c.fillStyle=selected?'#19382ff5':'#162822ed';c.strokeStyle=selected?'#bff8df9c':'#c3d3aa30';c.lineWidth=px;c.beginPath();c.roundRect(x-w/2,y-h/2,w,h,4*px);c.fill();c.stroke();}
        c.fillStyle=color;c.globalAlpha*=muted?.75:1;c.fillText(text,x,y);c.restore();return {x,y,w,h};
    }

    draw({view,people,t,selected,groups,postId,siteId,drag,pending,waypoints,fallback,rendezvous,hover,markers,damageFlashes,now}){
        this.t=t;const c=this.ctx,dpr=devicePixelRatio||1,px=1/t.scale;
        if(this.canvas.width!==Math.round(t.width*dpr)||this.canvas.height!==Math.round(t.height*dpr)){this.canvas.width=Math.round(t.width*dpr);this.canvas.height=Math.round(t.height*dpr);}
        c.setTransform(dpr,0,0,dpr,0,0);c.clearRect(0,0,t.width,t.height);
        if(!view)return [];
        // Rematches invalidate the layer explicitly. Ordinary snapshots carry identical terrain.
        if(!this.layer)this.makeTerrain(view.terrain);
        c.save();c.translate(t.ox,t.oy);c.scale(t.scale,t.scale);
        c.shadowColor='#00000040';c.shadowBlur=28;c.fillStyle=palette.ground;c.fillRect(0,0,90000,60000);c.shadowBlur=0;
        c.imageSmoothingEnabled=true;c.drawImage(this.layer,0,0,90000,60000);
        c.strokeStyle='#c8d2b252';c.lineWidth=px;c.strokeRect(0,0,90000,60000);
        c.font=`9px ui-monospace`; // Coordinate text is drawn below in screen units.
        const occupied=[];
        for(const r of this.landmarks){
            const text=r.kind===2?'WOODS':r.kind===4?'BRIDGE':'FORD';
            this.label(text,r.pos.x,r.pos.y+(r.kind===2?0:-2400),{size:9,plate:r.kind!==2,color:r.kind===2?'#c3d1b4':'#d1e1df',muted:true});
        }
        for(let side=0;side<2;side++){
            const p=view.terrain.headquarters[side],color=side===view.side?palette.ally:palette.enemy;
            c.strokeStyle=color;c.lineWidth=1.5*px;c.fillStyle=side===view.side?'#dfeebd0a':'#efa38e0a';c.fillRect(p.x-1700,p.y-1700,3400,3400);c.strokeRect(p.x-1700,p.y-1700,3400,3400);
            // The public capture area is configuration, not a prediction of its progress.
            c.globalAlpha=.25;c.setLineDash([3*px,5*px]);c.beginPath();c.arc(p.x,p.y,view.config.capture_radius,0,Math.PI*2);c.stroke();c.setLineDash([]);c.globalAlpha=1;
            occupied.push(this.label(side===view.side?'YOUR HQ':'ENEMY HQ',p.x+(side===0?-1:1)*(1900+31*px),p.y,{color,size:9}));
        }
        view.terrain.sites.forEach((p,i)=>{
            c.strokeStyle=siteId===i?palette.selected:'#d8dfb477';c.lineWidth=px;c.setLineDash([3*px,4*px]);c.strokeRect(p.x-1100,p.y-1100,2200,2200);c.setLineDash([]);
        });
        for(const post of view.posts){
            const p=post.pos,style=contactStyle(post,view.tick);c.save();c.globalAlpha=style.alpha;c.strokeStyle=postId===post.id?palette.selected:post.side===view.side?palette.ally:palette.enemy;c.lineWidth=1.5*px;c.fillStyle='#1d342c';c.fillRect(p.x-950,p.y-950,1900,1900);c.strokeRect(p.x-950,p.y-950,1900,1900);
            if(post.upgraded)c.strokeRect(p.x-1300,p.y-1300,2600,2600);
            if(post.hp<=0){c.beginPath();c.moveTo(p.x-900,p.y-900);c.lineTo(p.x+900,p.y+900);c.stroke();}
            if(postId===post.id)this.label(`POST ${post.id} · ${Math.floor(style.age/10)}s ago`,p.x,p.y-950-18*px,{selected:true});c.restore();
        }
        const drawnOrders=new Set();
        for(const o of view.orders.slice(0,12)){
            if(o.recipients.every(id=>drawnOrders.has(id)))continue;o.recipients.forEach(id=>drawnOrders.add(id));
            if(!o.recipients.some(id=>selected.has(id)))continue;
            const p=centre(people.filter(p=>o.recipients.includes(p.id)&&p.hp>0));if(!p)continue;
            c.strokeStyle='#f2dba28c';c.lineWidth=1.3*px;c.setLineDash([5*px,5*px]);c.beginPath();c.moveTo(p.x,p.y);for(const w of o.plan.route)c.lineTo(w.x,w.y);c.lineTo(o.plan.target.x,o.plan.target.y);c.stroke();c.setLineDash([]);
            c.beginPath();c.arc(o.plan.target.x,o.plan.target.y,6*px,0,Math.PI*2);c.stroke();
            this.label(`${o.plan.verb.toUpperCase()} · PLAN`,o.plan.target.x,o.plan.target.y+19*px,{color:'#ead7a8',size:9});
        }
        const selection=selectedPeople({...view,people},[...selected]);
        if(selection.length>1){
            const left=Math.min(...selection.map(p=>p.pos.x))-9*px,right=Math.max(...selection.map(p=>p.pos.x))+9*px;
            const top=Math.min(...selection.map(p=>p.pos.y))-9*px,bottom=Math.max(...selection.map(p=>p.pos.y))+9*px;
            c.strokeStyle='#bff8dfa0';c.lineWidth=px;c.beginPath();
            for(const [x,y,dx,dy] of [[left,top,1,1],[right,top,-1,1],[left,bottom,1,-1],[right,bottom,-1,-1]]){c.moveTo(x+dx*6*px,y);c.lineTo(x,y);c.lineTo(x,y+dy*6*px);}c.stroke();
        }
        // Each visible dot stays at its reported physical position. A hollow marker is a
        // dated/absent contact, not a live unit; it never acquires a fabricated trajectory.
        for(const p of people){
            const pos=p.pos,style=contactStyle(p,view.tick),active=selected.has(p.id),color=p.side===view.side?palette.ally:palette.enemy;
            const radius=p.role==='general'?5.5*px:Math.max(2.6*px,view.config.person_radius*.84);
            c.save();c.globalAlpha=style.alpha;c.lineWidth=1.1*px;c.strokeStyle=style.remembered?color:palette.ink;c.fillStyle=color;
            if(p.hp<=0){c.globalAlpha*=.65;c.strokeStyle=color;const r=2.8*px;c.beginPath();c.moveTo(pos.x-r,pos.y-r);c.lineTo(pos.x+r,pos.y+r);c.moveTo(pos.x+r,pos.y-r);c.lineTo(pos.x-r,pos.y+r);c.stroke();c.restore();continue;}
            if(active&&selected.size===1){c.strokeStyle=palette.selected;c.lineWidth=px;c.beginPath();c.arc(pos.x,pos.y,radius+3.3*px,0,Math.PI*2);c.stroke();c.strokeStyle=style.remembered?color:palette.ink;}
            c.beginPath();if(p.role==='general'){c.moveTo(pos.x,pos.y-radius);c.lineTo(pos.x+radius,pos.y);c.lineTo(pos.x,pos.y+radius);c.lineTo(pos.x-radius,pos.y);c.closePath();}else c.arc(pos.x,pos.y,radius,0,Math.PI*2);
            if(!style.remembered)c.fill();c.stroke();
            if(!style.remembered&&['ranged','courier','carrier','scout'].includes(p.role)){
                c.strokeStyle=palette.ink;c.lineWidth=px;c.beginPath();
                if(p.role==='ranged')c.arc(pos.x,pos.y,radius*.46,0,Math.PI*2);
                else {c.moveTo(pos.x-radius*.5,pos.y);c.lineTo(pos.x+radius*.5,pos.y);if(p.role==='carrier'){c.moveTo(pos.x,pos.y-radius*.5);c.lineTo(pos.x,pos.y+radius*.5);}}
                c.stroke();
            }
            if(p.hp<view.config.person_hp&&(active||style.age<30)){
                c.fillStyle='#14211c';c.fillRect(pos.x-5*px,pos.y-radius-5*px,10*px,2*px);c.fillStyle=color;c.fillRect(pos.x-5*px,pos.y-radius-5*px,10*px*Math.max(0,p.hp)/view.config.person_hp,2*px);
            }
            c.restore();
        }
        const hits=[];
        groups.forEach((ids,index)=>{
            const members=selectedPeople({...view,people},ids);if(!members.length||index>2&&!members.some(p=>selected.has(p.id)))return;
            const p=centre(members),age=Math.floor(Math.max(...members.map(p=>view.tick-p.observed))/10),active=members.some(p=>selected.has(p.id));
            const name=['LINE','RANGED','LIGHT','COURIERS','SUPPLY'][index],text=`${name}  ${members.length}${age>=3?' · '+age+'s ago':''}`;
            let y=Math.min(...members.map(p=>p.pos.y))-20*px;c.font=`${active?650:500} ${10*px}px system-ui`;
            const w=c.measureText(text).width+16*px,h=23*px;
            let x=Math.max(w/2+4*px,Math.min(90000-w/2-4*px,p.x));
            for(let n=0;n<8;n++){if(!occupied.some(b=>Math.abs(x-b.x)<(w+b.w)/2+4*px&&Math.abs(y-b.y)<(h+b.h)/2+3*px))break;y-=h+4*px;}
            c.strokeStyle=active?'#bff8df88':'#c3d3aa55';c.lineWidth=px;c.beginPath();c.moveTo(p.x,p.y);c.lineTo(x,y+h/2);c.stroke();
            const bounds=this.label(text,x,y,{selected:active,color:active?palette.selected:palette.ally,muted:age>=3});hits.push({...bounds,index});occupied.push(bounds);
        });
        if(selected.size===1){
            const p=selectedPeople({...view,people},[...selected])[0];
            if(p){
                const radius=p.role==='general'?view.config.exchange_radius:p.role==='ranged'?view.config.ranged_range:view.config.melee_range;
                c.strokeStyle='#d5edb64d';c.lineWidth=px;c.setLineDash([3*px,5*px]);c.beginPath();c.arc(p.pos.x,p.pos.y,radius,0,Math.PI*2);c.stroke();c.setLineDash([]);
                this.label(`${p.role.toUpperCase()} ${p.id}`,p.pos.x,p.pos.y-23*px,{selected:true,size:9});
            }
        }
        for(const p of people){const until=damageFlashes.get(p.id);if(until>now){const progress=1-(until-now)/700;c.save();c.globalAlpha=1-progress;c.strokeStyle='#ffb18e';c.lineWidth=1.5*px;c.beginPath();c.arc(p.pos.x,p.pos.y,(5+progress*9)*px,0,Math.PI*2);c.stroke();c.restore();}}
        if(drag){
            c.strokeStyle=palette.selected;c.fillStyle='#bff8df10';c.lineWidth=px;
            if(pending?.type==='order'&&['hold','observe','engage'].includes(pending.preset.verb)){c.beginPath();c.arc(drag.start.x,drag.start.y,Math.min(12000,distance(drag.current,drag.start)),0,Math.PI*2);c.fill();c.stroke();}
            else if(!pending){c.fillRect(drag.start.x,drag.start.y,drag.current.x-drag.start.x,drag.current.y-drag.start.y);c.strokeRect(drag.start.x,drag.start.y,drag.current.x-drag.start.x,drag.current.y-drag.start.y);}
        }
        for(const [p,name,color] of [[fallback,'FALLBACK','#ecd5a3'],[rendezvous,'RENDEZVOUS','#b2dce3']])if(p){c.strokeStyle=color;c.lineWidth=px;c.beginPath();c.arc(p.x,p.y,7*px,0,Math.PI*2);c.stroke();this.label(name,p.x,p.y+20*px,{color,size:9});}
        if(waypoints.length){c.strokeStyle='#d5edb6';c.lineWidth=px;c.setLineDash([3*px,4*px]);c.beginPath();waypoints.forEach((p,i)=>i?c.lineTo(p.x,p.y):c.moveTo(p.x,p.y));if(hover)c.lineTo(hover.x,hover.y);c.stroke();c.setLineDash([]);}
        waypoints.forEach((p,i)=>{c.fillStyle=palette.selected;c.beginPath();c.arc(p.x,p.y,3*px,0,Math.PI*2);c.fill();this.label(String(i+1),p.x,p.y-15*px,{plate:false,size:10});});
        for(const m of markers.filter(m=>m.until>now)){c.save();c.globalAlpha=Math.min(1,(m.until-now)/500);c.strokeStyle=palette.selected;c.lineWidth=1.5*px;c.beginPath();c.arc(m.target.x,m.target.y,8*px,0,Math.PI*2);c.stroke();c.beginPath();for(const [dx,dy] of [[1,0],[-1,0],[0,1],[0,-1]]){c.moveTo(m.target.x+dx*11*px,m.target.y+dy*11*px);c.lineTo(m.target.x+dx*15*px,m.target.y+dy*15*px);}c.stroke();c.restore();}
        if(hover&&!drag&&!pending){
            const p=people.filter(p=>distance(p.pos,hover)<Math.max(600,9*px)).sort((a,b)=>distance(a.pos,hover)-distance(b.pos,hover))[0];
            if(p){const style=contactStyle(p,view.tick);this.label(`${p.side===view.side?'Friendly':'Enemy'} ${p.role} #${p.id} · ${p.hp} HP · ${Math.floor(style.age/10)}s ago${style.absent?' · left area':''}`,p.pos.x,p.pos.y-51*px,{color:'#f5e8c9',size:11});}
        }
        c.restore();
        // Survey ticks use screen-size type, even when the player zooms in.
        c.save();c.font='9px ui-monospace, monospace';c.fillStyle='#b3c2b16b';c.textAlign='center';
        for(let x=0;x<=90000;x+=10000){const sx=t.ox+x*t.scale;if(sx>20&&sx<t.width-20)c.fillText(`${x/100}`,sx,Math.max(19,t.oy-9));}
        c.textAlign='right';for(let y=10000;y<60000;y+=10000){const sy=t.oy+y*t.scale;if(sy>70&&sy<t.height-32)c.fillText(`${y/100}`,Math.max(24,t.ox-9),sy+3);}
        c.restore();return hits;
    }
}
