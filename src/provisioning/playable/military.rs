//! P2 local unit control over existing Persons, not a remote military controller.
use crate::provisioning::{Action, Location, World};
use std::collections::BTreeSet;

/// Explicit scenario initial consent by the unit's voluntary members.
/// A session holds this agreement; selecting a unit does not create one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MilitaryAgreement {
    pub commander: usize,
    pub unit: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MilitaryOrder {
    Move,
    Hold,
    Attack,
    Withdraw,
}

/// Issue one locally heard objective. The caller cannot supply the issue node,
/// routes, recipients, injury, cargo or arrival. Agreement scope is one unit.
pub fn issue(
    world: &mut World,
    agreement: &MilitaryAgreement,
    order: MilitaryOrder,
    target: usize,
) -> Result<usize, String> {
    let commander = world
        .agents
        .get(agreement.commander)
        .ok_or("unknown commander")?;
    if commander.action == Action::Dead || world.body_factor(commander.id) == 0 {
        return Err("living capable commander required".into());
    }
    let Location::Node(node) = commander.location else {
        return Err("orders require physical local contact, not mid-edge transmission".into());
    };
    if world.military_role(commander.id) != Some(agreement.unit)
        || !world
            .unit_view(commander.id)
            .is_some_and(|view| view.active && view.unit == Some(agreement.unit))
        || !world
            .military_memory(commander.id)
            .and_then(|memory| memory.undertaking.as_ref())
            .is_some_and(|service| service.until.is_some_and(|until| until > world.now))
    {
        return Err("commander lacks active voluntary service in the agreed unit".into());
    }
    let known = target == node
        || commander.memory.nodes.contains_key(&target)
        || commander.memory.nodes.values().any(|seen| {
            seen.edges
                .iter()
                .any(|edge| edge.edge.a == target || edge.edge.b == target)
        });
    if !known {
        return Err("target is not in the commander's known geography".into());
    }
    if order == MilitaryOrder::Hold && target != node {
        return Err(
            "Hold applies at the commander's current node; Move or Attack approaches a location"
                .into(),
        );
    }
    if order == MilitaryOrder::Withdraw && target == node {
        return Err("withdrawal needs a different known destination".into());
    }
    if matches!(order, MilitaryOrder::Hold | MilitaryOrder::Attack) && world.config.combat.is_none()
    {
        return Err("local contact mechanics are unavailable".into());
    }
    // This is exactly the local recipient set of the established F11B endpoint.
    // There is no unit-wide stance change and no update to absent members.
    let recipients: Vec<_> = world
        .units()
        .get(&agreement.unit)
        .ok_or("unknown agreed unit")?
        .members
        .keys()
        .copied()
        .filter(|&id| {
            world.military_role(id) == Some(agreement.unit)
                && world.agents[id].location == Location::Node(node)
        })
        .collect();
    let old_queues: BTreeSet<_> = recipients
        .iter()
        .copied()
        .filter(|&id| {
            matches!(world.agents[id].action, Action::Queued { .. })
                && world
                    .unit_view(id)
                    .filter(|view| view.phase == "WITH_UNIT")
                    .and_then(|view| view.order.as_ref())
                    .is_some_and(|old| world.agents[id].goal == Some((old.target, false)))
        })
        .collect();
    let count = if matches!(order, MilitaryOrder::Hold | MilitaryOrder::Attack) {
        // Attack means approach the known location and engage already-recognized
        // hostile Persons there. It never designates arbitrary citizens hostile.
        world.issue_unit_protect(agreement.unit, node, target)?
    } else {
        world.issue_unit_move(agreement.unit, node, target)?
    };
    for &id in &recipients {
        world.playable_retreats.remove(&id);
        if order == MilitaryOrder::Withdraw {
            let serial = world.formation.views[id].order.as_ref().unwrap().serial;
            world.playable_retreats.insert(id, serial);
            // Retain combat allegiance. Clearing protect would grant immunity
            // while the withdrawing Person remains in physical contact.
        }
        if old_queues.contains(&id)
            || (matches!(order, MilitaryOrder::Move | MilitaryOrder::Withdraw)
                && matches!(world.agents[id].action, Action::Strike { .. }))
        {
            world.cancel_action(id, "locally received replacement unit objective");
        }
    }
    world.log(
        Some(agreement.commander),
        "PlayerUnitOrder",
        format!(
            "unit={}; order={order:?}; node={node}; target={target}; local_recipients={recipients:?}; voluntary command agreement",
            agreement.unit
        ),
    );
    world.audit()?;
    Ok(count)
}

