//! The only authoritative world. Network and browser code receive `view`, never this state.
use crate::{model::*, terrain::*};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
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
    fn source_pos(&self, side: usize, s: &Source) -> Result<Point, String> {
        match s {
            Source::Hq => Ok(self.terrain.headquarters[side]),
            Source::General { id } => self
                .people
                .get(*id)
                .filter(|p| p.side == side && p.alive() && p.role == Role::General)
                .map(|p| p.pos)
                .ok_or("General unavailable".into()),
            Source::Outpost { id } => self.knowledge[side]
                .posts
                .get(id)
                .filter(|p| p.side == side && p.complete)
                .map(|p| p.pos)
                .ok_or("No known friendly outpost".into()),
        }
    }
    fn known_destination(&self, side: usize, ids: &[usize]) -> Result<Point, String> {
        let points: Vec<_> = ids
            .iter()
            .filter_map(|i| self.knowledge[side].people.get(i))
            .map(|p| p.pos)
            .collect();
        if points.len() != ids.len() {
            return Err("Recipient has no known position".into());
        }
        Ok(Point::new(
            points.iter().map(|p| p.x as i64).sum::<i64>() as i32 / points.len() as i32,
            points.iter().map(|p| p.y as i64).sum::<i64>() as i32 / points.len() as i32,
        ))
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
                    if self
                        .people
                        .iter()
                        .any(|p| p.side == side && p.post == Some(id) && p.role == Role::Builder)
                    {
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
    fn dispatch(
        &mut self,
        side: usize,
        source: &Source,
        pos: Point,
        destination: Point,
        recipients: Vec<usize>,
        payload: Payload,
    ) {
        let mut path = vec![];
        let mut due = self.tick + self.config.local_exchange_ticks;
        if let Source::Outpost { id } = source {
            let relay = self.relay(side);
            if let Some((delay, p)) = relay.get(id) {
                due += delay;
                path = p.clone();
            } else {
                due = u64::MAX;
                path = vec![*id];
            }
        }
        let id = self.missions.len();
        self.missions.push(Mission {
            id,
            side,
            source: pos,
            destination,
            payload,
            courier: None,
            due,
            wait_since: None,
            returning: false,
            finished: false,
            relay_path: path,
            recipients,
        });
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
    fn staff(&self, id: usize) -> usize {
        let post = &self.posts[id];
        self.people
            .iter()
            .filter(|p| {
                p.alive()
                    && p.missed < self.config.staff_missed_meals
                    && p.side == post.side
                    && p.role == Role::Staff
                    && p.post == Some(id)
                    && p.pos.near(post.pos, self.config.logistics_radius)
            })
            .count()
    }
    pub fn relay(&self, side: usize) -> BTreeMap<usize, (Tick, Vec<usize>)> {
        let roots: Vec<_> = std::iter::once(self.terrain.headquarters[side])
            .chain(
                self.people
                    .iter()
                    .filter(|p| p.side == side && p.alive() && p.role == Role::General)
                    .map(|p| p.pos),
            )
            .collect();
        let eligible: Vec<_> = self
            .posts
            .iter()
            .filter(|p| {
                p.side == side
                    && p.hp > 0
                    && p.complete
                    && self.staff(p.id) >= self.config.required_staff
            })
            .map(|p| p.id)
            .collect();
        let mut found: BTreeMap<usize, (Tick, Vec<usize>)> = BTreeMap::new();
        for &id in &eligible {
            let p = &self.posts[id];
            let range = if p.upgraded {
                self.config.upgraded_range
            } else {
                self.config.relay_range
            };
            let delay = if p.upgraded {
                self.config.upgraded_ticks
            } else {
                self.config.relay_ticks
            };
            if roots
                .iter()
                .any(|&r| r.near(p.pos, range) && self.terrain.sight(r, p.pos))
            {
                found.insert(id, (delay, vec![id]));
            }
        }
        for _ in 0..eligible.len() {
            let old = found.clone();
            for &id in &eligible {
                let p = &self.posts[id];
                for (&other, (cost, path)) in &old {
                    if path.contains(&id) {
                        continue;
                    }
                    let q = &self.posts[other];
                    let upgraded = p.upgraded && q.upgraded;
                    let range = if upgraded {
                        self.config.upgraded_range
                    } else {
                        self.config.relay_range
                    };
                    let delay = if upgraded {
                        self.config.upgraded_ticks
                    } else {
                        self.config.relay_ticks
                    };
                    if p.pos.near(q.pos, range)
                        && self.terrain.sight(p.pos, q.pos)
                        && found.get(&id).is_none_or(|v| v.0 > cost + delay)
                    {
                        let mut pp = path.clone();
                        pp.push(id);
                        found.insert(id, (cost + delay, pp));
                    }
                }
            }
            if found == old {
                break;
            }
        }
        found
    }
    fn signal_valid(&self, side: usize, path: &[usize]) -> bool {
        path.last()
            .is_some_and(|id| self.relay(side).get(id).is_some_and(|v| v.1 == path))
    }
    fn observe(&self, side: usize, pos: Point, radius: i32, source: String, _key: u64) -> Report {
        let people = self
            .people
            .iter()
            .filter(|p| pos.near(p.pos, radius) && self.terrain.sight(pos, p.pos))
            .map(|p| Contact {
                absent_since: None,
                id: p.id,
                side: p.side,
                role: p.role,
                pos: p.pos,
                hp: p.hp,
                cargo: if p.side == side { Some(p.cargo) } else { None },
                received_order: if p.side == side {
                    Some(p.sequence)
                } else {
                    None
                },
                observed: self.tick,
                received: self.tick,
                source: source.clone(),
            })
            .collect();
        let posts = self
            .posts
            .iter()
            .filter(|p| {
                (p.complete || p.work > 0 || p.stock[1] > 0)
                    && pos.near(p.pos, radius)
                    && self.terrain.sight(pos, p.pos)
            })
            .map(|p| PostContact {
                id: p.id,
                side: p.side,
                pos: p.pos,
                hp: p.hp,
                complete: p.complete,
                upgraded: p.upgraded,
                staff: self.staff(p.id),
                stock: if p.side == side { Some(p.stock) } else { None },
                work: if p.side == side { Some(p.work) } else { None },
                upgrading_work: if p.side == side {
                    Some(p.upgrading_work)
                } else {
                    None
                },
                observed: self.tick,
                received: self.tick,
                source: source.clone(),
            })
            .collect();
        Report {
            id: format!("{side}:{}:{source}", self.tick),
            observed: self.tick,
            received: self.tick,
            source,
            pos,
            radius,
            people,
            posts,
        }
    }
    pub fn receive(&mut self, side: usize, mut report: Report) {
        report.received = self.tick;
        let k = &mut self.knowledge[side];
        let seen: BTreeSet<_> = report.people.iter().map(|p| p.id).collect();
        for old in k.people.values_mut() {
            if old.observed < report.observed
                && !seen.contains(&old.id)
                && old.pos.near(report.pos, report.radius)
                && self.terrain.sight(report.pos, old.pos)
            {
                old.absent_since = Some(report.observed);
            }
        }
        for p in &mut report.people {
            p.received = self.tick;
            if k.people
                .get(&p.id)
                .is_none_or(|old| old.observed <= p.observed)
            {
                k.people.insert(p.id, p.clone());
            }
        }
        for p in &mut report.posts {
            p.received = self.tick;
            if k.posts
                .get(&p.id)
                .is_none_or(|old| old.observed <= p.observed)
            {
                k.posts.insert(p.id, p.clone());
            }
        }
        if k.reports
            .iter()
            .any(|r| r.source == report.source && r.observed > report.observed)
        {
            return;
        }
        k.reports.retain(|r| r.source != report.source);
        k.reports.push(report);
        if k.reports.len() > 40 {
            k.reports.remove(0);
        }
    }
    fn sense(&mut self) {
        for side in 0..2 {
            let mut direct = vec![self.observe(
                side,
                self.terrain.headquarters[side],
                self.config.general_sight,
                "headquarters".into(),
                0,
            )];
            for p in self
                .people
                .iter()
                .filter(|p| p.side == side && p.role == Role::General && p.alive())
            {
                direct.push(self.observe(
                    side,
                    p.pos,
                    self.config.general_sight,
                    format!("general:{}", p.id),
                    p.id as u64 + 1,
                ));
            }
            for r in direct {
                self.receive(side, r);
            }
            if self.tick.is_multiple_of(10) {
                for (id, (delay, path)) in self.relay(side) {
                    let post = &self.posts[id];
                    let report = self.observe(
                        side,
                        post.pos,
                        self.config.outpost_sight,
                        format!("outpost:{id}"),
                        150 + id as u64,
                    );
                    self.signals.push(Signal {
                        side,
                        path,
                        due: self.tick + delay,
                        reports: vec![report],
                        ack: None,
                    });
                }
            }
        }
    }
    fn deliver_signals(&mut self) {
        let signals = std::mem::take(&mut self.signals);
        for s in signals {
            if !self.signal_valid(s.side, &s.path) {
                continue;
            }
            if s.due <= self.tick {
                for r in s.reports {
                    self.receive(s.side, r);
                }
                if let Some((id, status)) = s.ack {
                    self.acknowledge(s.side, id, &status);
                }
            } else {
                self.signals.push(s);
            }
        }
    }
    pub fn view(&self, side: usize) -> Value {
        let k = &self.knowledge[side];
        // No authoritative troop, mission, relay or opponent counters cross this boundary.
        json!({"tick":self.tick,"side":side,"status":self.status,"ready":self.ready,"connected":self.connected,
            "terrain":self.terrain,"config":self.config,"stock":self.stock[side],"people":k.people.values().collect::<Vec<_>>(),
            "posts":k.posts.values().collect::<Vec<_>>(),"reports":k.reports.iter().map(|r|json!({"id":r.id,"observed":r.observed,"received":r.received,"source":r.source,"pos":r.pos,"radius":r.radius,"people":r.people.iter().map(|p|json!({"id":p.id,"side":p.side,"hp":p.hp})).collect::<Vec<_>>()})).collect::<Vec<_>>(),"log":k.log,"orders":k.orders.iter().rev().take(40).collect::<Vec<_>>()})
    }
}

impl Game {
    fn acknowledge(&mut self, side: usize, id: u64, status: &str) {
        if let Some(o) = self.knowledge[side].orders.iter_mut().find(|o| o.id == id) {
            o.status = status.into();
        }
        self.notice(side, &format!("Order {id}: {status}"));
    }
    fn deliver_to_command(&mut self, side: usize, pos: Point, payload: &Payload) -> bool {
        let direct = pos.near(self.terrain.headquarters[side], self.config.exchange_radius)
            || self.people.iter().any(|p| {
                p.side == side
                    && p.role == Role::General
                    && p.alive()
                    && p.pos.near(pos, self.config.exchange_radius)
            });
        let (reports, ack) = match payload {
            Payload::Reports { reports } => (reports.clone(), None),
            Payload::Ack {
                id,
                status,
                reports,
            } => (reports.clone(), Some((*id, status.clone()))),
            _ => return false,
        };
        if direct {
            for r in reports {
                self.receive(side, r);
            }
            if let Some((id, status)) = ack {
                self.acknowledge(side, id, &status);
            }
            return true;
        }
        for (id, (delay, path)) in self.relay(side) {
            if self.posts[id].pos.near(pos, self.config.exchange_radius) {
                self.signals.push(Signal {
                    side,
                    path,
                    due: self.tick + delay,
                    reports,
                    ack,
                });
                return true;
            }
        }
        false
    }
    fn apply_payload(
        &mut self,
        side: usize,
        pos: Point,
        payload: &Payload,
    ) -> Option<(u64, String)> {
        match payload {
            Payload::Order {
                id,
                sequence,
                recipients,
                plan,
            } => {
                let present: Vec<_> = recipients
                    .iter()
                    .copied()
                    .filter(|&i| {
                        self.people[i].alive()
                            && self.people[i].pos.near(pos, self.config.exchange_radius)
                    })
                    .collect();
                if present.is_empty() {
                    return None;
                }
                let mut received = 0;
                for &i in &present {
                    let p = &mut self.people[i];
                    if p.sequence >= *sequence
                        || matches!(p.role, Role::General | Role::Trainee)
                        || p.task.is_some()
                    {
                        continue;
                    }
                    p.sequence = *sequence;
                    p.plan = plan.clone();
                    p.route_index = 0;
                    p.path_goal = None;
                    p.path.clear();
                    p.accepted_group = present.clone();
                    p.accepted_strength = present.len();
                    received += 1;
                }
                Some((
                    *id,
                    format!(
                        "Acknowledged locally by {received} people; execution is not confirmed complete"
                    ),
                ))
            }
            Payload::Assign {
                id,
                sequence,
                people,
                role,
                target,
                post,
                escort,
            } => {
                let present: Vec<_> = people
                    .iter()
                    .copied()
                    .filter(|&i| {
                        self.people[i].alive()
                            && self.people[i].pos.near(pos, self.config.exchange_radius)
                    })
                    .collect();
                if present.is_empty() {
                    return None;
                }
                let mut count = 0;
                for i in present {
                    let p = &mut self.people[i];
                    if p.sequence >= *sequence || !p.role.assignable() || p.task.is_some() {
                        continue;
                    }
                    p.sequence = *sequence;
                    p.role = *role;
                    p.duty_target = *target;
                    p.post = *post;
                    p.escort = *escort;
                    p.path_goal = None;
                    p.path.clear();
                    p.plan = Plan::hold(*target);
                    p.material_quota = 0;
                    p.report_returning = false;
                    if *role == Role::Scout {
                        p.plan.verb = Verb::Observe;
                        p.plan.manner = Manner::Avoid;
                    }
                    count += 1;
                }
                Some((*id, format!("Duty received by {count} people")))
            }
            _ => {
                let _ = side;
                None
            }
        }
    }
    fn communications(&mut self) {
        for idx in 0..self.missions.len() {
            if self.missions[idx].finished {
                continue;
            }
            let mut m = self.missions[idx].clone();
            if !m.relay_path.is_empty() {
                let endpoint = *m.relay_path.last().unwrap();
                let routes = self.relay(m.side);
                if !self.signal_valid(m.side, &m.relay_path) {
                    m.due = u64::MAX;
                }
                if m.due == u64::MAX {
                    if let Some((delay, path)) = routes.get(&endpoint) {
                        m.relay_path = path.clone();
                        m.due = self.tick + delay;
                    }
                    self.missions[idx] = m;
                    continue;
                }
                if self.tick < m.due {
                    continue;
                }
                m.relay_path.clear();
            }
            if self.tick < m.due {
                continue;
            }
            if let Some(id) = m.courier {
                if !self.people[id].alive() {
                    m.finished = true;
                    self.missions[idx] = m;
                    continue;
                }
                if !self.people[id]
                    .pos
                    .near(m.destination, self.config.courier_arrival_radius)
                {
                    self.missions[idx] = m;
                    continue;
                }
            } else if !m.source.near(m.destination, self.config.exchange_radius) {
                if let Some(p) = self.people.iter_mut().find(|p| {
                    p.side == m.side
                        && p.alive()
                        && p.role == Role::Courier
                        && p.task.is_none()
                        && p.pos.near(m.source, self.config.exchange_radius)
                }) {
                    p.task = Some(idx);
                    p.path_goal = None;
                    m.courier = Some(p.id);
                }
                self.missions[idx] = m;
                continue;
            }
            let at = m.courier.map(|id| self.people[id].pos).unwrap_or(m.source);
            if m.returning || matches!(m.payload, Payload::Reports { .. }) {
                if self.deliver_to_command(m.side, at, &m.payload) {
                    m.finished = true;
                } else if m.courier.is_some() {
                    m.destination = self.terrain.headquarters[m.side];
                }
            } else {
                let result = self.apply_payload(m.side, at, &m.payload);
                let timed_out = m
                    .wait_since
                    .is_some_and(|t| self.tick - t >= self.config.courier_wait);
                if let Some((id, status)) = result {
                    let report = self.observe(
                        m.side,
                        at,
                        self.config.sight,
                        format!("courier:{}", m.courier.unwrap_or(999)),
                        250 + (id / 2 % 200),
                    );
                    m.payload = Payload::Ack {
                        id,
                        status,
                        reports: vec![report],
                    };
                    m.returning = true;
                    m.wait_since = None;
                    if m.courier.is_some() {
                        m.destination = m.source;
                    } else if self.deliver_to_command(m.side, at, &m.payload) {
                        m.finished = true;
                    } else {
                        m.destination = self.terrain.headquarters[m.side];
                    }
                } else if timed_out {
                    let id = match m.payload {
                        Payload::Order { id, .. } | Payload::Assign { id, .. } => id,
                        _ => 0,
                    };
                    m.payload = Payload::Ack {
                        id,
                        status: "Returned undelivered: recipient not found at rendezvous".into(),
                        reports: vec![],
                    };
                    m.returning = true;
                    m.wait_since = None;
                    if m.courier.is_some() {
                        m.destination = m.source;
                    } else {
                        self.deliver_to_command(m.side, at, &m.payload);
                        m.finished = true;
                    }
                } else if m.wait_since.is_none() {
                    m.wait_since = Some(self.tick);
                }
            }
            if m.finished
                && let Some(id) = m.courier
            {
                self.people[id].task = None;
                self.people[id].path_goal = None;
            }
            self.missions[idx] = m;
        }
    }
    fn local_enemies(&self, id: usize) -> Vec<usize> {
        let p = &self.people[id];
        self.people
            .iter()
            .filter(|q| {
                q.alive()
                    && q.side != p.side
                    && p.pos.near(q.pos, self.config.sight)
                    && self.terrain.sight(p.pos, q.pos)
            })
            .map(|q| q.id)
            .collect()
    }
    fn nearest_enemy(&self, id: usize) -> Option<usize> {
        let p = &self.people[id];
        self.local_enemies(id)
            .into_iter()
            .min_by_key(|&j| (p.pos.distance2(self.people[j].pos), j))
    }
    fn speed(&self, p: &Person) -> i32 {
        let speed = match p.role {
            Role::Line | Role::Guard | Role::Staff => self.config.speeds[0],
            Role::Ranged => self.config.speeds[1],
            Role::Light | Role::Scout => self.config.speeds[2],
            Role::General | Role::Escort => self.config.speeds[3],
            _ => self.config.speeds[4],
        };
        let rate = match self.terrain.kind(p.pos) {
            1 => self.config.road_speed_percent,
            2 => self.config.woods_speed_percent,
            5 => self.config.ford_speed_percent,
            _ => 100,
        };
        speed * rate / 100
            * (100
                - (p.missed.min(self.config.starvation_max_meals)
                    * self.config.starvation_step_percent) as i32)
            / 100
    }
    fn move_toward(&mut self, id: usize, target: Point) {
        if !self.terrain.walkable(target) {
            return;
        }
        let p = &self.people[id];
        if p.pos.near(target, self.config.arrival_radius) {
            return;
        }
        if p.path_goal
            .is_none_or(|old| Terrain::cell(old) != Terrain::cell(target))
            || p.path.is_empty()
        {
            let path = self.terrain.path(p.pos, target);
            self.people[id].path = path;
            self.people[id].path_goal = Some(target);
        }
        let p = &self.people[id];
        let Some(next) = p.path.first().copied() else {
            return;
        };
        let speed = self.speed(p).max(1);
        let dx = next.x - p.pos.x;
        let dy = next.y - p.pos.y;
        let dist = isqrt((dx as i64 * dx as i64 + dy as i64 * dy as i64) as u64).max(1) as i32;
        let candidate = if dist <= speed {
            next
        } else {
            Point::new(p.pos.x + dx * speed / dist, p.pos.y + dy * speed / dist)
        };
        // FIFO bridge admission, shared across both travel directions.
        if self.terrain.kind(candidate) == 4 && self.terrain.kind(p.pos) != 4 {
            self.bridge_queue.entry(id).or_insert(self.tick);
            let occupants = self
                .people
                .iter()
                .filter(|q| q.alive() && self.terrain.kind(q.pos) == 4)
                .count();
            let head = self
                .bridge_queue
                .iter()
                .min_by_key(|&(who, when)| (*when, *who))
                .map(|(&who, _)| who);
            if occupants >= self.config.bridge_capacity || head != Some(id) {
                return;
            }
            self.bridge_queue.remove(&id);
        }
        let p = &mut self.people[id];
        p.pos = candidate;
        if candidate == next {
            p.path.remove(0);
        }
    }
    fn work_and_move(&mut self) {
        // Queue entries cannot retain priority after an actor dies or changes route.
        self.bridge_queue.retain(|id, _| {
            self.people[*id].alive()
                && self.people[*id]
                    .path
                    .first()
                    .is_some_and(|p| self.terrain.kind(*p) == 4)
        });
        for id in 0..self.people.len() {
            if !self.people[id].alive() {
                continue;
            }
            let p = self.people[id].clone();
            match p.role {
                Role::Trainee => {
                    if p.pos.near(
                        self.terrain.headquarters[p.side],
                        self.config.exchange_radius,
                    ) {
                        self.people[id].training = self.people[id].training.saturating_sub(1);
                        if self.people[id].training == 0 {
                            self.people[id].role = Role::General;
                            self.people[id].plan = Plan::hold(p.pos);
                            self.notice(p.side, "General training completed at headquarters.");
                        }
                    }
                }
                Role::Courier => {
                    if let Some(task) = p.task {
                        if let Some(m) = self.missions.get(task)
                            && !m.finished
                        {
                            self.move_toward(id, m.destination);
                        }
                    } else {
                        self.move_toward(id, p.duty_target);
                    }
                }
                Role::Carrier => self.carrier(id),
                Role::Builder => self.builder(id),
                Role::Staff => {
                    if let Some(post) = p.post {
                        let dest = self.posts[post].pos;
                        self.move_toward(id, dest);
                    }
                }
                Role::Escort => {
                    if let Some(g) = p.escort {
                        let q = &self.people[g];
                        if q.alive()
                            && p.pos.near(q.pos, self.config.sight)
                            && self.terrain.sight(p.pos, q.pos)
                        {
                            let target = Point::new(
                                q.pos.x + self.config.escort_offset,
                                q.pos.y + self.config.escort_offset,
                            );
                            if !p.pos.near(target, self.config.escort_radius) {
                                self.move_toward(id, target);
                            }
                        }
                    }
                }
                Role::Scout => {
                    self.scout(id);
                }
                _ => self.execute_plan(id),
            }
        }
    }
    fn carrier(&mut self, id: usize) {
        let p = self.people[id].clone();
        let hq = self.terrain.headquarters[p.side];
        if p.pos.near(hq, self.config.logistics_radius) {
            let room = self.config.carrier_capacity - p.cargo.iter().sum::<u64>();
            let amount = room.min(self.stock[p.side][0]);
            self.stock[p.side][0] -= amount;
            self.people[id].cargo[0] += amount;
        }
        if self.people[id].cargo[0] <= self.config.reserve_food {
            self.move_toward(id, hq);
            return;
        }
        if !p.pos.near(p.duty_target, self.config.rendezvous_radius) {
            self.move_toward(id, p.duty_target);
            return;
        }
        // Only people physically encountered at the assigned rendezvous can receive food.
        for other in 0..self.people.len() {
            if self.people[id].cargo[0] <= self.config.reserve_food {
                break;
            }
            let q = &self.people[other];
            if other != id
                && q.alive()
                && q.side == p.side
                && q.role != Role::Carrier
                && q.pos.near(p.pos, self.config.exchange_radius)
                && q.cargo[0] < self.config.refill_target
                && q.cargo.iter().sum::<u64>() < self.config.carrier_capacity
            {
                self.people[other].cargo[0] += 1;
                self.people[id].cargo[0] -= 1;
            }
        }
        if self.people[id].cargo[0] <= self.config.reserve_food {
            self.move_toward(id, hq);
        }
    }
    fn builder(&mut self, id: usize) {
        let p = self.people[id].clone();
        let Some(post_id) = p.post else { return };
        let post = self.posts[post_id].clone();
        let hq = self.terrain.headquarters[p.side];
        if p.material_quota > 0 && p.cargo[1] == 0 {
            if !p.pos.near(hq, self.config.logistics_radius) {
                self.move_toward(id, hq);
                return;
            }
            let amount = p
                .material_quota
                .min(self.config.carrier_capacity - p.cargo.iter().sum::<u64>())
                .min(self.stock[p.side][1]);
            self.stock[p.side][1] -= amount;
            self.people[id].cargo[1] += amount;
            self.people[id].material_quota -= amount;
            if amount == 0 {
                return;
            }
        }
        if !p.pos.near(post.pos, self.config.logistics_radius) {
            self.move_toward(id, post.pos);
            return;
        }
        if post.hp <= 0 {
            self.people[id].role = Role::Carrier;
            self.people[id].post = None;
            self.people[id].duty_target = hq;
            return;
        }
        // No duplicate structures at a site, including an unseen enemy construction.
        if self
            .posts
            .iter()
            .any(|q| q.id != post_id && q.pos == post.pos && q.hp > 0 && q.complete)
        {
            self.people[id].role = Role::Guard;
            self.people[id].plan = Plan::hold(p.pos);
            return;
        }
        let material = self.people[id].cargo[1];
        self.people[id].cargo[1] = 0;
        self.posts[post_id].stock[1] += material;
        if self.people[id].material_quota > 0 {
            return;
        }
        let post = &mut self.posts[post_id];
        if !post.complete && post.stock[1] >= self.config.build_material {
            post.work += 1;
            if post.work >= self.config.build_work {
                post.stock[1] -= self.config.build_material;
                self.consumed[p.side][1] += self.config.build_material;
                post.complete = true;
            }
        } else if post.complete && post.upgrading && post.stock[1] >= self.config.upgrade_material {
            post.upgrading_work += 1;
            if post.upgrading_work >= self.config.upgrade_work {
                post.stock[1] -= self.config.upgrade_material;
                self.consumed[p.side][1] += self.config.upgrade_material;
                post.upgraded = true;
                post.upgrading = false;
            }
        }
        if self.posts[post_id].complete && !self.posts[post_id].upgrading {
            self.people[id].role = if self.staff(post_id) < self.config.required_staff {
                Role::Staff
            } else {
                Role::Guard
            };
            self.people[id].plan = Plan::hold(p.pos);
        }
    }
    fn scout(&mut self, id: usize) {
        let p = self.people[id].clone();
        let hq = self.terrain.headquarters[p.side];
        if p.report_returning {
            let report_payload = self
                .missions
                .iter()
                .find(|m| m.courier == Some(id) && !m.finished)
                .map(|m| (m.id, m.payload.clone(), m.destination, m.wait_since));
            if let Some((mid, payload, destination, wait_since)) = report_payload {
                if self.deliver_to_command(p.side, p.pos, &payload) {
                    self.missions[mid].finished = true;
                    self.people[id].report_returning = false;
                    self.people[id].last_report = self.tick;
                    self.people[id].path_goal = None;
                } else {
                    // Travel to a fixed, briefed rendezvous, never a hidden moving unit.
                    if p.pos.near(destination, self.config.exchange_radius) {
                        if wait_since.is_some_and(|t| self.tick - t >= self.config.courier_wait) {
                            self.missions[mid].destination = hq;
                        } else if wait_since.is_none() {
                            self.missions[mid].wait_since = Some(self.tick);
                        }
                    }
                    self.move_toward(id, self.missions[mid].destination);
                }
                return;
            }
            self.people[id].report_returning = false;
        }
        // Scouts carry their own observations home; they do not spawn couriers.
        if self.tick - p.last_report >= self.config.report_ticks
            && (!self.local_enemies(id).is_empty()
                || p.pos
                    .near(p.plan.target, self.config.observation_arrival_radius))
        {
            let report = self.observe(
                p.side,
                p.pos,
                self.config.sight,
                format!("scout:{id}"),
                300 + id as u64,
            );
            let payload = Payload::Reports {
                reports: vec![report],
            };
            if self.deliver_to_command(p.side, p.pos, &payload) {
                self.people[id].last_report = self.tick;
            } else {
                let mid = self.missions.len();
                self.missions.push(Mission {
                    id: mid,
                    side: p.side,
                    source: p.pos,
                    destination: [hq, self.terrain.sites[if p.side == 0 { 0 } else { 3 }]]
                        .into_iter()
                        .min_by_key(|q| p.pos.distance2(*q))
                        .unwrap(),
                    payload,
                    courier: Some(id),
                    due: u64::MAX,
                    wait_since: None,
                    returning: true,
                    finished: false,
                    relay_path: vec![],
                    recipients: vec![],
                });
                self.people[id].report_returning = true;
            }
        }
        self.execute_plan(id);
    }
    fn execute_plan(&mut self, id: usize) {
        let p = self.people[id].clone();
        if p.plan.verb == Verb::Observe
            && p.role.combatant()
            && self.tick - p.last_report >= self.config.report_ticks
        {
            let report = self.observe(
                p.side,
                p.pos,
                self.config.sight,
                format!("observer:{id}"),
                id as u64,
            );
            if report.people.iter().any(|q| q.side != p.side) {
                let payload = Payload::Reports {
                    reports: vec![report],
                };
                if self.deliver_to_command(p.side, p.pos, &payload) {
                    self.people[id].last_report = self.tick;
                } else if let Some(courier) = self
                    .people
                    .iter()
                    .find(|q| {
                        q.side == p.side
                            && q.alive()
                            && q.role == Role::Courier
                            && q.task.is_none()
                            && q.pos.near(p.pos, self.config.exchange_radius)
                    })
                    .map(|q| q.id)
                {
                    let mid = self.missions.len();
                    self.missions.push(Mission {
                        id: mid,
                        side: p.side,
                        source: p.pos,
                        destination: self.terrain.headquarters[p.side],
                        payload,
                        courier: Some(courier),
                        due: self.tick + self.config.local_exchange_ticks,
                        wait_since: None,
                        returning: true,
                        finished: false,
                        relay_path: vec![],
                        recipients: vec![],
                    });
                    self.people[courier].task = Some(mid);
                    self.people[courier].path_goal = None;
                    self.people[courier].path.clear();
                    self.people[id].last_report = self.tick;
                }
            }
        }
        let mut plan = p.plan.clone();
        if let Some(fallback) = plan.fallback {
            let local_alive = p
                .accepted_group
                .iter()
                .filter(|&&i| {
                    self.people[i].alive()
                        && self.people[i].pos.near(p.pos, self.config.sight)
                        && self.terrain.sight(p.pos, self.people[i].pos)
                })
                .count();
            // Local absent members do not count as confirmed casualties. Only locally seen dead members do.
            let local_dead = p
                .accepted_group
                .iter()
                .filter(|&&i| {
                    !self.people[i].alive()
                        && self.people[i].pos.near(p.pos, self.config.sight)
                        && self.terrain.sight(p.pos, self.people[i].pos)
                })
                .count();
            let _ = local_alive;
            if p.accepted_strength > 0
                && (p.accepted_strength - local_dead) * 100
                    < p.accepted_strength * self.config.fallback_percent as usize
            {
                plan.verb = Verb::Withdraw;
                plan.target = fallback;
                plan.fallback = None;
                self.people[id].plan = plan.clone();
            }
        }
        let enemy = self.nearest_enemy(id);
        if (plan.manner == Manner::Avoid || matches!(plan.verb, Verb::Observe | Verb::Withdraw))
            && let Some(e) = enemy
        {
            let q = &self.people[e];
            if p.pos.near(q.pos, self.config.avoidance_radius) && plan.verb != Verb::Withdraw {
                let target = Point::new(
                    (p.pos.x + (p.pos.x - q.pos.x).signum() * self.config.avoidance_step)
                        .clamp(500, 89500),
                    (p.pos.y + (p.pos.y - q.pos.y).signum() * self.config.avoidance_step)
                        .clamp(500, 59500),
                );
                self.move_toward(id, target);
                return;
            }
        }
        let mut target = if plan.verb == Verb::Move && p.route_index < plan.route.len() {
            plan.route[p.route_index]
        } else {
            plan.target
        };
        if plan.verb == Verb::Hold
            && p.pos
                .near(target, plan.area_radius.max(self.config.hold_radius))
        {
            return;
        }
        if plan.verb == Verb::Observe && plan.area_radius > 0 {
            let offsets = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            let (dx, dy) = offsets[p.route_index % 4];
            let candidate = Point::new(
                plan.target.x + dx * plan.area_radius,
                plan.target.y + dy * plan.area_radius,
            );
            if self.terrain.walkable(candidate) {
                target = candidate;
            }
        }
        if plan.verb == Verb::Engage
            && let Some(e) = enemy
        {
            let q = &self.people[e];
            if (plan.area_radius == 0 || q.pos.near(plan.target, plan.area_radius))
                && (plan.manner != Manner::NoPursuit
                    || q.pos.near(plan.target, self.config.pursuit_radius))
            {
                target = q.pos;
                let range = if p.role == Role::Ranged {
                    self.config.ranged_range * self.config.firing_standoff_percent / 100
                } else {
                    self.config.melee_range * self.config.firing_standoff_percent / 100
                };
                if p.pos.near(target, range) {
                    return;
                }
            }
        }
        if plan.manner == Manner::Together && p.accepted_group.len() > 1 {
            let nearby: Vec<_> = p
                .accepted_group
                .iter()
                .map(|&i| &self.people[i])
                .filter(|q| {
                    q.alive()
                        && q.pos.near(p.pos, self.config.sight)
                        && self.terrain.sight(p.pos, q.pos)
                })
                .collect();
            if !nearby.is_empty() {
                let center = Point::new(
                    nearby.iter().map(|q| q.pos.x as i64).sum::<i64>() as i32 / nearby.len() as i32,
                    nearby.iter().map(|q| q.pos.y as i64).sum::<i64>() as i32 / nearby.len() as i32,
                );
                if !p.pos.near(center, self.config.cohesion_radius)
                    && p.pos.distance2(target) < center.distance2(target)
                {
                    return;
                }
            }
        }
        if p.pos.near(target, self.config.waypoint_radius) {
            if (plan.verb == Verb::Move && p.route_index < plan.route.len())
                || (plan.verb == Verb::Observe && plan.area_radius > 0)
            {
                self.people[id].route_index += 1;
            } else if let Some(next) = plan.then {
                self.people[id].plan.verb = next;
                self.people[id].plan.then = None;
            }
            return;
        }
        self.move_toward(id, target);
    }
    fn combat(&mut self) {
        let mut hits = vec![0i32; self.people.len()];
        let mut post_hits = vec![0i32; self.posts.len()];
        for id in 0..self.people.len() {
            let p = &self.people[id];
            if !p.alive()
                || p.next_attack > self.tick
                || (!p.role.combatant() && p.role != Role::General)
            {
                continue;
            }
            let retreat = matches!(p.plan.verb, Verb::Withdraw | Verb::Observe)
                || p.plan.manner == Manner::Avoid;
            let ranged = p.role == Role::Ranged && !retreat;
            let range = if ranged {
                self.config.ranged_range
            } else {
                self.config.melee_range
            };
            let target = self
                .local_enemies(id)
                .into_iter()
                .filter(|&j| p.pos.near(self.people[j].pos, range))
                .min_by_key(|&j| (p.pos.distance2(self.people[j].pos), j));
            let raw = if ranged {
                self.config.ranged_damage
            } else {
                match p.role {
                    Role::Line => self.config.melee_damage[0],
                    Role::Light => self.config.melee_damage[2],
                    _ => self.config.melee_damage[1],
                }
            };
            let damage = (raw
                * (100
                    - (p.missed.min(self.config.starvation_max_meals)
                        * self.config.starvation_step_percent) as i32)
                / 100)
                .max(1);
            if let Some(j) = target {
                let q = &self.people[j];
                let armor = if q.role == Role::Line {
                    self.config.line_armor
                } else {
                    0
                };
                let cover = if ranged && self.terrain.kind(q.pos) == 2 {
                    self.config.woods_cover_divisor
                } else {
                    1
                };
                hits[j] += ((damage - armor) / cover).max(1);
                self.people[id].next_attack = self.tick
                    + if ranged {
                        self.config.ranged_ticks
                    } else {
                        self.config.melee_ticks
                    };
            } else if p.plan.verb == Verb::Engage
                && let Some(post) = self
                    .posts
                    .iter()
                    .filter(|q| {
                        q.side != p.side
                            && q.hp > 0
                            && (q.complete || q.work > 0 || q.stock[1] > 0)
                            && p.pos.near(q.pos, range)
                            && self.terrain.sight(p.pos, q.pos)
                    })
                    .min_by_key(|q| (p.pos.distance2(q.pos), q.id))
            {
                post_hits[post.id] += damage;
                self.people[id].next_attack = self.tick
                    + if ranged {
                        self.config.ranged_ticks
                    } else {
                        self.config.melee_ticks
                    };
            }
        }
        let mut lost_generals = vec![];
        for (id, damage) in hits.into_iter().enumerate() {
            let was_alive = self.people[id].alive();
            self.people[id].hp = (self.people[id].hp - damage).max(0);
            if was_alive && !self.people[id].alive() && self.people[id].role == Role::General {
                lost_generals.push(id);
            }
        }
        for id in lost_generals {
            let p = &self.people[id];
            let report = self.observe(p.side, p.pos, 0, format!("general:{id}"), id as u64 + 1);
            self.receive(p.side, report);
        }
        for (id, damage) in post_hits.into_iter().enumerate() {
            self.posts[id].hp = (self.posts[id].hp - damage).max(0);
        }
    }
    fn victory(&mut self) {
        for attacker in 0..2 {
            let pos = self.terrain.headquarters[1 - attacker];
            let attackers = self
                .people
                .iter()
                .filter(|p| {
                    p.side == attacker
                        && p.alive()
                        && p.role.combatant()
                        && p.pos.near(pos, self.config.capture_radius)
                })
                .count();
            let defenders = self.people.iter().any(|p| {
                p.side != attacker
                    && p.alive()
                    && p.role.combatant()
                    && p.pos.near(pos, self.config.capture_radius)
            });
            if attackers >= self.config.capture_people && !defenders {
                self.capture[attacker] += 1;
            } else {
                self.capture[attacker] = 0;
            }
        }
        let captures = [
            self.capture[0] >= self.config.capture_ticks,
            self.capture[1] >= self.config.capture_ticks,
        ];
        if captures == [true; 2] {
            self.status = Status::Finished {
                winner: None,
                reason: "simultaneous capture".into(),
            };
        } else if captures.iter().any(|&v| v) {
            self.status = Status::Finished {
                winner: Some(usize::from(captures[1])),
                reason: "headquarters captured".into(),
            };
        } else if self.connected == [false; 2]
            && self
                .disconnected_at
                .iter()
                .flatten()
                .any(|t| self.tick - t >= self.config.disconnect_ticks)
        {
            self.status = Status::Finished {
                winner: None,
                reason: "both players disconnected".into(),
            };
        } else if let Some(side) = (0..2).find(|&s| {
            self.disconnected_at[s].is_some_and(|t| self.tick - t >= self.config.disconnect_ticks)
        }) {
            self.status = Status::Finished {
                winner: Some(1 - side),
                reason: "disconnect timeout".into(),
            };
        } else if self.tick >= self.config.max_ticks {
            self.status = Status::Finished {
                winner: None,
                reason: "twenty-minute time limit".into(),
            };
        }
    }
}
fn isqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut x = n;
    let mut y = x.div_ceil(2);
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}
