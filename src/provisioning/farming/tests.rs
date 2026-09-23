use super::*;
fn fixture(agents: usize) -> World {
    World::new(super::super::Config {
        nodes: 2,
        edges: vec![Edge {
            a: 0,
            b: 1,
            nominal_ms: HOUR,
            capacity: 4,
        }],
        starts: vec![0; agents],
        food: vec![],
        boards: vec![],
        communal: false,
        farming: Some(Config {
            units: vec![0],
            rules: Rules::default(),
        }),
        autonomous: false,
        trace: true,
        ..super::super::Config::default()
    })
    .unwrap()
}
fn work(w: &mut World, id: usize, plot: usize) {
    w.apply_intents(vec![(id, Intent::Cultivate { plot })])
        .unwrap();
    while matches!(w.agents[id].action, Action::Farm { .. }) {
        let next = w.next_time(w.now + HOUR);
        w.advance_to(next).unwrap();
    }
}
#[test]
fn ownership_begins_at_investment_excludes_others_and_ends_with_crop_not_land() {
    let mut w = fixture(2);
    w.apply_intents(vec![
        (1, Intent::Cultivate { plot: 0 }),
        (0, Intent::Cultivate { plot: 0 }),
    ])
    .unwrap();
    assert_eq!(w.plots()[0].crop.as_ref().unwrap().owner, Owner::Agent(0));
    assert_eq!(w.agents[1].action, Action::Idle);
    work(&mut w, 0, 0);
    w.advance_to(13 * HOUR).unwrap();
    w.apply_intents(vec![(1, Intent::Cultivate { plot: 0 })])
        .unwrap();
    assert_eq!(w.agents[1].action, Action::Idle);
    work(&mut w, 0, 0);
    assert_eq!(w.farm.produced, 24);
    assert!(w.plots()[0].crop.is_none());
    let batch = w.batches().last().unwrap().clone();
    assert_eq!(batch.owner, Owner::Agent(0));
    assert_eq!(batch.holder, Holder::Loose(0));
    w.apply_intents(vec![(
        1,
        Intent::Eat {
            batch: batch.id,
            quantity: 1,
        },
    )])
    .unwrap();
    assert_eq!(w.batches()[batch.id].quantity, 24);
    w.apply_intents(vec![(1, Intent::Cultivate { plot: 0 })])
        .unwrap();
    assert_eq!(w.plots()[0].crop.as_ref().unwrap().owner, Owner::Agent(1));
    assert_eq!(w.goods.initial[0], 0);
    w.audit().unwrap();
}
#[test]
fn interruption_retains_work_and_claim_but_releases_agent_action() {
    let mut w = fixture(2);
    w.apply_intents(vec![(0, Intent::Cultivate { plot: 0 })])
        .unwrap();
    w.advance_to(HOUR / 2).unwrap();
    let remaining = w.plots()[0].crop.as_ref().unwrap().remaining;
    w.apply_intents(vec![(0, Intent::CancelAction)]).unwrap();
    w.advance_to(HOUR).unwrap();
    assert_eq!(w.plots()[0].crop.as_ref().unwrap().remaining, remaining);
    work(&mut w, 0, 0);
    assert_eq!(w.now, HOUR + HOUR / 2);
    assert_eq!(w.farm.labor_ms, HOUR as u128);
    assert_eq!(w.farm.interruptions, 1);
    assert_eq!(w.farm.resumptions, 1);
}
#[test]
fn death_retains_mature_crop_and_other_agent_cannot_take_it() {
    let mut w = fixture(2);
    work(&mut w, 0, 0);
    w.advance_to(36 * HOUR).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert_eq!(w.plots()[0].crop.as_ref().unwrap().owner, Owner::Agent(0));
    assert_eq!(w.farming_counters()["inaccessible_mature"], 1);
    assert_eq!(w.farm.produced, 0);
    w.audit().unwrap();
}
#[test]
fn active_worker_death_does_not_finish_harvest_or_create_goods() {
    let mut w = fixture(1);
    work(&mut w, 0, 0);
    w.advance_to(35 * HOUR + 59 * MINUTE).unwrap();
    w.apply_intents(vec![(0, Intent::Cultivate { plot: 0 })])
        .unwrap();
    w.advance_to(36 * HOUR).unwrap();
    assert_eq!(w.agents[0].action, Action::Dead);
    assert_eq!(w.farm.produced, 0);
    assert!(w.plots()[0].crop.as_ref().unwrap().remaining > 0);
}
#[test]
fn harvest_age_is_preserved_through_split_carry_and_owner_death() {
    let mut w = fixture(1);
    work(&mut w, 0, 0);
    w.advance_to(13 * HOUR).unwrap();
    work(&mut w, 0, 0);
    let b = w.batches()[0].clone();
    w.apply_intents(vec![(
        0,
        Intent::Pickup {
            batch: 0,
            quantity: 10,
        },
    )])
    .unwrap();
    w.advance_to(b.expires_at() - 1).unwrap();
    assert!(
        w.batches()
            .iter()
            .filter(|b| b.quantity > 0)
            .all(|b| b.material == Material::Food)
    );
    w.advance_to(b.expires_at()).unwrap();
    assert!(
        w.batches()
            .iter()
            .filter(|b| b.quantity > 0)
            .all(|x| x.material == Material::Waste
                && x.created_at == b.created_at
                && x.owner == Owner::Agent(0))
    );
    assert_eq!(w.metrics.spoiled, 24);
    w.audit().unwrap();
}
#[test]
fn local_observation_does_not_update_remote_crop_memory() {
    let mut w = fixture(1);
    work(&mut w, 0, 0);
    w.observe_farming(0);
    let remembered = w.farm.memories[0].clone();
    w.apply_intents(vec![(
        0,
        Intent::Travel {
            edge: 0,
            target: 1,
            food: false,
        },
    )])
    .unwrap();
    w.advance_to(13 * HOUR).unwrap();
    w.observe_farming(0);
    assert_eq!(w.farm.memories[0][&0], remembered[&0]);
    assert!(w.farm.memories[0][&1].plots.is_empty());
    assert_eq!(w.plots()[0].crop.as_ref().unwrap().stage, Stage::Mature);
    assert!(matches!(
        w.farm.memories[0][&0].plots[0].crop.as_ref().unwrap().stage,
        Stage::Growing { .. }
    ));
}
#[test]
fn no_work_no_output_and_no_land_no_cultivation() {
    let mut w = fixture(1);
    w.advance_to(168 * HOUR).unwrap();
    assert_eq!(w.farm.produced, 0);
    assert_eq!(w.farm.started, 0);
    let mut c = w.config.clone();
    c.farming.as_mut().unwrap().units.clear();
    let mut w = World::new(c).unwrap();
    w.apply_intents(vec![(0, Intent::Cultivate { plot: 0 })])
        .unwrap();
    assert!(w.plots().is_empty());
    assert_eq!(w.agents[0].action, Action::Idle);
}
#[test]
fn frozen_forecast_matches_executable_labor_growth_physiology() {
    for n in [1, 6, 12, 24] {
        for f in [0, 12, 20, 24] {
            for grow in [HOUR, 12 * HOUR, 36 * HOUR] {
                let mut w = fixture(1);
                w.agents[0].nutrition = n * HOUR;
                w.agents[0].fatigue = f * HOUR;
                w.config.farming.as_mut().unwrap().rules.grow_ms = grow;
                let o = w.observe(0).unwrap();
                let predicted = policy::coherent::cultivation_forecast(&o, HOUR, grow, HOUR);
                work(&mut w, 0, 0);
                if let Some(Crop {
                    stage: Stage::Growing { ready_at },
                    ..
                }) = w.plots()[0].crop.clone()
                {
                    w.advance_to(ready_at).unwrap();
                    if w.agents[0].action != Action::Dead {
                        work(&mut w, 0, 0);
                    }
                }
                assert_eq!(
                    predicted.is_some(),
                    w.farm.harvested == 1,
                    "n={n} f={f} grow={grow}"
                );
                if let Some(t) = predicted {
                    assert_eq!(t, w.now, "n={n} f={f} grow={grow}");
                    assert_eq!(w.agents[0].nutrition, (o.nutrition - t).max(0));
                    assert_eq!(w.agents[0].fatigue, (o.fatigue + t).min(24 * HOUR));
                }
                w.audit().unwrap();
            }
        }
    }
}
#[test]
fn same_personal_history_same_decision_even_if_hidden_land_changes() {
    let mut a = fixture(1);
    a.config.autonomous = true;
    let mut c = a.config.clone();
    c.farming.as_mut().unwrap().units.push(1);
    let mut b = World::new(c).unwrap();
    a.advance_to(0).unwrap();
    b.advance_to(0).unwrap();
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn advance_partition_and_submission_order_do_not_change_crop_physics() {
    let mut a = fixture(2);
    let mut b = a.clone();
    a.apply_intents(vec![
        (0, Intent::Cultivate { plot: 0 }),
        (1, Intent::Cultivate { plot: 0 }),
    ])
    .unwrap();
    b.apply_intents(vec![
        (1, Intent::Cultivate { plot: 0 }),
        (0, Intent::Cultivate { plot: 0 }),
    ])
    .unwrap();
    a.advance_to(15 * HOUR).unwrap();
    for m in 1..=15 * 60 {
        b.advance_to(m * MINUTE).unwrap();
    }
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.event_hash(), b.event_hash());
}
#[test]
fn food_farming_world_is_not_specific_to_node_labels() {
    let c = super::super::Config {
        farming: Some(Config {
            units: vec![0; 4],
            rules: Rules::default(),
        }),
        starts: vec![0; 4],
        ..super::super::Config::default()
    };
    let mut a = World::new(c.clone()).unwrap();
    let mut b = World::new(geography::reversed_labels(c)).unwrap();
    for h in 0..=72 {
        a.advance_to(h * HOUR).unwrap();
        b.advance_to(h * HOUR).unwrap();
        assert_eq!(a.farming_counters(), b.farming_counters());
        assert_eq!(a.metrics, b.metrics);
        for (x, y) in a.agents.iter().zip(&b.agents) {
            assert_eq!(
                (x.nutrition, x.fatigue, x.starvation, x.consumed),
                (y.nutrition, y.fatigue, y.starvation, y.consumed)
            );
        }
    }
}
