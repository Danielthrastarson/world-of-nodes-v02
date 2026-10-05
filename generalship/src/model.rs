use serde::{Deserialize, Serialize};
pub type PersonId = usize;
pub type Tick = u64;
pub const DT_MS: u64 = 100;
// Legacy integer stock convention: food, wood, stone. Materials use wood.
pub type Stock = world_of_nodes::Stock;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance2(self, b: Self) -> i64 {
        let dx = (self.x - b.x) as i64;
        let dy = (self.y - b.y) as i64;
        dx * dx + dy * dy
    }
    pub fn near(self, b: Self, radius: i32) -> bool {
        self.distance2(b) <= radius as i64 * radius as i64
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub max_ticks: Tick,
    pub people_per_side: usize,
    pub hq_food: u64,
    pub hq_material: u64,
    pub starting_food: u64,
    pub meal_ticks: Tick,
    pub carrier_capacity: u64,
    pub sight: i32,
    pub general_sight: i32,
    pub outpost_sight: i32,
    pub exchange_radius: i32,
    pub relay_range: i32,
    pub upgraded_range: i32,
    pub relay_ticks: Tick,
    pub upgraded_ticks: Tick,
    pub local_exchange_ticks: Tick,
    pub courier_wait: Tick,
    pub build_material: u64,
    pub build_work: Tick,
    pub upgrade_material: u64,
    pub upgrade_work: Tick,
    pub training_food: u64,
    pub training_ticks: Tick,
    pub capture_ticks: Tick,
    pub capture_people: usize,
    pub disconnect_ticks: Tick,
    pub capture_radius: i32,
    pub person_hp: i32,
    pub post_hp: i32,
    pub speeds: [i32; 5],
    pub melee_damage: [i32; 3],
    pub ranged_damage: i32,
    pub melee_ticks: Tick,
    pub ranged_ticks: Tick,
    pub melee_range: i32,
    pub ranged_range: i32,
    pub line_armor: i32,
    pub bridge_capacity: usize,
    pub required_staff: usize,
    pub staff_missed_meals: u32,
    pub starvation_step_percent: u32,
    pub starvation_max_meals: u32,
    pub report_ticks: Tick,
    pub fallback_percent: i32,
    pub avoidance_radius: i32,
    pub avoidance_step: i32,
    pub cohesion_radius: i32,
    pub pursuit_radius: i32,
    pub hold_radius: i32,
    pub waypoint_radius: i32,
    pub arrival_radius: i32,
    pub logistics_radius: i32,
    pub rendezvous_radius: i32,
    pub courier_arrival_radius: i32,
    pub observation_arrival_radius: i32,
    pub escort_offset: i32,
    pub escort_radius: i32,
    pub refill_target: u64,
    pub reserve_food: u64,
    pub firing_standoff_percent: i32,
    pub woods_cover_divisor: i32,
    pub road_speed_percent: i32,
    pub woods_speed_percent: i32,
    pub ford_speed_percent: i32,
    pub short_sight: i32,
    pub woods_visibility: i32,
}
impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(include_str!("../scenarios/g0-v1.json"))
            .expect("valid committed scenario")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    General,
    Line,
    Ranged,
    Light,
    Carrier,
    Courier,
    Scout,
    Staff,
    Escort,
    Guard,
    Builder,
    Trainee,
}
impl Role {
    pub fn combatant(self) -> bool {
        matches!(
            self,
            Self::Line | Self::Ranged | Self::Light | Self::Escort | Self::Guard
        )
    }
    pub fn assignable(self) -> bool {
        !matches!(self, Self::General | Self::Trainee)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verb {
    Move,
    Hold,
    Observe,
    Engage,
    Withdraw,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Manner {
    Together,
    NoPursuit,
    Avoid,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    #[serde(default)]
    pub area_radius: i32,
    pub verb: Verb,
    pub target: Point,
    #[serde(default)]
    pub route: Vec<Point>,
    pub manner: Manner,
    pub then: Option<Verb>,
    pub fallback: Option<Point>,
}
impl Plan {
    pub fn hold(p: Point) -> Self {
        Self {
            area_radius: 0,
            verb: Verb::Hold,
            target: p,
            route: vec![],
            manner: Manner::NoPursuit,
            then: None,
            fallback: None,
        }
    }
    pub fn valid(&self) -> bool {
        self.area_radius >= 0
            && self.area_radius <= 12000
            && (self.area_radius == 0
                || matches!(self.verb, Verb::Hold | Verb::Observe | Verb::Engage))
            && self.route.len() <= 8
            && (self.route.is_empty() || self.verb == Verb::Move)
            && !(self.then.is_some() && self.fallback.is_some())
            && self
                .then
                .is_none_or(|v| self.verb == Verb::Move && matches!(v, Verb::Hold | Verb::Observe))
            && !(self.manner == Manner::Avoid && matches!(self.verb, Verb::Hold | Verb::Engage))
            && self
                .route
                .iter()
                .chain(std::iter::once(&self.target))
                .chain(self.fallback.iter())
                .all(|p| p.x >= 0 && p.x <= 90000 && p.y >= 0 && p.y <= 60000)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Source {
    Hq,
    General { id: usize },
    Outpost { id: usize },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Input {
    Ready,
    MoveGeneral {
        id: usize,
        target: Point,
    },
    Order {
        #[serde(default)]
        rendezvous: Option<Point>,
        source: Source,
        recipients: Vec<usize>,
        plan: Plan,
    },
    Assign {
        source: Source,
        people: Vec<usize>,
        role: Role,
        target: Point,
        post: Option<usize>,
        escort: Option<usize>,
    },
    Build {
        site: usize,
        people: Vec<usize>,
    },
    Upgrade {
        post: usize,
        people: Vec<usize>,
    },
    Train {
        person: usize,
    },
    Surrender,
    Connection {
        connected: bool,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: usize,
    pub side: usize,
    pub role: Role,
    pub pos: Point,
    pub hp: i32,
    pub cargo: Stock,
    pub missed: u32,
    pub plan: Plan,
    pub path: Vec<Point>,
    pub path_goal: Option<Point>,
    pub sequence: u64,
    pub accepted_group: Vec<usize>,
    pub accepted_strength: usize,
    pub next_attack: Tick,
    pub duty_target: Point,
    pub post: Option<usize>,
    pub escort: Option<usize>,
    pub task: Option<usize>,
    pub training: Tick,
    pub route_index: usize,
    pub last_report: Tick,
    pub report_returning: bool,
    pub material_quota: u64,
}
impl Person {
    pub fn alive(&self) -> bool {
        self.hp > 0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: usize,
    pub side: usize,
    pub pos: Point,
    pub hp: i32,
    pub complete: bool,
    pub upgraded: bool,
    pub upgrading: bool,
    pub stock: Stock,
    pub work: Tick,
    pub upgrading_work: Tick,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    pub absent_since: Option<Tick>,
    pub id: usize,
    pub side: usize,
    pub role: Role,
    pub pos: Point,
    pub hp: i32,
    pub cargo: Option<Stock>,
    pub missed_meals: Option<u32>,
    pub received_order: Option<u64>,
    pub observed: Tick,
    pub received: Tick,
    pub source: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostContact {
    pub id: usize,
    pub side: usize,
    pub pos: Point,
    pub hp: i32,
    pub complete: bool,
    pub upgraded: bool,
    pub staff: usize,
    pub stock: Option<Stock>,
    pub work: Option<Tick>,
    pub upgrading_work: Option<Tick>,
    pub observed: Tick,
    pub received: Tick,
    pub source: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub observed: Tick,
    pub received: Tick,
    pub source: String,
    pub pos: Point,
    pub radius: i32,
    pub people: Vec<Contact>,
    pub posts: Vec<PostContact>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notice {
    pub tick: Tick,
    pub text: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Knowledge {
    pub people: std::collections::BTreeMap<usize, Contact>,
    pub posts: std::collections::BTreeMap<usize, PostContact>,
    pub reports: Vec<Report>,
    pub log: Vec<Notice>,
    pub orders: Vec<OrderRecord>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderRecord {
    pub id: u64,
    pub issued: Tick,
    pub recipients: Vec<usize>,
    pub plan: Plan,
    pub status: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Payload {
    Order {
        id: u64,
        sequence: u64,
        recipients: Vec<usize>,
        plan: Plan,
    },
    Assign {
        id: u64,
        sequence: u64,
        people: Vec<usize>,
        role: Role,
        target: Point,
        post: Option<usize>,
        escort: Option<usize>,
    },
    Reports {
        reports: Vec<Report>,
    },
    Ack {
        id: u64,
        status: String,
        reports: Vec<Report>,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mission {
    pub id: usize,
    pub side: usize,
    pub source: Point,
    pub destination: Point,
    pub payload: Payload,
    pub courier: Option<usize>,
    pub due: Tick,
    pub wait_since: Option<Tick>,
    pub returning: bool,
    pub finished: bool,
    pub relay_path: Vec<usize>,
    pub recipients: Vec<usize>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signal {
    pub side: usize,
    pub path: Vec<usize>,
    pub due: Tick,
    pub reports: Vec<Report>,
    pub ack: Option<(u64, String)>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRecord {
    pub tick: Tick,
    pub side: usize,
    pub input: Input,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Replay {
    pub config: Config,
    pub seed: u64,
    pub inputs: Vec<InputRecord>,
    pub end_tick: Tick,
    pub hash: String,
    pub checkpoints: Vec<(Tick, String)>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Status {
    Lobby,
    Running,
    Finished {
        winner: Option<usize>,
        reason: String,
    },
}
