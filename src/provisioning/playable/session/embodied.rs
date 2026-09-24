//! E0 command/pose adapter, not new world physics. Local pose refines Node(n)
//! within a bounded cell; inter-node movement uses ordinary FIFO traversal.
use super::*;
use crate::provisioning::playable::{Snapshot, ViewMode};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Input {
    Walk { x: i32, z: i32 },
    Travel { target: usize },
    Institutional(PlayerCommand),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub at: Time,
    pub input: Input,
    pub result: Result<String, String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pose {
    pub node: usize,
    pub x_mm: i32,
    pub z_mm: i32,
}
/// E0 embedding metadata, shared with its renderer; millimetres in world space.
pub const LOCAL_CENTERS_MM: [(i32, i32); 5] = [
    (-40000, 22000),
    (0, 0),
    (74000, 0),
    (42000, -40000),
    (105000, -27500),
];
pub struct EmbodiedSession {
    session: Session,
    pose: Pose,
    last_walk: Time,
    log: Vec<Record>,
}
impl EmbodiedSession {
    pub fn new(seed: u64) -> Result<Self, String> {
        let world = scenario_world_population(seed, ScenarioKind::Local, 40)?;
        let mut session = Session::from_world(seed, ScenarioKind::Local, world)?;
        session.dispatch(PlayerCommand::Commission {
            office: 0,
            project: 0,
        })?;
        session.advance_to(2 * MINUTE)?;
        // Controlled willing contributors supply their real initial holdings.
        // This is fixture prehistory, not an autonomous cooperation claim.
        for donor in 19..24 {
            let batch = session
                .world
                .goods
                .batches
                .iter()
                .find(|b| {
                    b.owner == Owner::Agent(donor)
                        && b.holder == Holder::Agent(donor)
                        && b.material == Material::Wood
                        && b.quantity >= 4
                })
                .map(|b| b.id);
            if let Some(batch) = batch {
                session.world.donate_crossing(donor, 0, batch, 4)?;
            }
        }
        session.world.begin_crossing(19, 0)?;
        session.advance_to(14 * HOUR)?;
        // Labelled physical prehistory: owner uses its constructed capital, never a free cart.
        if let Some(cart) = session
            .world
            .carts()
            .iter()
            .find(|c| c.owner == Owner::Agent(18) && c.location == Location::Node(1))
            .map(|c| c.id)
        {
            session
                .world
                .apply_intents(vec![(18, Intent::HitchCart { cart })])?;
            session.world.apply_intents(vec![(
                18,
                Intent::Travel {
                    edge: 1,
                    target: 2,
                    food: true,
                },
            )])?;
        }
        // Existing F12 controlled delivery purpose; destination is an agreed
        // rendezvous, not a query of the recipient's present position.
        let report = session.world.capture_report(ORDER_COURIER)?;
        session
            .world
            .begin_report_delivery(ORDER_COURIER, report, 12, 2)?;
        let node = match session.world.agents[RULER].location {
            Location::Node(n) => n,
            _ => return Err("E0 prehistory ruler must be at a settlement".into()),
        };
        let now = session.now();
        Ok(Self {
            session,
            pose: Pose {
                node,
                x_mm: 0,
                z_mm: 0,
            },
            last_walk: now,
            log: vec![],
        })
    }
    pub fn now(&self) -> Time {
        self.session.now()
    }
    pub fn seed(&self) -> u64 {
        self.session.seed()
    }
    pub fn world(&self) -> &World {
        self.session.world()
    }
    pub fn snapshot(&self) -> Snapshot {
        self.session.snapshot(ViewMode::Play)
    }
    pub fn pose(&self) -> Pose {
        self.pose
    }
    pub fn log(&self) -> &[Record] {
        &self.log
    }
    pub fn advance_to(&mut self, t: Time) -> Result<(), String> {
        self.session.advance_to(t)?;
        if let Location::Node(n) = self.world().agents[RULER].location {
            if n != self.pose.node {
                self.pose = Pose {
                    node: n,
                    x_mm: 0,
                    z_mm: 0,
                };
                self.last_walk = t;
            }
        } else {
            self.pose.x_mm = 0;
            self.pose.z_mm = 0;
            self.last_walk = t;
        }
        Ok(())
    }
    pub fn dispatch(&mut self, input: Input) -> Result<String, String> {
        let result = self.execute(&input);
        self.log.push(Record {
            at: self.now(),
            input,
            result: result.clone(),
        });
        result
    }
    fn execute(&mut self, input: &Input) -> Result<String, String> {
        if let Input::Institutional(command) = input {
            return self
                .session
                .dispatch(command.clone())
                .map(|r| format!("{r:?}"));
        }
        let p = &self.world().agents[RULER];
        if p.action == Action::Dead || self.world().body_factor(RULER) == 0 {
            return Err("Ruler is dead or incapacitated; no replacement avatar".into());
        }
        let Location::Node(node) = p.location else {
            return Err(
                "Already travelling: atomic edge traversal cannot be steered or cancelled".into(),
            );
        };
        if !matches!(p.action, Action::Idle | Action::MilitaryDuty { .. }) {
            return Err("Ruler is busy, queued, resting or meeting a survival need".into());
        }
        match *input {
            Input::Walk { x, z } => {
                if !(-1000..=1000).contains(&x) || !(-1000..=1000).contains(&z) {
                    return Err("Invalid local direction".into());
                }
                let elapsed = (self.now() - self.last_walk).clamp(0, 1000);
                self.last_walk = self.now();
                if elapsed == 0 {
                    return Err("Local walking requires elapsed authoritative time".into());
                }
                // 0.18 m/s simulation time: 1.8m/wall second at E0's labelled 10x clock.
                let len = ((x as f64).hypot(z as f64)).max(1000.);
                let dx = (x as f64 / len * elapsed as f64 * 0.18).round() as i32;
                let dz = (z as f64 / len * elapsed as f64 * 0.18).round() as i32;
                let (nx, nz) = (self.pose.x_mm + dx, self.pose.z_mm + dz);
                let grove = self.snapshot().goods.iter().any(|g| {
                    g.holder == Holder::Deposit(node)
                        && g.material.contains("Wood")
                        && g.quantity > 0
                });
                let in_grove = grove
                    && (i64::from(nx) - 12600).pow(2) + (i64::from(nz) - 6200).pow(2)
                        < 6000_i64.pow(2);
                if !walkable(node, nx, nz) || in_grove {
                    return Err(
                        "Local boundary or building: use a known route to leave this place".into(),
                    );
                }
                self.pose = Pose {
                    node,
                    x_mm: nx,
                    z_mm: nz,
                };
                Ok("Local walk within current authoritative node".into())
            }
            Input::Travel { target } => {
                if self.pose.x_mm.abs() > 2500 || self.pose.z_mm.abs() > 2500 {
                    return Err(
                        "Return to the road meeting point before inter-location travel".into(),
                    );
                }
                let view = self.snapshot();
                let edge = view
                    .edges
                    .iter()
                    .find(|e| (e.a == node && e.b == target) || (e.b == node && e.a == target))
                    .ok_or("No locally connected known route")?;
                if edge.capacity == 0 {
                    return Err("Known crossing/route is closed".into());
                }
                let id = edge.id;
                self.session.world.apply_intents(vec![(
                    RULER,
                    Intent::Travel {
                        edge: id,
                        target,
                        food: false,
                    },
                )])?;
                if !matches!(
                    self.world().agents[RULER].action,
                    Action::Travel | Action::Queued { .. }
                ) {
                    return Err("Physical travel was not admitted".into());
                }
                self.pose.x_mm = 0;
                self.pose.z_mm = 0;
                Ok(format!(
                    "Travel requested to {target}; shared FIFO and travel time apply"
                ))
            }
            Input::Institutional(_) => unreachable!(),
        }
    }
    pub fn replay(seed: u64, log: &[Record], until: Time) -> Result<Self, String> {
        let mut s = Self::new(seed)?;
        for r in log {
            s.advance_to(r.at)?;
            let result = s.dispatch(r.input.clone());
            if result != r.result {
                return Err("E0 input replay diverged".into());
            }
        }
        s.advance_to(until)?;
        Ok(s)
    }
}
/// A local detail cell is not permission to cross node boundaries. These
/// prototype footprints are shared with the 3D embedding; no hidden NPC data.
pub fn walkable(node: usize, x: i32, z: i32) -> bool {
    let Some(&(cx, cz)) = LOCAL_CENTERS_MM.get(node) else {
        return false;
    };
    let (wx, wz) = (i64::from(cx) + i64::from(x), i64::from(cz) + i64::from(z));
    // Conservative river/bank exclusion, including headwaters near node3.
    // Crossing travel happens only through authoritative graph traversal.
    if (31000..=43000).contains(&wx) && wz > -26000 {
        return false;
    }
    let radius2 = i64::from(x) * i64::from(x) + i64::from(z) * i64::from(z);
    radius2 <= 18_000_i64.pow(2)
        && !(node == 1 && (-14000..=-3000).contains(&x) && (-12000..=-4000).contains(&z))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_population_cart_bridge_and_replay() {
        let mut s = EmbodiedSession::new(0).unwrap();
        assert_eq!(s.world().agents().len(), 40);
        assert!(!s.world().carts().is_empty());
        assert!(
            s.world().effective_edge(1).capacity > 0,
            "{:?}",
            s.world().crossing_projects()
        );
        s.advance_to(s.now() + 1000).unwrap();
        s.dispatch(Input::Walk { x: 1000, z: 0 }).unwrap();
        assert_eq!(s.pose.x_mm, 180);
        let r = EmbodiedSession::replay(0, s.log(), s.now()).unwrap();
        assert_eq!(s.pose, r.pose);
        assert_eq!(s.world().state_hash(), r.world().state_hash());
    }
    #[test]
    fn no_time_no_walk_no_remote_or_closed_travel() {
        let mut s = EmbodiedSession::new(0).unwrap();
        assert!(s.dispatch(Input::Walk { x: 1000, z: 0 }).is_err());
        assert!(s.dispatch(Input::Travel { target: 4 }).is_err());
        s.session.world.infrastructure.projects[0].completed_at = None;
        assert!(s.dispatch(Input::Travel { target: 2 }).is_err());
        assert_eq!(s.world().agents[RULER].location, Location::Node(1));
    }
    #[test]
    fn local_cell_blocks_buildings_water_and_teleport() {
        assert!(!walkable(1, -8000, -8000));
        assert!(!walkable(1, 19000, 0));
        assert!(walkable(1, 0, 0));
        assert!(!walkable(3, -7000, 15500)); // headwater inside detour detail cell
        assert!(walkable(3, 0, 0));
        let mut s = EmbodiedSession::new(0).unwrap();
        s.pose.x_mm = 10000;
        assert!(s.dispatch(Input::Travel { target: 2 }).is_err());
    }
    #[test]
    fn personal_route_travel_uses_existing_person_and_replays() {
        let mut s = EmbodiedSession::new(0).unwrap();
        let before = (s.world().state_hash(), s.world().event_hash());
        for _ in 0..20 {
            let _ = s.snapshot();
        }
        assert_eq!(before, (s.world().state_hash(), s.world().event_hash()));
        s.dispatch(Input::Travel { target: 2 }).unwrap();
        assert!(matches!(
            s.world().agents[RULER].action,
            Action::Travel { .. } | Action::Queued { .. }
        ));
        assert_eq!(s.world().agents().len(), 40);
        s.advance_to(s.now() + MINUTE).unwrap();
        assert!(s.dispatch(Input::Walk { x: 1000, z: 0 }).is_err());
        let replay = EmbodiedSession::replay(0, s.log(), s.now()).unwrap();
        assert_eq!(s.world().state_hash(), replay.world().state_hash());
        assert_eq!(s.world().event_hash(), replay.world().event_hash());
    }
    #[test]
    fn mortality_and_time_partition() {
        let mut a = EmbodiedSession::new(0).unwrap();
        let mut b = EmbodiedSession::new(0).unwrap();
        let start = a.now();
        for i in 1..=300 {
            a.advance_to(start + i * 1000).unwrap();
        }
        b.advance_to(start + 5 * MINUTE).unwrap();
        assert_eq!(a.world().state_hash(), b.world().state_hash());
        a.session.world.agents[RULER].action = Action::Dead;
        assert!(a.dispatch(Input::Walk { x: 1000, z: 0 }).is_err());
    }
}
