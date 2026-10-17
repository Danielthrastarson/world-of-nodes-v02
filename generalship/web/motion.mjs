// Render a short, buffered history of already-authorized observations. Never predict
// movement beyond the newest received position or interpolate a courier's old report.
const TICK_MS = 100;
const live = (p, tick) => p && p.hp > 0 && p.absent_since == null
    && tick - p.received <= 2
    && /^(headquarters|general:\d+|outpost:\d+)$/.test(p.source);
const sameStream = (a, b) => a && b
    && a.received - a.observed === b.received - b.observed;

export class SnapshotMotion {
    constructor(delay = 200) { this.delay = delay; this.reset(); }
    reset() { this.frames = []; this.offset = Infinity; this.time = null; this.lastFrame = null; }
    push(view, now) {
        const last = this.frames.at(-1);
        if (last && view.tick < last.tick) this.reset();
        // Duplicate snapshots (lobby, jitter, or stalled server) must not restart animation.
        if (this.frames.at(-1)?.tick === view.tick) return;
        this.offset = Math.min(this.offset, now - view.tick * TICK_MS);
        this.frames.push({tick: view.tick, people: new Map(view.people.map(p => [p.id, p]))});
        if (this.frames.length > 12) this.frames.shift();
    }
    sample(view, now) {
        if (!this.frames.length) return new Map(view.people.map(p => [p.id, p.pos]));
        const newest = this.frames.at(-1).tick * TICK_MS;
        const target = Math.min(newest, now - this.offset - this.delay);
        if (this.time === null) this.time = Math.max(this.frames[0].tick * TICK_MS, target);
        else {
            // A slightly faster catch-up absorbs clock corrections without reversing or
            // jumping when packets arrive together. An empty buffer holds its final point.
            const elapsed = Math.max(0, now - this.lastFrame);
            this.time += Math.min(Math.max(0, target - this.time), elapsed * 1.05);
        }
        this.lastFrame = now;
        let before = this.frames[0], after = before;
        for (const frame of this.frames) {
            if (frame.tick * TICK_MS <= this.time) before = frame;
            after = frame;
            if (frame.tick * TICK_MS >= this.time) break;
        }
        const span = (after.tick - before.tick) * TICK_MS;
        const alpha = span ? Math.max(0, Math.min(1, (this.time - before.tick * TICK_MS) / span)) : 0;
        return new Map(view.people.map(p => {
            const a = before.people.get(p.id), b = after.people.get(p.id);
            let pos = p.pos;
            if (live(p, view.tick) && live(a, before.tick) && live(b, after.tick)
                && sameStream(a, b) && sameStream(b, p)
                && b.observed >= a.observed && b.observed - a.observed <= 3
                && p.observed >= b.observed && p.observed - b.observed <= 3
                && b.received - a.received <= 3) {
                pos = {x: a.pos.x + (b.pos.x - a.pos.x) * alpha,
                    y: a.pos.y + (b.pos.y - a.pos.y) * alpha};
            }
            return [p.id, pos];
        }));
    }
}
