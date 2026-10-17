import test from 'node:test';
import assert from 'node:assert/strict';
import {terrainEdges,terrainRegions,contactStyle,mapTransform} from '../web/field.mjs';

test('terrain outlines preserve exact banks and omit interior cell borders',()=>{
    const cells=[3,3,0,0,3,0];
    const edges=terrainEdges(cells,3,3,2);
    assert.equal(edges.length,8);
    assert(!edges.some(([x,y,x2,y2])=>x===1&&x2===1&&Math.min(y,y2)===0));
    assert(edges.some(e=>e.join() === '2,0,2,1'));
    assert(edges.some(e=>e.join() === '2,1,2,2'));
});

test('landmark placement comes from distinct public terrain regions',()=>{
    const regions=terrainRegions([2,0,2,2,0,2],2,3,2);
    assert.deepEqual(regions,[{pos:{x:500,y:1000},count:2},{pos:{x:2500,y:1000},count:2}]);
});

test('dated and cleared contacts stay hollow, including an absence at tick zero',()=>{
    const p={observed:100,absent_since:null};
    assert.equal(contactStyle(p,129).remembered,false);
    assert.equal(contactStyle(p,130).remembered,true);
    assert.equal(contactStyle({...p,absent_since:0},100).absent,true);
    assert(contactStyle({...p,absent_since:0},100).alpha<contactStyle(p,100).alpha);
});

test('fit map retains the full world and zoom uses the same invertible position transform',()=>{
    for(const [width,height] of [[1440,794],[800,494],[390,600]]){
        const t=mapTransform(width,height);
        assert(t.ox>=23&&t.oy>=31);
        assert(t.ox+90000*t.scale<=width-23);
        assert(t.oy+60000*t.scale<=height-31);
        const z=mapTransform(width,height,3,{x:48,y:-33}),point={x:42300,y:20100};
        assert(Math.abs(((z.ox+point.x*z.scale)-z.ox)/z.scale-point.x)<1e-8);
        assert.equal(z.scale,t.scale*3);
    }
});
