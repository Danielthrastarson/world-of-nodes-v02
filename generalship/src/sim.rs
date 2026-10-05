//! The only authoritative world. Network and browser code receive `view`, never this state.
use crate::{model::*, terrain::*};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub config: Config,
    pub seed: u64,
    pub terrain: Terrain,
    pub tick: Tick,
    pub status: Status,
    pub people: Vec<Person>,
    pub posts: Vec<Post>,
    pub stock: [Stock; 2],
    pub consumed: [Stock; 2],
    pub initial: [Stock; 2],
    pub knowledge: [Knowledge; 2],
    pub missions: Vec<Mission>,
    pub signals: Vec<Signal>,
    pub sequence: [u64; 2],
    pub ready: [bool; 2],
    pub connected: [bool; 2],
    pub disconnected_at: [Option<Tick>; 2],
    pub capture: [Tick; 2],
    pub bridge_queue: BTreeMap<usize, Tick>,
    #[serde(skip)]
    pub inputs: Vec<InputRecord>,
    #[serde(skip)]
    pub checkpoints: Vec<(Tick, String)>,
}
impl Game {
    pub fn new(seed: u64, config: Config) -> Self {
        let terrain = Terrain::configured(seed, &config);
        let mut people = vec![];
        let mut posts = vec![];
        for side in 0..2 {
            let hq = terrain.headquarters[side];
            let post_id = side * terrain.sites.len() + if side == 0 { 0 } else { 3 };
            // Site-based identities cannot reveal how many structures the opponent has built.
            for &pos in &terrain.sites {
                let id = posts.len();
                let initial = id == post_id;
                posts.push(Post {
                    id,
                    side,
                    pos,
                    hp: if initial { config.post_hp } else { 0 },
                    complete: initial,
                    upgraded: false,
                    upgrading: false,
                    stock: [0; 3],
                    work: if initial { config.build_work } else { 0 },
                    upgrading_work: 0,
                });
            }
            for n in 0..48 {
                let role = match n {
                    0..=1 => Role::General,
                    2..=11 => Role::Line,
                    12..=21 => Role::Ranged,
                    22..=31 => Role::Light,
                    32..=35 => Role::Carrier,
                    36..=39 => Role::Courier,
                    40..=41 => Role::Scout,
                    42..=43 => Role::Staff,
                    44..=45 => Role::Escort,
                    _ => Role::Guard,
                };
                let dir = if side == 0 { 1 } else { -1 };
                let mut pos = Point::new(
                    hq.x + dir * ((n % 6) as i32 * 650 - 1300),
                    hq.y + (n / 6) as i32 * 650 - 1900,
                );
                if (2..32).contains(&n) {
                    let j = n - 2;
                    pos = Point::new(
                        hq.x + dir * (1800 + (j % 5) as i32 * 800),
                        hq.y - 4500 + (j / 10) as i32 * 4500 + ((j % 10) / 5) as i32 * 800,
                    );
                }
                if role == Role::Scout {
                    pos = Point::new(
                        if side == 0 { 30000 } else { 60000 },
                        if n == 40 { 27500 } else { 48000 },
                    );
                }
                if role == Role::Staff {
                    pos = Point::new(
                        posts[post_id].pos.x + (n as i32 - 42) * 600 - 300,
                        posts[post_id].pos.y,
                    );
                }
                if role == Role::Guard {
                    pos = Point::new(hq.x + (n as i32 - 46) * 600, hq.y + 1200);
                }
                let target = if role == Role::Scout {
                    Point::new(
                        if side == 0 { 40000 } else { 50000 },
                        if n == 40 { 27500 } else { 49000 },
                    )
                } else {
                    pos
                };
                let mut plan = Plan::hold(target);
                if role == Role::Scout {
                    plan.verb = Verb::Observe;
                    plan.manner = Manner::Avoid;
                }
                people.push(Person {
                    id: people.len(),
                    side,
                    role,
                    pos,
                    hp: config.person_hp,
                    cargo: [config.starting_food, 0, 0],
                    missed: 0,
                    plan,
                    path: vec![],
                    path_goal: None,
                    sequence: 0,
                    accepted_group: vec![],
                    accepted_strength: 0,
                    next_attack: 0,
                    duty_target: if role == Role::Courier {
                        pos
                    } else {
                        Point::new(if side == 0 { 31000 } else { 59000 }, 28000)
                    },
                    post: if role == Role::Staff {
                        Some(post_id)
                    } else {
                        None
                    },
                    escort: if role == Role::Escort {
                        Some(side * 48 + (n - 44))
                    } else {
                        None
                    },
                    task: None,
                    training: 0,
                    route_index: 0,
                    last_report: 0,
                    report_returning: false,
                    material_quota: 0,
                });
            }
        }
        let stock = [[config.hq_food, config.hq_material, 0]; 2];
        let initial = [[
            config.hq_food + 48 * config.starting_food,
            config.hq_material,
            0,
        ]; 2];
        let mut g = Self {
            config,
            seed,
            terrain,
            tick: 0,
            status: Status::Lobby,
            people,
            posts,
            stock,
            consumed: [[0; 3]; 2],
            initial,
            knowledge: Default::default(),
            missions: vec![],
            signals: vec![],
            sequence: [0; 2],
            ready: [false; 2],
            connected: [false; 2],
            disconnected_at: [None; 2],
            capture: [0; 2],
            bridge_queue: BTreeMap::new(),
            inputs: vec![],
            checkpoints: vec![],
        };
        // Explicit prior briefing of our starting deployment, never enemy deployment.
        for side in 0..2 {
            let reports: Vec<_> = g
                .people
                .iter()
                .filter(|p| p.side == side)
                .map(|p| g.observe(side, p.pos, 0, format!("deployment:{}", p.id), p.id as u64))
                .collect();
            for r in reports {
                g.receive(side, r);
            }
            g.knowledge[side].reports.clear();
            g.notice(side,"Deploy your generals, protect your signal network, and capture the enemy headquarters.");
        }
        g.sense();
        g
    }
    pub fn notice(&mut self, side: usize, text: &str) {
        let k = &mut self.knowledge[side];
        k.log.push(Notice {
            tick: self.tick,
            text: text.into(),
        });
        if k.log.len() > 60 {
            k.log.remove(0);
        }
    }
    pub fn hash(&self) -> String {
        format!("{:x}", Sha256::digest(serde_json::to_vec(self).unwrap()))
    }
    pub fn replay(&self) -> Replay {
        Replay {
            config: self.config.clone(),
            seed: self.seed,
            inputs: self.inputs.clone(),
            end_tick: self.tick,
            hash: self.hash(),
            checkpoints: self.checkpoints.clone(),
        }
    }
    pub fn replay_run(r: &Replay) -> Result<Self, String> {
        let mut g = Self::new(r.seed, r.config.clone());
        let mut index = 0;
        loop {
            while index < r.inputs.len() && r.inputs[index].tick == g.tick {
                let i = &r.inputs[index];
                g.input(i.side, i.input.clone())?;
                index += 1;
            }
            if g.tick >= r.end_tick {
                break;
            }
            if g.status != Status::Running {
                return Err("replay stopped before end tick".into());
            }
            g.step();
        }
        if index != r.inputs.len() {
            return Err("unreplayed input".into());
        }
        if g.checkpoints != r.checkpoints {
            return Err("Replay checkpoint mismatch".into());
        }
        Ok(g)
    }
    fn owned(&self, side: usize, ids: &[usize]) -> bool {
        !ids.is_empty()
            && ids.len() <= 48
            && ids
                .iter()
                .all(|&i| self.people.get(i).is_some_and(|p| p.side == side))
            && ids.iter().copied().collect::<BTreeSet<_>>().len() == ids.len()
    }
    pub fn input(&mut self, side: usize, input: Input) -> Result<(), String> {
        if side > 1 {
            return Err("Invalid seat".into());
        }
        if let Input::Connection { connected } = input {
            self.connected[side] = connected;
            self.disconnected_at[side] = if connected { None } else { Some(self.tick) };
            if self.status == Status::Lobby
                && self.connected == [true; 2]
                && self.ready == [true; 2]
            {
                self.status = Status::Running;
            }
            self.inputs.push(InputRecord {
                tick: self.tick,
                side,
                input,
            });
            return Ok(());
        }
        if matches!(input, Input::Ready) {
            if self.status != Status::Lobby {
                return Err("Match already started".into());
            }
            self.ready[side] = true;
            if self.ready == [true; 2] && self.connected == [true; 2] {
                self.status = Status::Running;
            }
        } else {
            if self.status != Status::Running {
                return Err("Match is not running".into());
            }
            match &input {
                Input::MoveGeneral { id, target } => {
                    if !self.terrain.walkable(*target) {
                        return Err("Impassable destination".into());
                    }
                    let p = self
                        .people
                        .get_mut(*id)
                        .filter(|p| p.side == side && p.role == Role::General && p.alive())
                        .ok_or("General unavailable")?;
                    p.plan = Plan {
                        area_radius: 0,
                        verb: Verb::Move,
                        target: *target,
                        route: vec![],
                        manner: Manner::NoPursuit,
                        then: Some(Verb::Hold),
                        fallback: None,
                    };
                    p.path_goal = None;
                    p.path.clear();
                }
                Input::Order {
                    rendezvous,
                    source,
                    recipients,
                    plan,
                } => {
                    if !plan.valid()
                        || !self.terrain.walkable(plan.target)
                        || plan.route.iter().any(|p| !self.terrain.walkable(*p))
                        || plan.fallback.is_some_and(|p| !self.terrain.walkable(p))
                    {
                        return Err("Invalid plan or target".into());
                    }
                    if !self.owned(side, recipients) {
                        return Err("Invalid recipients".into());
                    }
                    let pos = self.source_pos(side, source)?;
                    let dest = match rendezvous {
                        Some(point) if self.terrain.walkable(*point) => *point,
                        Some(_) => return Err("Impassable courier rendezvous".into()),
                        None => self.known_destination(side, recipients)?,
                    };
                    self.sequence[side] += 1;
                    let seq = self.sequence[side];
                    let id = seq * 2 + side as u64;
                    self.knowledge[side].orders.push(OrderRecord {
                        id,
                        issued: self.tick,
                        recipients: recipients.clone(),
                        plan: plan.clone(),
                        status: "Dispatched; awaiting acknowledgement".into(),
                    });
                    self.dispatch(
                        side,
                        source,
                        pos,
                        dest,
                        recipients.clone(),
                        Payload::Order {
                            id,
                            sequence: seq,
                            recipients: recipients.clone(),
                            plan: plan.clone(),
                        },
                    );
                    self.notice(
                        side,
                        &format!("Order {id} dispatched to the last reported position."),
                    );
                }
                Input::Assign {
                    source,
                    people,
                    role,
                    target,
                    post,
                    escort,
                } => {
                    if !self.owned(side, people)
                        || !matches!(
                            role,
                            Role::Courier
                                | Role::Carrier
                                | Role::Scout
                                | Role::Staff
                                | Role::Escort
                                | Role::Guard
                        )
                        || !self.terrain.walkable(*target)
                    {
                        return Err("Invalid assignment".into());
                    }
                    if *role == Role::Staff && post.is_none() {
                        return Err("Select an outpost for its staff".into());
                    }
                    if *role == Role::Escort && escort.is_none() {
                        return Err("Select a general to escort".into());
                    }
                    if post.is_some_and(|id| {
                        !self.knowledge[side]
                            .posts
                            .get(&id)
                            .is_some_and(|p| p.side == side)
                    }) || escort.is_some_and(|id| {
                        !self
                            .people
                            .get(id)
                            .is_some_and(|p| p.side == side && p.role == Role::General)
                    }) {
                        return Err("Invalid duty destination".into());
                    }
                    let pos = self.source_pos(side, source)?;
                    let dest = self.known_destination(side, people)?;
                    self.sequence[side] += 1;
                    let seq = self.sequence[side];
                    let id = seq * 2 + side as u64;
                    self.dispatch(
                        side,
                        source,
                        pos,
                        dest,
                        people.clone(),
                        Payload::Assign {
                            id,
                            sequence: seq,
                            people: people.clone(),
                            role: *role,
                            target: *target,
                            post: *post,
                            escort: *escort,
                        },
                    );
                    self.notice(
                        side,
                        &format!("Duty assignment {id} dispatched; existing work ends on receipt."),
                    );
                }
                Input::Build { site, people } => {
                    let pos = *self
                        .terrain
                        .sites
                        .get(*site)
                        .ok_or("Unknown construction site")?;
                    if self.knowledge[side]
                        .posts
                        .values()
                        .any(|p| p.pos == pos && p.hp > 0)
                    {
                        return Err("Site already has a known structure".into());
                    }
                    self.validate_workers(side, people)?;
                    if self.stock[side][1] < self.config.build_material {
                        return Err("Not enough headquarters materials".into());
                    }
                    // Materials remain physical HQ stock until workers pick them up.
                    let id = side * self.terrain.sites.len() + site;
                    if self.people.iter().any(|p| {
                        p.side == side && p.alive() && p.post == Some(id) && p.role == Role::Builder
                    }) {
                        return Err("Construction has already been assigned at this site".into());
                    }
                    let old_stock = self.posts[id].stock;
                    self.posts[id] = Post {
                        id,
                        side,
                        pos,
                        hp: self.config.post_hp,
                        complete: false,
                        upgraded: false,
                        upgrading: false,
                        stock: old_stock,
                        work: 0,
                        upgrading_work: 0,
                    };
                    self.set_builders(people, id);
                    self.notice(
                        side,
                        "Outpost work assigned. Workers must haul materials and build on site.",
                    );
                }
                Input::Upgrade { post, people } => {
                    let known = self.knowledge[side]
                        .posts
                        .get(post)
                        .filter(|p| p.side == side && p.complete && !p.upgraded)
                        .ok_or("No known eligible outpost")?;
                    let _ = known;
                    self.validate_workers(side, people)?;
                    if self.stock[side][1] < self.config.upgrade_material {
                        return Err("Not enough headquarters materials".into());
                    }
                    // An unseen ruined post may receive a futile job; no hidden-state rejection.
                    self.posts[*post].upgrading = true;
                    self.set_builders(people, *post);
                    self.notice(
                        side,
                        "Relay upgrade assigned. Workers will deliver its materials.",
                    );
                }
                Input::Train { person } => {
                    if self
                        .people
                        .iter()
                        .filter(|p| {
                            p.side == side
                                && p.alive()
                                && matches!(p.role, Role::General | Role::Trainee)
                        })
                        .count()
                        >= 2
                    {
                        return Err("Two generals are already active or training".into());
                    }
                    self.validate_workers(side, &[*person])?;
                    if self.stock[side][0] < self.config.training_food {
                        return Err("Not enough food for training".into());
                    }
                    self.stock[side][0] -= self.config.training_food;
                    self.consumed[side][0] += self.config.training_food;
                    let p = &mut self.people[*person];
                    p.role = Role::Trainee;
                    p.training = self.config.training_ticks;
                    p.path.clear();
                    p.path_goal = None;
                    p.task = None;
                    p.post = None;
                    p.escort = None;
                    self.notice(
                        side,
                        "An existing headquarters resident has begun general training.",
                    );
                }
                Input::Surrender => {
                    self.status = Status::Finished {
                        winner: Some(1 - side),
                        reason: "surrender".into(),
                    }
                }
                _ => {}
            }
        }
        self.inputs.push(InputRecord {
            tick: self.tick,
            side,
            input,
        });
        Ok(())
    }
    fn validate_workers(&self, side: usize, ids: &[usize]) -> Result<(), String> {
        if !self.owned(side, ids)
            || ids.iter().any(|&i| {
                let p = &self.people[i];
                !p.alive()
                    || !p.role.assignable()
                    || p.task.is_some()
                    || !p
                        .pos
                        .near(self.terrain.headquarters[side], self.config.exchange_radius)
            })
        {
            return Err("Select available people physically at headquarters".into());
        }
        Ok(())
    }
    fn set_builders(&mut self, ids: &[usize], post: usize) {
        let cost = if self.posts[post].complete {
            self.config.upgrade_material
        } else {
            self.config.build_material
        };
        for (n, &i) in ids.iter().enumerate() {
            let p = &mut self.people[i];
            p.role = Role::Builder;
            p.post = Some(post);
            p.task = None;
            p.escort = None;
            p.path_goal = None;
            p.path.clear();
            p.plan = Plan::hold(p.pos);
            p.material_quota =
                cost / ids.len() as u64 + u64::from((n as u64) < cost % ids.len() as u64);
        }
    }
    pub fn step(&mut self) {
        if self.status != Status::Running {
            return;
        }
        self.tick += 1;
        self.meals();
        self.deliver_signals();
        self.communications();
        self.work_and_move();
        self.combat();
        self.sense();
        self.victory();
        if self.tick.is_multiple_of(100) {
            self.checkpoints.push((self.tick, self.hash()));
        }
        debug_assert!(self.audit().is_ok());
    }
    pub fn audit(&self) -> Result<(), String> {
        if self.people.len() != 96 {
            return Err("People were created or erased".into());
        }
        for side in 0..2 {
            let mut total = self.stock[side];
            for (r, value) in total.iter_mut().enumerate() {
                *value += self.consumed[side][r];
            }
            for p in self.people.iter().filter(|p| p.side == side) {
                for (r, value) in total.iter_mut().enumerate() {
                    *value += p.cargo[r];
                }
                if p.cargo.iter().sum::<u64>() > self.config.carrier_capacity {
                    return Err("Carrying capacity exceeded".into());
                }
                if p.task.is_some() && p.role != Role::Courier {
                    return Err("Concurrent courier and other work".into());
                }
            }
            for p in self.posts.iter().filter(|p| p.side == side) {
                for (r, value) in total.iter_mut().enumerate() {
                    *value += p.stock[r];
                }
            }
            if total != self.initial[side] {
                return Err(format!(
                    "Stock balance changed: side {side}, {total:?} != {:?}",
                    self.initial[side]
                ));
            }
        }
        Ok(())
    }
    fn meals(&mut self) {
        if !self.tick.is_multiple_of(self.config.meal_ticks) {
            return;
        }
        for p in &mut self.people {
            if !p.alive() {
                continue;
            }
            if p.cargo[0] > 0 {
                p.cargo[0] -= 1;
                self.consumed[p.side][0] += 1;
                p.missed = p.missed.saturating_sub(1);
            } else if p.pos.near(
                self.terrain.headquarters[p.side],
                self.config.exchange_radius,
            ) && self.stock[p.side][0] > 0
            {
                self.stock[p.side][0] -= 1;
                self.consumed[p.side][0] += 1;
                p.missed = p.missed.saturating_sub(1);
            } else {
                p.missed += 1;
            }
        }
    }
}