impl World {
    /// Only this person's received order and service determine its retreat state.
    pub(crate) fn playable_withdrawing(&self, id: usize) -> bool {
        let Some(&serial) = self.playable_retreats.get(&id) else {
            return false;
        };
        let Some(person) = self.agents.get(id) else {
            return false;
        };
        person.action != Action::Dead
            && self.unit_view(id).is_some_and(|view| {
                view.active
                    && view.order.as_ref().is_some_and(|order| {
                        order.serial == serial && person.location != Location::Node(order.target)
                    })
            })
            && self
                .military_memory(id)
                .and_then(|memory| memory.undertaking.as_ref())
                .is_some_and(|service| service.until.is_some_and(|until| until > self.now))
    }

    pub(crate) fn refresh_playable_retreats(&mut self) {
        let ended: Vec<_> = self
            .playable_retreats
            .keys()
            .copied()
            .filter(|&id| !self.playable_withdrawing(id))
            .collect();
        for id in ended {
            self.playable_retreats.remove(&id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provisioning::{HOUR, Intent, MINUTE, combat, formation, social};

    fn ready(case: &str) -> World {
        let mut world = formation::research::fixture(case, 0).unwrap();
        formation::research::prepare_command(&mut world, case).unwrap();
        world
    }

    fn agreement(commander: usize) -> MilitaryAgreement {
        MilitaryAgreement { commander, unit: 0 }
    }

    #[test]
    fn local_move_uses_existing_persons_and_physics() {
        let mut world = ready("short");
        let ids: Vec<_> = world.agents.iter().map(|person| person.id).collect();
        assert_eq!(
            issue(&mut world, &agreement(0), MilitaryOrder::Move, 1).unwrap(),
            4
        );
        world.advance_to(4 * HOUR).unwrap();
        assert_eq!(world.unit_snapshot(0).unwrap().arrived, 4);
        assert_eq!(
            world
                .agents
                .iter()
                .map(|person| person.id)
                .collect::<Vec<_>>(),
            ids
        );
        assert!(world.playable_retreats.is_empty());
        assert_eq!(world.metrics.overtakes, 0);
        world.audit().unwrap();
    }

    #[test]
    fn invalid_or_unknown_commands_do_not_change_the_world() {
        let mut world = ready("unknown_route");
        for (commander, unit, order, target) in [
            (0, 0, MilitaryOrder::Move, 2),
            (0, 1, MilitaryOrder::Move, 1),
            (99, 0, MilitaryOrder::Move, 1),
            (0, 0, MilitaryOrder::Hold, 1),
            (0, 0, MilitaryOrder::Withdraw, 0),
        ] {
            let before = (world.state_hash(), world.event_hash());
            assert!(
                issue(
                    &mut world,
                    &MilitaryAgreement { commander, unit },
                    order,
                    target
                )
                .is_err()
            );
            assert_eq!((world.state_hash(), world.event_hash()), before);
        }
        // Naming a known destination does not supply the recipients a route.
        let map = world.agents[0].memory.nodes[&0].clone();
        world.agents[0].memory.nodes.insert(
            2,
            crate::provisioning::NodeView {
                node: 2,
                edges: vec![],
                ..map
            },
        );
        let before = world.agents[1].memory.clone();
        issue(&mut world, &agreement(0), MilitaryOrder::Move, 2).unwrap();
        assert_eq!(world.agents[1].memory, before);
        world.advance_to(4 * HOUR).unwrap();
        assert_eq!(world.unit_snapshot(0).unwrap().departed, 0);
        assert!(world.formation.journal.count("PAUSED_NO_KNOWN_ROUTE") > 0);
    }

    #[test]
    fn absent_late_members_do_not_receive_the_command_or_stance() {
        let mut world = ready("late");
        let commander = world
            .agents
            .iter()
            .find(|person| {
                person.location == Location::Node(1) && world.military_role(person.id) == Some(0)
            })
            .unwrap()
            .id;
        let absent: Vec<_> = world
            .agents
            .iter()
            .filter(|p| p.location != Location::Node(1))
            .map(|p| p.id)
            .collect();
        assert_eq!(
            issue(&mut world, &agreement(commander), MilitaryOrder::Move, 2).unwrap(),
            2
        );
        for &id in &absent {
            assert!(world.unit_view(id).unwrap().order.is_none());
        }
        world.advance_to(6 * HOUR).unwrap();
        for id in absent {
            assert!(world.unit_view(id).unwrap().order.is_none());
        }
    }

    fn contact() -> World {
        let mut world = combat::research::fixture("local", true, true, 0).unwrap();
        combat::research::prepare(&mut world, true).unwrap();
        world.config.autonomous = false;
        // Controlled static contact fixture, never a claim of autonomous travel.
        for id in 0..9 {
            world.agents[id].location = Location::Node(2);
            world.agents[id].action = Action::Idle;
            world.agents[id].goal = None;
        }
        world.advance_to(2 * HOUR).unwrap();
        world
    }

    #[test]
    fn withdrawal_stops_own_strike_but_does_not_grant_immunity() {
        let mut world = contact();
        world
            .apply_intents(vec![
                (0, Intent::Strike { target: 6 }),
                (6, Intent::Strike { target: 0 }),
            ])
            .unwrap();
        let enemy_action = world.agents[6].action.clone();
        let food_before: u64 = world.goods.batches.iter().map(|b| b.quantity).sum();
        assert_eq!(
            issue(&mut world, &agreement(0), MilitaryOrder::Withdraw, 0).unwrap(),
            4
        );
        assert_eq!(world.agents[0].action, Action::Idle);
        assert_eq!(world.agents[6].action, enemy_action);
        assert_eq!(world.agents[0].location, Location::Node(2));
        assert!(world.combat.views[0].protect.is_some());
        world.observe_combat(0);
        assert!(matches!(
            social::research::decision(&world, 0).intent,
            Intent::Travel { target: 0, .. }
        ));
        // With autonomous execution paused, the retreating person is still
        // physically here when the already-started enemy contact completes.
        world.advance_to(2 * HOUR + 15 * MINUTE).unwrap();
        assert_eq!(world.body(0).unwrap().injuries, 1);
        assert_eq!(world.body(6).unwrap().injuries, 0);
        assert_eq!(
            world.goods.batches.iter().map(|b| b.quantity).sum::<u64>(),
            food_before
        );
        world.audit().unwrap();
    }

    #[test]
    fn withdrawing_people_travel_and_resume_contact_only_after_arrival_or_new_order() {
        let mut world = contact();
        issue(&mut world, &agreement(0), MilitaryOrder::Withdraw, 0).unwrap();
        for id in 0..4 {
            world
                .apply_intents(vec![(
                    id,
                    Intent::Travel {
                        edge: 0,
                        target: 0,
                        food: false,
                    },
                )])
                .unwrap();
        }
        assert!(world.playable_withdrawing(0));
        world.advance_to(4 * HOUR).unwrap();
        assert_eq!(world.agents[0].location, Location::Node(0));
        assert!(world.playable_retreats.is_empty());
        assert_eq!(
            issue(&mut world, &agreement(0), MilitaryOrder::Hold, 0).unwrap(),
            4
        );
        assert_eq!(world.combat.views[0].protect, Some(0));
        assert_eq!(
            issue(&mut world, &agreement(0), MilitaryOrder::Attack, 2).unwrap(),
            4
        );
        assert_eq!(world.combat.views[0].protect, Some(2));
        world.audit().unwrap();
    }

    #[test]
    fn replacement_cancels_only_old_local_unit_queue_not_atomic_travel() {
        let mut world = ready("bottleneck");
        issue(&mut world, &agreement(0), MilitaryOrder::Move, 1).unwrap();
        world.advance_to(HOUR + MINUTE).unwrap();
        let travelling = world
            .agents
            .iter()
            .find(|p| p.action == Action::Travel)
            .unwrap()
            .id;
        let queued = world
            .agents
            .iter()
            .find(|p| matches!(p.action, Action::Queued { .. }))
            .unwrap()
            .id;
        let old_location = world.agents[travelling].location.clone();
        let old_order = world.unit_view(travelling).unwrap().order.clone();
        let before = (world.state_hash(), world.event_hash());
        assert!(issue(&mut world, &agreement(travelling), MilitaryOrder::Move, 0).is_err());
        assert_eq!((world.state_hash(), world.event_hash()), before);
        // A local commander can redirect those still waiting, never the member
        // already physically inside the edge. Use Move because this fixture has
        // no combat institution and Hold intentionally requires one.
        issue(&mut world, &agreement(queued), MilitaryOrder::Move, 0).unwrap();
        assert_eq!(world.agents[queued].action, Action::Idle);
        assert_eq!(world.agents[travelling].action, Action::Travel);
        assert_eq!(world.agents[travelling].location, old_location);
        assert_eq!(world.unit_view(travelling).unwrap().order, old_order);
        world.audit().unwrap();
    }

    #[test]
    fn incapacity_and_expired_service_prevent_issuance_and_end_retreat() {
        let mut world = contact();
        world.combat.bodies[0].injuries = 2;
        let before = (world.state_hash(), world.event_hash());
        assert!(issue(&mut world, &agreement(0), MilitaryOrder::Attack, 0).is_err());
        assert_eq!((world.state_hash(), world.event_hash()), before);
        world.combat.bodies[0].injuries = 0;
        issue(&mut world, &agreement(0), MilitaryOrder::Withdraw, 0).unwrap();
        let until = world.now + MINUTE;
        for id in 0..4 {
            world.military.memories[id]
                .undertaking
                .as_mut()
                .unwrap()
                .until = Some(until);
        }
        world.advance_to(until).unwrap();
        assert!(world.playable_retreats.is_empty());
        assert!(issue(&mut world, &agreement(0), MilitaryOrder::Move, 0).is_err());
    }

    #[test]
    fn withdrawal_preserves_immediate_person_survival_priorities() {
        let mut world = contact();
        issue(&mut world, &agreement(0), MilitaryOrder::Withdraw, 0).unwrap();
        // Controlled physiology checkpoints exercise the common Person arbiter.
        world.agents[0].fatigue = 22 * HOUR;
        assert_eq!(social::research::decision(&world, 0).intent, Intent::Sleep);
        world.agents[0].nutrition = 0;
        assert!(matches!(
            social::research::decision(&world, 0).intent,
            Intent::Eat { .. }
        ));
        assert!(world.playable_withdrawing(0));
    }

    #[test]
    fn withdrawal_does_not_depend_on_observer_time_partitions() {
        let mut once = contact();
        issue(&mut once, &agreement(0), MilitaryOrder::Withdraw, 0).unwrap();
        once.config.autonomous = true;
        once.next_policy = once.now;
        let mut sampled = once.clone();
        once.advance_to(6 * HOUR).unwrap();
        for minute in 121..=360 {
            sampled.advance_to(minute * MINUTE).unwrap();
        }
        let first_difference = once
            .events
            .iter()
            .zip(&sampled.events)
            .find(|(a, b)| a != b);
        assert_eq!(
            once.state_hash(),
            sampled.state_hash(),
            "first event difference: {first_difference:?}"
        );
        assert_eq!(once.event_hash(), sampled.event_hash());
    }
}
