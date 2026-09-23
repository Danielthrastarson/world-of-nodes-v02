//! A deterministic input boundary over the existing headless world.
//!
//! Scenario setup is explicit initial history, not an autonomous result. After
//! setup every Person uses the normal policy. There are no research actor holds,
//! scripted project completions, protection orders, or outcome interventions.
use super::super::*;
use super::military::{self as player_military, MilitaryAgreement, MilitaryOrder};

pub const RULER: usize = 0;
pub const STEWARD: usize = 1;
pub const REPORT_COURIER: usize = 2;
pub const ORDER_COURIER: usize = 5;
pub const STEWARD_OFFICE: usize = 0;
pub const CROSSING_PROJECT: usize = 0;
pub const UNIT: usize = 0;
pub const ORDER_LIFETIME: Time = 48 * HOUR;
/// Institutional contact opportunities occur on this absolute schedule, never
/// once per renderer frame or snapshot request.
pub const CONTACT_INTERVAL: Time = MINUTE;
pub const NODE_LABELS: [&str; 5] = [
    "Ruler's rest",
    "Ford settlement",
    "Food settlement",
    "Detour",
    "Raid approach",
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScenarioKind {
    /// Accepted office and voluntary militia already physically meet the ruler.
    #[default]
    Local,
    /// The ruler and agreed courier make a real pre-play trip to node 0. The
    /// steward continues ordinary autonomous life throughout that trip.
    Remote,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerCommand {
    Commission {
        office: usize,
        project: usize,
    },
    Revoke {
        command: commands::CommandId,
    },
    Unit {
        verb: MilitaryOrder,
        target: usize,
    },
    /// Take a report of the ruler's actual local observation. No remote query.
    ObserveReport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DispatchState {
    DeliveredLocally,
    WithAgreedCourier,
    HeldByIssuer,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandResult {
    Order {
        message: commands::MessageId,
        dispatch: DispatchState,
    },
    Revocation {
        message: commands::MessageId,
        dispatch: DispatchState,
    },
    Unit {
        receivers: usize,
    },
    Report {
        report: information::ReportId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandRecord {
    pub at: Time,
    pub command: PlayerCommand,
    /// Failed inputs are part of replay evidence as well as successful inputs.
    pub outcome: Result<CommandResult, String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CourierLeg {
    Outbound { message: commands::MessageId },
    Returning { report: information::ReportId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CourierTrip {
    return_node: usize,
    leg: CourierLeg,
}

/// Player knowledge of a handoff personally performed by the embodied ruler.
/// This is not the recipient's remote obligation state or an acknowledgement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct LocalHandoff {
    pub(super) at: Time,
    pub(super) state: DispatchState,
}

/// A Session adds typed ruler inputs and a finite, explicitly agreed courier
/// service to World. Renderer consumers receive only an immutable reference.
pub struct Session {
    pub(super) world: World,
    seed: u64,
    scenario: ScenarioKind,
    command_log: Vec<CommandRecord>,
    pending_messages: Vec<commands::MessageId>,
    next_contact: Time,
    courier_trip: Option<CourierTrip>,
    courier_round_trips_left: usize,
    military_agreement: MilitaryAgreement,
    /// Explicit scenario metadata distinguishes a chart from a stock/person
    /// observation. It does not reinterpret arbitrary empty NPC memories.
    pub(super) map_only_seeds: BTreeMap<usize, Time>,
    pub(super) local_handoffs: BTreeMap<commands::MessageId, LocalHandoff>,
}

impl Session {
    pub fn new(seed: u64, scenario: ScenarioKind) -> Result<Self, String> {
        let mut world = scenario_world(seed, scenario)?;
        if scenario == ScenarioKind::Remote {
            // This is a physical starting journey by the two consenting actors,
            // not a location rewrite. Any subsequent autonomous choice remains.
            world.apply_intents(vec![
                (
                    RULER,
                    Intent::Travel {
                        edge: 0,
                        target: 0,
                        food: false,
                    },
                ),
                (
                    ORDER_COURIER,
                    Intent::Travel {
                        edge: 0,
                        target: 0,
                        food: false,
                    },
                ),
            ])?;
            world.advance_to(HOUR / 4)?;
        }
        let next_contact = (world.now() / CONTACT_INTERVAL + 1) * CONTACT_INTERVAL;
        let map_only_seeds = world.agents[RULER]
            .memory
            .nodes
            .iter()
            .filter(|(_, view)| view.observed_at == -HOUR)
            .map(|(&node, view)| (node, view.observed_at))
            .collect();
        Ok(Self {
            world,
            seed,
            scenario,
            command_log: Vec::new(),
            pending_messages: Vec::new(),
            next_contact,
            courier_trip: None,
            // An initial agreement for two round trips, not a general right to
            // command civilians: one order and one later revocation can travel.
            courier_round_trips_left: 2,
            military_agreement: MilitaryAgreement {
                commander: RULER,
                unit: UNIT,
            },
            map_only_seeds,
            local_handoffs: BTreeMap::new(),
        })
    }

    pub fn world(&self) -> &World {
        &self.world
    }
    pub fn ruler(&self) -> usize {
        RULER
    }
    pub fn steward(&self) -> usize {
        STEWARD
    }
    pub fn office(&self) -> usize {
        STEWARD_OFFICE
    }
    pub fn courier(&self) -> usize {
        ORDER_COURIER
    }
    pub fn seed(&self) -> u64 {
        self.seed
    }
    pub fn scenario(&self) -> ScenarioKind {
        self.scenario
    }
    pub fn command_log(&self) -> &[CommandRecord] {
        &self.command_log
    }

    /// Arbitrary read/checkpoint cadence does not create additional contact
    /// opportunities. The kernel itself owns all physical event timing.
    pub fn advance_to(&mut self, to: Time) -> Result<(), String> {
        if to < self.world.now() {
            return Err("session cannot run backwards".into());
        }
        // Replaying several inputs at the paused initial timestamp must not run
        // an initial policy boundary between them merely because time was read.
        if to == self.world.now() {
            return Ok(());
        }
        while self.next_contact <= to {
            self.world.advance_to(self.next_contact)?;
            self.courier_contact()?;
            self.next_contact = self
                .next_contact
                .checked_add(CONTACT_INTERVAL)
                .ok_or("session clock overflow")?;
        }
        self.world.advance_to(to)
    }

    pub fn dispatch(&mut self, command: PlayerCommand) -> Result<CommandResult, String> {
        let at = self.world.now();
        let outcome = self.dispatch_inner(&command);
        self.command_log.push(CommandRecord {
            at,
            command,
            outcome: outcome.clone(),
        });
        outcome
    }

    fn dispatch_inner(&mut self, command: &PlayerCommand) -> Result<CommandResult, String> {
        let result = match *command {
            PlayerCommand::Commission { office, project } => {
                // Scope comes from the ruler's own physical office certificate,
                // never from a lookup of the distant steward's current state.
                let settlement = self
                    .world
                    .command_memory(RULER)
                    .and_then(|m| m.accepted_offices.get(&office))
                    .ok_or("ruler has no accepted office certificate")?
                    .settlement;
                let message = self.world.issue_order(
                    RULER,
                    office,
                    commands::Objective::CommissionCrossing {
                        project,
                        settlement,
                    },
                    self.world
                        .now()
                        .checked_add(ORDER_LIFETIME)
                        .ok_or("order deadline overflow")?,
                )?;
                self.pending_messages.push(message);
                let dispatch = self.try_message_contact(message)?;
                CommandResult::Order { message, dispatch }
            }
            PlayerCommand::Revoke { command } => {
                let message = self.world.issue_revocation(RULER, command)?;
                self.pending_messages.push(message);
                let dispatch = self.try_message_contact(message)?;
                CommandResult::Revocation { message, dispatch }
            }
            PlayerCommand::Unit { verb, target } => CommandResult::Unit {
                receivers: player_military::issue(
                    &mut self.world,
                    &self.military_agreement,
                    verb,
                    target,
                )?,
            },
            PlayerCommand::ObserveReport => CommandResult::Report {
                report: self.world.capture_report(RULER)?,
            },
        };
        self.world.audit()?;
        Ok(result)
    }

    /// Contact decisions use only the ruler's current local view and its held
    /// envelope. A missing person is never sought through a global tracker.
    fn try_message_contact(
        &mut self,
        message: commands::MessageId,
    ) -> Result<DispatchState, String> {
        let Some(local) = self.world.observe(RULER).and_then(|o| o.local) else {
            return Ok(DispatchState::HeldByIssuer);
        };
        let envelope = self
            .world
            .command_memory(RULER)
            .and_then(|m| m.envelopes.get(&message))
            .ok_or("issuer lost envelope")?
            .envelope
            .clone();
        let recipient = envelope.recipient();
        if local.people.contains(&recipient)
            && self
                .world
                .receive_command(RULER, recipient, message)
                .is_ok()
        {
            self.pending_messages.retain(|&id| id != message);
            self.local_handoffs.insert(
                message,
                LocalHandoff {
                    at: self.world.now(),
                    state: DispatchState::DeliveredLocally,
                },
            );
            return Ok(DispatchState::DeliveredLocally);
        }
        if self.courier_trip.is_some()
            || self.courier_round_trips_left == 0
            || !local.people.contains(&ORDER_COURIER)
            || self
                .world
                .command_memory(ORDER_COURIER)
                .is_some_and(|m| m.delivery.is_some())
            || self
                .world
                .information_memory(ORDER_COURIER)
                .is_some_and(|m| m.delivery.is_some())
        {
            return Ok(DispatchState::HeldByIssuer);
        }
        let office_id = match &envelope {
            commands::Envelope::Order(o) => o.office,
            commands::Envelope::Revoke(r) => r.office,
        };
        let rendezvous = self
            .world
            .command_memory(RULER)
            .and_then(|m| m.accepted_offices.get(&office_id))
            .ok_or("no personally known office rendezvous")?
            .settlement;
        // Kernel endpoints still validate actual contact, capability and held
        // information. Neither possession nor consent guarantees future arrival.
        if self
            .world
            .receive_command(RULER, ORDER_COURIER, message)
            .is_err()
        {
            return Ok(DispatchState::HeldByIssuer);
        }
        self.world
            .begin_command_delivery(ORDER_COURIER, message, recipient, rendezvous)?;
        self.courier_round_trips_left -= 1;
        self.courier_trip = Some(CourierTrip {
            return_node: local.node,
            leg: CourierLeg::Outbound { message },
        });
        self.pending_messages.retain(|&id| id != message);
        self.local_handoffs.insert(
            message,
            LocalHandoff {
                at: self.world.now(),
                state: DispatchState::WithAgreedCourier,
            },
        );
        Ok(DispatchState::WithAgreedCourier)
    }

    /// The pre-agreed return leg carries an actual local F12 observation, not a
    /// telepathic completion acknowledgement. Missed rendezvous stays a failure.
    fn courier_contact(&mut self) -> Result<(), String> {
        if let Some(trip) = self.courier_trip.clone() {
            match trip.leg {
                CourierLeg::Outbound { message } => {
                    let still_delivering = self
                        .world
                        .command_memory(ORDER_COURIER)
                        .is_some_and(|m| m.delivery.as_ref().is_some_and(|d| d.message == message));
                    if !still_delivering {
                        if self.world.information_actor_node(ORDER_COURIER).is_ok() {
                            let report = self.world.capture_report(ORDER_COURIER)?;
                            self.world.begin_report_delivery(
                                ORDER_COURIER,
                                report,
                                RULER,
                                trip.return_node,
                            )?;
                            self.courier_trip = Some(CourierTrip {
                                return_node: trip.return_node,
                                leg: CourierLeg::Returning { report },
                            });
                        } else if self.world.agents[ORDER_COURIER].action == Action::Dead {
                            self.courier_trip = None;
                        }
                    }
                }
                CourierLeg::Returning { report } => {
                    if !self
                        .world
                        .information_memory(ORDER_COURIER)
                        .is_some_and(|m| m.delivery.as_ref().is_some_and(|d| d.report == report))
                    {
                        self.courier_trip = None;
                    }
                }
            }
        }
        for message in self.pending_messages.clone() {
            self.try_message_contact(message)?;
        }
        self.world.audit()
    }

    pub fn replay(
        seed: u64,
        scenario: ScenarioKind,
        commands: &[CommandRecord],
        until: Time,
    ) -> Result<Self, String> {
        let mut session = Self::new(seed, scenario)?;
        for record in commands {
            if record.at > until {
                return Err("replay ends before a recorded input".into());
            }
            session.advance_to(record.at)?;
            let actual = session.dispatch(record.command.clone());
            if actual != record.outcome {
                return Err(format!(
                    "input replay diverged at {}: {actual:?} != {:?}",
                    record.at, record.outcome
                ));
            }
        }
        session.advance_to(until)?;
        Ok(session)
    }
}

/// Same physical quantities and capabilities as F13A's alternate-route world.
/// This constructor intentionally does not reuse its research Run controller.
fn scenario_world(seed: u64, scenario: ScenarioKind) -> Result<World, String> {
    let mut starts = vec![1; 18];
    starts[REPORT_COURIER] = 2;
    starts[9..12].fill(4);
    starts[12..16].fill(2);
    let config = Config {
        seed,
        trace: true,
        information: true,
        commands: true,
        formation: true,
        nodes: 5,
        starts,
        edges: vec![
            Edge {
                a: 0,
                b: 1,
                nominal_ms: HOUR / 4,
                capacity: 4,
            },
            Edge {
                a: 1,
                b: 2,
                nominal_ms: HOUR / 4,
                capacity: 0,
            },
            Edge {
                a: 1,
                b: 3,
                nominal_ms: 2 * HOUR,
                capacity: 4,
            },
            Edge {
                a: 3,
                b: 2,
                nominal_ms: 2 * HOUR,
                capacity: 4,
            },
            Edge {
                a: 4,
                b: 2,
                nominal_ms: 2 * HOUR,
                capacity: 4,
            },
        ],
        food: vec![(1, 24, 0, 48 * HOUR)],
        boards: vec![1],
        initial_request: 60,
        initial_nutrition: 24 * HOUR,
        authority: Some(authority::Config {
            polity_id: 17,
            sovereign: RULER,
            settlements: vec![1, 2],
        }),
        infrastructure: Some(infrastructure::Config {
            enabled: true,
            wood: vec![(1, 100)],
            sites: vec![infrastructure::Site {
                edge: 1,
                node: 1,
                wood_cost: 20,
                work_ms: 8 * HOUR,
                opened_nominal_ms: HOUR / 4,
                opened_capacity: 4,
            }],
        }),
        paths: Some(paths::Config::ground(vec![0, 2, 3, 4], true)),
        farming: Some(farming::Config {
            units: vec![1, 1, 2, 2, 2, 2],
            rules: Default::default(),
        }),
        household: Some(household::Config {
            enabled: true,
            households: vec![
                household::Household {
                    id: 0,
                    members: vec![3, 16],
                    home: Some(1),
                },
                household::Household {
                    id: 1,
                    members: vec![4, 17],
                    home: Some(1),
                },
            ],
            kin: vec![
                household::Relation {
                    from: 3,
                    to: 16,
                    kind: household::Kin::Parent,
                },
                household::Relation {
                    from: 4,
                    to: 17,
                    kind: household::Kin::Parent,
                },
            ],
            care: vec![
                household::Caregiver {
                    person: 3,
                    dependent: 16,
                },
                household::Caregiver {
                    person: 4,
                    dependent: 17,
                },
            ],
        }),
        social: Some(social::Config::default()),
        lifecycle: Some(lifecycle::Config {
            people: (0..18)
                .map(|id| lifecycle::Seed {
                    birth_time: if id >= 16 { 0 } else { -24 * HOUR },
                    sex: if id % 2 == 0 {
                        lifecycle::Sex::Male
                    } else {
                        lifecycle::Sex::Female
                    },
                    drive: false,
                })
                .collect(),
            rules: Default::default(),
        }),
        military: Some(military::Config {
            enabled: true,
            requests: vec![military::Request {
                id: UNIT,
                posting: 1,
                muster: 1,
                attendance: 4,
                opens: 0,
                expires: 60 * HOUR,
                duration: 12 * HOUR,
                objective: military::Objective::ProtectCrossing,
            }],
        }),
        combat: Some(combat::Config {
            nodes: vec![0, 1, 2, 3, 4],
            raids: (9..12)
                .map(|person| combat::Raid {
                    person,
                    target: 2,
                    home: 4,
                    begins: 2 * HOUR,
                    quota: 10,
                })
                .collect(),
            strike_work: 15 * MINUTE,
            theft_work: 5 * MINUTE,
        }),
        ..Default::default()
    };
    let mut world = World::new(config)?;
    world.goods.add(
        Material::Food,
        200,
        Owner::Commons(0),
        Holder::Commons(2),
        0,
        48 * HOUR,
    );
    for id in 0..18 {
        if id >= 16 {
            world.agents[id].nutrition = 8 * HOUR;
        } else {
            world.goods.add(
                Material::Food,
                6,
                Owner::Agent(id),
                Holder::Agent(id),
                0,
                48 * HOUR,
            );
        }
    }
    // Remembered visits describe adjacency, never present remote stocks/people.
    for node in 0..world.config.nodes {
        let mut view = world.node_view(node);
        view.observed_at = -HOUR;
        view.food.clear();
        view.people.clear();
        view.board = None;
        for person in &mut world.agents {
            person.memory.nodes.insert(node, view.clone());
        }
    }
    world.log(None, "PlayableInitialConditions", format!(
        "scenario={scenario:?};18 existing Persons;F13A alternate physical parameters;old adjacency-only visits;seeded raiders/kin/goods;locally consented steward;voluntary assembled militia0,6,7,8 accepted ruler0 as local commander;courier5 agreed to at most2 physical round trips;no actor hold/no scheduled protection/no outcome intervention"
    ));
    world.apply_intents(vec![(
        RULER,
        Intent::AppointSteward {
            candidate: STEWARD,
            settlement: 1,
        },
    )])?;
    world.accept_office(STEWARD, STEWARD_OFFICE)?;
    world.observe_command_sites(RULER)?;
    // Explicit starting consent, executed through ordinary eligibility/admission.
    // These four people already exist and retain property, physiology and kin.
    for person in [RULER, 6, 7, 8] {
        world.observe_military(person);
        world.volunteer(person, UNIT)?;
        world.join_muster(person)?;
    }
    let report = world.capture_report(REPORT_COURIER)?;
    world.begin_report_delivery(
        REPORT_COURIER,
        report,
        RULER,
        if scenario == ScenarioKind::Remote {
            0
        } else {
            1
        },
    )?;
    world.audit()?;
    Ok(world)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_only_initial_knowledge_never_claims_empty_remote_contents() {
        use crate::provisioning::playable::{ViewMode, snapshot::KnowledgeSource};
        let mut session = Session::new(3, ScenarioKind::Local).unwrap();
        let initial = session.snapshot(ViewMode::Play);
        let remote = initial.nodes.iter().find(|node| node.id == 2).unwrap();
        assert!(!remote.contents_known);
        assert!(
            remote
                .inspect_lines
                .iter()
                .all(|line| !line.contains("0 people"))
        );
        assert!(
            initial
                .edges
                .iter()
                .filter(|edge| edge.evidence.source == KnowledgeSource::Memory)
                .all(|edge| edge.queued.is_none())
        );
        assert!(
            initial
                .nodes
                .iter()
                .find(|node| node.id == 1)
                .unwrap()
                .contents_known
        );
        let before = (session.world.state_hash(), session.world.event_hash());
        session.snapshot(ViewMode::Play);
        assert_eq!(
            (session.world.state_hash(), session.world.event_hash()),
            before
        );

        // A genuinely empty later observation is known empty, not another chart.
        // This is an explicit observation-state fixture, not a remote player action.
        let mut observed = session.world.node_view(2);
        observed.observed_at = 0;
        observed.food.clear();
        observed.people.clear();
        session.world.agents[RULER].memory.nodes.insert(2, observed);
        let later = session.snapshot(ViewMode::Play);
        let node = later.nodes.iter().find(|node| node.id == 2).unwrap();
        assert!(node.contents_known);
        assert_eq!(node.people_seen, 0);
    }

    #[test]
    fn ruler_remembers_own_local_handoff_without_reading_remote_outcome() {
        use crate::provisioning::playable::ViewMode;
        let mut session = Session::new(4, ScenarioKind::Local).unwrap();
        session
            .dispatch(PlayerCommand::Commission {
                office: 0,
                project: 0,
            })
            .unwrap();
        let initial = session.snapshot(ViewMode::Play);
        let order = initial.orders.iter().find(|order| order.id == 0).unwrap();
        assert_eq!(order.received_at, Some(0));
        assert_eq!(
            order.status,
            "Delivered locally; recipient decision unreported"
        );
        session.world.fulfil_order(STEWARD, 0).unwrap();
        let fulfilled = session.snapshot(ViewMode::Play);
        assert_eq!(fulfilled.orders[0], initial.orders[0]);
        assert!(fulfilled.crossings[0].authorized.unwrap());
        // The ruler can see the resulting local commission; that does not give
        // access to the steward's private obligation record.
        assert_eq!(
            session.world.command_memory(STEWARD).unwrap().obligations[&0].status,
            commands::Status::Completed
        );
    }

    #[test]
    fn courier_handoff_does_not_claim_remote_receipt() {
        use crate::provisioning::playable::ViewMode;
        let mut session = Session::new(5, ScenarioKind::Remote).unwrap();
        session
            .dispatch(PlayerCommand::Commission {
                office: 0,
                project: 0,
            })
            .unwrap();
        let snapshot = session.snapshot(ViewMode::Play);
        let order = &snapshot.orders[0];
        assert_eq!(order.received_at, None);
        assert!(order.status.starts_with("Handed to agreed courier"));
        assert!(order.status.ends_with("recipient status unknown"));
        session.advance_to(2 * HOUR).unwrap();
        let after_receipt = session.snapshot(ViewMode::Play);
        assert_eq!(after_receipt.orders[0], *order);
    }

    #[test]
    fn local_command_reaches_an_accepted_office_but_does_not_spawn_crossing() {
        let mut s = Session::new(11, ScenarioKind::Local).unwrap();
        let people = s.world.agents.len();
        assert_eq!(s.world.military_role(RULER), Some(UNIT));
        assert_eq!(s.world.units().get(&UNIT).unwrap().members.len(), 4);
        let result = s
            .dispatch(PlayerCommand::Commission {
                office: 0,
                project: 0,
            })
            .unwrap();
        assert!(matches!(
            result,
            CommandResult::Order {
                dispatch: DispatchState::DeliveredLocally,
                ..
            }
        ));
        assert_eq!(s.world.agents.len(), people);
        assert!(s.world.commissions().is_empty());
        assert_eq!(s.world.config.edges[1].capacity, 0);
        s.advance_to(2 * MINUTE).unwrap();
        assert_eq!(s.world.commissions().len(), 1);
        assert_eq!(s.world.config.edges[1].capacity, 0);
        s.world.audit().unwrap();
    }

    #[test]
    fn command_replay_includes_rejected_inputs_and_exact_hashes() {
        let mut s = Session::new(17, ScenarioKind::Local).unwrap();
        assert!(
            s.dispatch(PlayerCommand::Commission {
                office: 999,
                project: 0
            })
            .is_err()
        );
        s.dispatch(PlayerCommand::Commission {
            office: 0,
            project: 0,
        })
        .unwrap();
        s.advance_to(2 * MINUTE).unwrap();
        s.dispatch(PlayerCommand::Revoke { command: 0 }).unwrap();
        s.advance_to(4 * HOUR).unwrap();
        let replay = Session::replay(17, ScenarioKind::Local, s.command_log(), 4 * HOUR).unwrap();
        assert_eq!(s.world.state_hash(), replay.world.state_hash());
        assert_eq!(s.world.event_hash(), replay.world.event_hash());
        assert_eq!(s.command_log(), replay.command_log());
    }

    #[test]
    fn remote_order_starts_with_a_physical_courier_not_a_remote_obligation() {
        let mut s = Session::new(23, ScenarioKind::Remote).unwrap();
        assert_eq!(s.world.now(), HOUR / 4);
        assert_eq!(s.world.agents[RULER].location, Location::Node(0));
        assert_eq!(s.world.agents[ORDER_COURIER].location, Location::Node(0));
        let result = s
            .dispatch(PlayerCommand::Commission {
                office: 0,
                project: 0,
            })
            .unwrap();
        assert_eq!(
            result,
            CommandResult::Order {
                message: 0,
                dispatch: DispatchState::WithAgreedCourier
            }
        );
        assert!(
            s.world
                .command_memory(STEWARD)
                .unwrap()
                .obligations
                .is_empty()
        );
        assert!(
            s.world
                .command_memory(ORDER_COURIER)
                .unwrap()
                .envelopes
                .contains_key(&0)
        );
        let issued_at = s.world.now();
        s.advance_to(issued_at + MINUTE).unwrap();
        assert!(
            s.world
                .command_memory(STEWARD)
                .unwrap()
                .obligations
                .is_empty()
        );
        s.advance_to(2 * HOUR).unwrap();
        let handoff = s
            .world
            .command_journal()
            .transmissions
            .iter()
            .find(|h| h.message == 0 && h.recipient == STEWARD)
            .unwrap();
        assert!(handoff.received_at > issued_at + MINUTE);
        assert_eq!(handoff.sender, ORDER_COURIER);
        assert_eq!(handoff.node, 1);
    }

    #[test]
    fn local_unit_control_targets_existing_people_and_keeps_physical_travel() {
        let mut s = Session::new(29, ScenarioKind::Local).unwrap();
        assert_eq!(
            s.dispatch(PlayerCommand::Unit {
                verb: MilitaryOrder::Move,
                target: 0
            })
            .unwrap(),
            CommandResult::Unit { receivers: 4 }
        );
        for id in [RULER, 6, 7, 8] {
            assert_eq!(s.world.agents[id].location, Location::Node(1));
        }
        s.advance_to(HOUR / 8).unwrap();
        for id in [RULER, 6, 7, 8] {
            assert!(matches!(
                s.world.agents[id].location,
                Location::Edge { edge: 0, .. }
            ));
        }
        assert_eq!(s.world.agents.len(), 18);
        s.world.audit().unwrap();
    }

    #[test]
    fn arbitrary_reader_checkpoints_do_not_schedule_contacts() {
        for scenario in [ScenarioKind::Local, ScenarioKind::Remote] {
            let mut direct = Session::new(19, scenario).unwrap();
            let mut sampled = Session::new(19, scenario).unwrap();
            let input = PlayerCommand::Commission {
                office: 0,
                project: 0,
            };
            assert_eq!(direct.dispatch(input.clone()), sampled.dispatch(input));
            let start = direct.world.now();
            direct.advance_to(start + 6 * HOUR).unwrap();
            for offset in [
                1,
                MINUTE - 1,
                MINUTE + 1,
                17 * MINUTE + 1,
                HOUR + 1,
                3 * HOUR + 17,
                6 * HOUR,
            ] {
                sampled.advance_to(start + offset).unwrap();
                // Read-only render/headless consumers can take arbitrary reads.
                let _ = sampled.world().observe(sampled.ruler());
                let _ = sampled.world().event_hash();
            }
            assert_eq!(
                direct.world.state_hash(),
                sampled.world.state_hash(),
                "{scenario:?}"
            );
            assert_eq!(
                direct.world.event_hash(),
                sampled.world.event_hash(),
                "{scenario:?}"
            );
        }
    }
}
