//! Backend-neutral world presentation. No graphics API, World, Session, commands,
//! random generator or wall clock. Inputs are already-permitted observations.
use std::collections::BTreeMap;
use world_of_nodes::provisioning::playable::snapshot::*;
use world_of_nodes::provisioning::{HOUR, Holder, Location, SCALE};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
        )
    }
    pub fn offset(self, x: f32, y: f32) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}
/// Display metadata only: one stable coordinate system, independent of camera,
/// visibility, resource contents and simulation path cost.
#[derive(Clone, Debug)]
pub struct Geography {
    pub nodes: BTreeMap<usize, Point>,
    pub route_bends: BTreeMap<(usize, usize), f32>,
}
impl Default for Geography {
    fn default() -> Self {
        Self {
            nodes: [
                (0, Point::new(90., 385.)),
                (1, Point::new(430., 395.)),
                (2, Point::new(960., 405.)),
                (3, Point::new(710., 90.)),
                (4, Point::new(1200., 160.)),
            ]
            .into(),
            route_bends: BTreeMap::new(),
        }
    }
}
impl Geography {
    pub fn node(&self, id: usize) -> Point {
        self.nodes.get(&id).copied().unwrap_or_else(|| {
            Point::new(80. + (id % 6) as f32 * 155., 520. + (id / 6) as f32 * 130.)
        })
    }
    pub fn route(&self, a: usize, b: usize, width: f32) -> RouteGeometry {
        let (low, high) = if a <= b { (a, b) } else { (b, a) };
        let start = self.node(low);
        let end = self.node(high);
        let bend = self
            .route_bends
            .get(&(low, high))
            .copied()
            .unwrap_or((((low.wrapping_mul(3).wrapping_add(high)) % 3) as f32 - 1.) * 27.);
        RouteGeometry {
            a: low,
            b: high,
            start,
            end,
            bend,
            visual_width: width,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct RouteGeometry {
    pub a: usize,
    pub b: usize,
    pub start: Point,
    pub end: Point,
    pub bend: f32,
    pub visual_width: f32,
}
impl RouteGeometry {
    /// A canonical parametric curve; reversing direction retraces it exactly.
    pub fn point(&self, from: usize, t: f32) -> Point {
        debug_assert!(from == self.a || from == self.b);
        let t = if from == self.a { t } else { 1. - t }.clamp(0., 1.);
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let len = dx.hypot(dy).max(0.001);
        let wave = (std::f32::consts::PI * t).sin() * self.bend;
        self.start
            .lerp(self.end, t)
            .offset(-dy / len * wave, dx / len * wave)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Terrain,
    Water,
    ResourceAreas,
    Routes,
    Infrastructure,
    Actors,
    Effects,
    Lighting,
    Overlays,
    Ui,
}
pub const LAYERS: [Layer; 10] = [
    Layer::Terrain,
    Layer::Water,
    Layer::ResourceAreas,
    Layer::Routes,
    Layer::Infrastructure,
    Layer::Actors,
    Layer::Effects,
    Layer::Lighting,
    Layer::Overlays,
    Layer::Ui,
];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionKind {
    Settlement,
    Fertile,
    WoodDeposit,
    Water,
}
#[derive(Clone, Debug)]
pub struct Region {
    pub kind: RegionKind,
    pub source_id: usize,
    pub center: Point,
    pub half_size: Point,
    pub evidence: Evidence,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lod {
    Far,
    Medium,
    Close,
}
impl Lod {
    pub fn at_zoom(zoom: f32) -> Self {
        if zoom < 0.5 {
            Self::Far
        } else if zoom < 0.95 {
            Self::Medium
        } else {
            Self::Close
        }
    }
}
#[derive(Clone, Debug)]
pub struct Lighting {
    pub ambient: f32,
    pub sun_direction: Point,
    pub phase: f32,
    pub local: Vec<RadialLight>,
}
#[derive(Clone, Debug)]
pub struct RadialLight {
    pub source: VisualSource,
    pub position: Point,
    pub radius: f32,
    pub intensity: f32,
}
#[derive(Clone, Copy, Debug)]
pub enum VisualSource {
    Person(usize),
    #[allow(dead_code)] // Reserved source type; no crossing particles/lights are spawned in W0.
    Crossing(usize),
}
#[derive(Clone, Copy, Debug)]
pub enum EffectKind {
    Movement,
    #[allow(dead_code)] // Interface only until actual construction effects are approved.
    Construction,
}
#[derive(Clone, Debug)]
pub struct EffectCue {
    pub source: VisualSource,
    pub kind: EffectKind,
    pub position: Point,
    pub observed_at: i64,
}
impl Lighting {
    pub fn at_time(time: i64) -> Self {
        // Presentation-only 24h cycle: T=0 is dawn (06:00 solar phase), not
        // a change to the simulation calendar or a cause of work/visibility.
        let phase = (time.rem_euclid(24 * HOUR) as f64 / (24 * HOUR) as f64) as f32;
        let angle = phase * std::f32::consts::TAU;
        let daylight = angle.sin().max(0.);
        Self {
            ambient: 0.86 + 0.14 * daylight,
            sun_direction: Point::new(angle.cos(), angle.sin()),
            phase,
            local: vec![],
        }
    }
}
pub fn current(e: &Evidence) -> bool {
    matches!(
        e.source,
        KnowledgeSource::Local | KnowledgeSource::SelfKnowledge | KnowledgeSource::Debug
    )
}
fn progress(p: &PersonSnapshot) -> f32 {
    let Location::Edge { progress, .. } = p.location else {
        return 0.;
    };
    (progress as f64 / (p.traversal_nominal_ms.unwrap_or(1).max(1) as f64 * SCALE as f64))
        .clamp(0., 1.) as f32
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Packing {
    Military(usize),
    Waiting,
    Working,
    Civilian,
}
fn packing(p: &PersonSnapshot) -> Packing {
    if p.action.as_deref() == Some("queued") {
        Packing::Waiting
    } else if let Some(id) = p.unit {
        Packing::Military(id)
    } else if p.action.as_deref() == Some("cultivating") {
        Packing::Working
    } else {
        Packing::Civilian
    }
}
fn anchor(group: Packing) -> Point {
    match group {
        Packing::Military(_) => Point::new(-65., 38.),
        Packing::Waiting => Point::new(76., 20.),
        Packing::Working => Point::new(-48., -69.),
        Packing::Civilian => Point::new(14., 47.),
    }
}
fn positions(s: &Snapshot, g: &Geography) -> BTreeMap<usize, Point> {
    let mut slots = BTreeMap::<(usize, Packing), usize>::new();
    let mut people: Vec<_> = s.persons.iter().collect();
    people.sort_by_key(|p| p.id);
    people
        .into_iter()
        .map(|p| {
            let point = match p.location {
                Location::Node(n) => {
                    let group = packing(p);
                    let slot = slots.entry((n, group)).or_default();
                    let rank = *slot as f32;
                    *slot += 1;
                    let radius = rank.sqrt() * 15.;
                    let angle = rank * 2.399_963_1;
                    let at = anchor(group);
                    g.node(n)
                        .offset(at.x + angle.cos() * radius, at.y + angle.sin() * radius)
                }
                Location::Edge { from, to, .. } => g.route(from, to, 0.).point(from, progress(p)),
            };
            (p.id, point)
        })
        .collect()
}
/// Sample only between executed observations. Same-edge motion follows the
/// route parameter, never the straight chord. Reacquired or remembered people
/// are never extrapolated; mode switches never blend Debug truth into Play.
pub fn sample_person(
    now: &PersonSnapshot,
    old: Option<&PersonSnapshot>,
    at: Point,
    before: Option<Point>,
    g: &Geography,
    alpha: f32,
) -> Point {
    let alpha = if alpha.is_finite() {
        alpha.clamp(0., 1.)
    } else {
        1.
    };
    let Some(old) = old.filter(|old| current(&old.evidence) && current(&now.evidence)) else {
        return at;
    };
    match (&old.location, &now.location) {
        (
            Location::Edge {
                edge: a, from, to, ..
            },
            Location::Edge {
                edge: b,
                from: f,
                to: t,
                ..
            },
        ) if a == b && from == f && to == t => g.route(*from, *to, 0.).point(
            *from,
            progress(old) + (progress(now) - progress(old)) * alpha,
        ),
        (Location::Node(a), Location::Node(b)) if a == b => before.unwrap_or(at).lerp(at, alpha),
        (Location::Node(node), Location::Edge { from, to, .. }) if node == from => {
            // Blend the presentation-only packing offset into the known route
            // endpoint. This asserts no sub-tick gameplay position or timing.
            let start = g.node(*from);
            let packed = before.unwrap_or(start);
            g.route(*from, *to, 0.)
                .point(*from, progress(now) * alpha)
                .offset(
                    (packed.x - start.x) * (1. - alpha),
                    (packed.y - start.y) * (1. - alpha),
                )
        }
        (Location::Edge { from, to, .. }, Location::Node(node)) if node == to => {
            let end = g.node(*to);
            g.route(*from, *to, 0.)
                .point(*from, progress(old) + (1. - progress(old)) * alpha)
                .offset((at.x - end.x) * alpha, (at.y - end.y) * alpha)
        }
        // Unrelated observations or an unrecorded route change are not a license
        // to invent a connecting journey. Reacquisition snaps to known evidence.
        _ => at,
    }
}
/// Immutable backend contract. References to permitted observation payloads are
/// plain Rust data; renderer handles/textures, commands and World do not appear.
/// Optional future backends can consume these same coordinates and payloads.
pub struct RenderSnapshot<'a> {
    pub observation: &'a Snapshot,
    pub geography: &'a Geography,
    pub routes: BTreeMap<usize, RouteGeometry>,
    pub persons: BTreeMap<usize, Point>,
    pub fields: BTreeMap<usize, Point>,
    pub goods: BTreeMap<usize, Point>,
    pub crossings: BTreeMap<usize, Point>,
    pub regions: Vec<Region>,
    pub lighting: Lighting,
    pub effects: Vec<EffectCue>,
}
impl<'a> RenderSnapshot<'a> {
    pub fn project(
        s: &'a Snapshot,
        previous: Option<&Snapshot>,
        alpha: f32,
        g: &'a Geography,
    ) -> Self {
        let routes: BTreeMap<_, _> = s
            .edges
            .iter()
            .map(|e| {
                (
                    e.id,
                    g.route(
                        e.a,
                        e.b,
                        match e.path_stage.as_deref() {
                            Some("worn_path") => 11.,
                            Some("trail") => 7.,
                            _ => 4.,
                        },
                    ),
                )
            })
            .collect();
        let mut persons = positions(s, g);
        if let Some(previous) = previous.filter(|p| p.mode == s.mode && p.now <= s.now) {
            let old = positions(previous, g);
            let old_people: BTreeMap<_, _> = previous.persons.iter().map(|p| (p.id, p)).collect();
            for p in &s.persons {
                let at = persons[&p.id];
                persons.insert(
                    p.id,
                    sample_person(
                        p,
                        old_people.get(&p.id).copied(),
                        at,
                        old.get(&p.id).copied(),
                        g,
                        alpha,
                    ),
                );
            }
        }
        let mut regions = vec![];
        for n in &s.nodes {
            if n.contents_known && n.board_quantity.is_some() {
                regions.push(Region {
                    kind: RegionKind::Settlement,
                    source_id: n.id,
                    center: g.node(n.id),
                    half_size: Point::new(165., 128.),
                    evidence: n.evidence.clone(),
                });
            }
        }
        let mut slots = BTreeMap::<usize, usize>::new();
        let fields = s
            .fields
            .iter()
            .map(|f| {
                let slot = slots.entry(f.node).or_default();
                let at = g.node(f.node).offset(
                    -83. + (*slot % 3) as f32 * 66.,
                    -113. - (*slot / 3) as f32 * 47.,
                );
                *slot += 1;
                regions.push(Region {
                    kind: RegionKind::Fertile,
                    source_id: f.id,
                    center: at,
                    half_size: Point::new(29., 20.),
                    evidence: f.evidence.clone(),
                });
                (f.id, at)
            })
            .collect();
        let mut slots = BTreeMap::<(usize, bool), usize>::new();
        let goods = s
            .goods
            .iter()
            .filter_map(|item| {
                let n = item.node?;
                let wood =
                    matches!(item.holder, Holder::Deposit(_)) && item.material.contains("Wood");
                let slot = slots.entry((n, wood)).or_default();
                let at = if wood {
                    g.node(n).offset(126., 62. + *slot as f32 * 34.)
                } else {
                    g.node(n).offset(
                        66. + (*slot % 2) as f32 * 41.,
                        -33. - (*slot / 2) as f32 * 32.,
                    )
                };
                *slot += 1;
                if wood && item.quantity > 0 {
                    regions.push(Region {
                        kind: RegionKind::WoodDeposit,
                        source_id: item.id,
                        center: at,
                        half_size: Point::new(40., 44.),
                        evidence: item.evidence.clone(),
                    });
                }
                Some((item.id, at))
            })
            .collect();
        let crossings = s
            .crossings
            .iter()
            .map(|c| {
                let at = c
                    .edge
                    .and_then(|e| routes.get(&e))
                    .map(|r| r.point(r.a, 0.5))
                    .unwrap_or_else(|| g.node(c.node).offset(95., -8.));
                (c.id, at)
            })
            .collect();
        let effects = s
            .persons
            .iter()
            .filter(|p| current(&p.evidence))
            .filter_map(|p| {
                if p.action.as_deref() == Some("travelling")
                    && matches!(p.location, Location::Edge { .. })
                {
                    Some(EffectCue {
                        source: VisualSource::Person(p.id),
                        kind: EffectKind::Movement,
                        position: persons[&p.id],
                        observed_at: p.evidence.observed_at,
                    })
                } else {
                    None
                }
            })
            .collect();
        Self {
            observation: s,
            geography: g,
            routes,
            persons,
            fields,
            goods,
            crossings,
            regions,
            lighting: Lighting::at_time(s.now),
            effects,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world_of_nodes::provisioning::playable::{ScenarioKind, Session};
    #[test]
    fn movement_effect_requires_known_travel_not_a_body_left_on_an_edge() {
        let mut s = Session::new(0, ScenarioKind::Local)
            .unwrap()
            .snapshot(ViewMode::Play);
        let g = Geography::default();
        s.persons.truncate(1);
        s.persons[0].location = Location::Edge {
            edge: 2,
            from: 1,
            to: 3,
            progress: 0,
        };
        s.persons[0].action = Some("travelling".into());
        assert_eq!(RenderSnapshot::project(&s, None, 1., &g).effects.len(), 1);
        s.persons[0].action = Some("dead".into());
        assert!(RenderSnapshot::project(&s, None, 1., &g).effects.is_empty());
        s.persons[0].action = Some("travelling".into());
        s.persons[0].evidence.source = KnowledgeSource::Memory;
        assert!(RenderSnapshot::project(&s, None, 1., &g).effects.is_empty());
    }
    #[test]
    fn projection_is_pure_and_never_fills_remote_contents() {
        let session = Session::new(0, ScenarioKind::Local).unwrap();
        let state = (session.world().state_hash(), session.world().event_hash());
        let s = session.snapshot(ViewMode::Play);
        let g = Geography::default();
        let p = RenderSnapshot::project(&s, None, 1., &g);
        assert_eq!(p.persons.len(), s.persons.len());
        assert_eq!(p.fields.len(), s.fields.len());
        assert!(!p.regions.iter().any(|r| r.kind == RegionKind::Water));
        assert!(
            p.regions
                .iter()
                .filter(|r| r.kind == RegionKind::Settlement)
                .all(|r| s
                    .nodes
                    .iter()
                    .any(|n| n.id == r.source_id && n.contents_known))
        );
        assert!(p.lighting.local.is_empty());
        assert_eq!(
            state,
            (session.world().state_hash(), session.world().event_hash())
        );
    }
    #[test]
    fn curved_motion_stays_on_route_and_memory_does_not_move() {
        let s = Session::new(0, ScenarioKind::Local)
            .unwrap()
            .snapshot(ViewMode::Play);
        let g = Geography::default();
        let mut a = s.persons[0].clone();
        a.traversal_nominal_ms = Some(1000);
        a.location = Location::Edge {
            edge: 2,
            from: 1,
            to: 3,
            progress: 0,
        };
        let mut b = a.clone();
        b.location = Location::Edge {
            edge: 2,
            from: 1,
            to: 3,
            progress: 1000 * SCALE,
        };
        let end = g.route(1, 3, 0.).point(1, 1.);
        for i in 0..=60 {
            let t = i as f32 / 60.;
            assert_eq!(
                sample_person(&b, Some(&a), end, None, &g, t),
                g.route(1, 3, 0.).point(1, t)
            );
        }
        b.evidence.source = KnowledgeSource::Memory;
        assert_eq!(sample_person(&b, Some(&a), end, None, &g, 0.), end);
    }
    #[test]
    fn node_edge_boundaries_are_continuous_and_never_extrapolate() {
        let s = Session::new(0, ScenarioKind::Local)
            .unwrap()
            .snapshot(ViewMode::Play);
        let g = Geography::default();
        let mut node = s.persons[0].clone();
        node.location = Location::Node(1);
        let packed = g.node(1).offset(-40., 30.);
        let mut edge = node.clone();
        edge.traversal_nominal_ms = Some(1000);
        edge.location = Location::Edge {
            edge: 2,
            from: 1,
            to: 3,
            progress: 500 * SCALE,
        };
        let middle = g.route(1, 3, 0.).point(1, 0.5);
        assert_eq!(
            sample_person(&edge, Some(&node), middle, Some(packed), &g, 0.),
            packed
        );
        assert_eq!(
            sample_person(&edge, Some(&node), middle, Some(packed), &g, 1.),
            middle
        );
        node.location = Location::Node(3);
        let arrived = g.node(3).offset(10., 20.);
        assert_eq!(
            sample_person(&node, Some(&edge), arrived, Some(middle), &g, 0.),
            middle
        );
        assert_eq!(
            sample_person(&node, Some(&edge), arrived, Some(middle), &g, 2.),
            arrived
        );
        assert_eq!(
            sample_person(&node, Some(&edge), arrived, Some(middle), &g, f32::NAN),
            arrived
        );
    }
    #[test]
    fn changing_projection_or_prior_debug_cannot_change_known_world() {
        let session = Session::new(0, ScenarioKind::Local).unwrap();
        let play = session.snapshot(ViewMode::Play);
        let mut debug = session.snapshot(ViewMode::Debug);
        debug.persons[0].location = Location::Node(4);
        let mut g = Geography::default();
        g.nodes.insert(1, Point::new(10000., -700.));
        g.route_bends.insert((1, 3), 90.);
        let a = RenderSnapshot::project(&play, None, 1., &g);
        let b = RenderSnapshot::project(&play, Some(&debug), 0., &g);
        assert_eq!(a.persons, b.persons);
        assert_eq!(a.routes.len(), play.edges.len());
        assert_eq!(a.observation, &play);
        assert_eq!(session.snapshot(ViewMode::Play), play);
        assert_eq!(a.routes[&2].point(1, 0.), g.node(1));
    }
    #[test]
    fn coordinate_and_clock_do_not_depend_on_view_order_or_wall_time() {
        let g = Geography::default();
        for (a, b) in [(0, 1), (1, 3), (3, 2)] {
            let r = g.route(a, b, 7.);
            for i in 0..=20 {
                let t = i as f32 / 20.;
                let p = r.point(a, t);
                let q = r.point(b, 1. - t);
                assert!((p.x - q.x).abs() < 0.001 && (p.y - q.y).abs() < 0.001);
            }
        }
        assert_eq!(
            Lighting::at_time(0).ambient,
            Lighting::at_time(24 * HOUR).ambient
        );
        assert_eq!(Lod::at_zoom(0.3), Lod::Far);
        assert_eq!(Lod::at_zoom(0.7), Lod::Medium);
        assert_eq!(Lod::at_zoom(1.1), Lod::Close);
    }
}
