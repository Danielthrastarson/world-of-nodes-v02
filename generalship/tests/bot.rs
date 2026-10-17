use generalship_g0::{bot::Bot, model::*, sim::Game};

fn running() -> Game {
    let mut game = Game::new(4, Config::default());
    for side in 0..2 {
        game.input(side, Input::Connection { connected: true })
            .unwrap();
        game.input(side, Input::Ready).unwrap();
    }
    game
}

#[test]
fn hidden_enemy_changes_cannot_change_bot_decisions() {
    let a = running();
    let mut b = a.clone();
    for person in b.people.iter_mut().filter(|p| p.side == 0) {
        person.pos = Point::new(1000, 1000);
        person.hp = 1;
        person.plan.target = Point::new(70000, 10000);
    }
    let mut bot_a = Bot::default();
    let mut bot_b = Bot::default();
    assert_eq!(a.view(1), b.view(1));
    assert_eq!(
        serde_json::to_value(bot_a.decide(a.view(1))).unwrap(),
        serde_json::to_value(bot_b.decide(b.view(1))).unwrap()
    );
}

#[test]
fn practice_bot_moves_fights_and_replays_with_conserved_resources() {
    let mut game = running();
    let mut bot = Bot::default();
    let mut orders = 0;
    let mut crossed = false;
    let mut took_ford = false;
    for _ in 0..6000 {
        if game.status != Status::Running {
            break;
        }
        for input in bot.decide(game.view(1)) {
            if matches!(input, Input::Order { .. }) {
                orders += 1;
            }
            game.input(1, input).unwrap();
        }
        game.step();
        let spacing2 = i64::from(game.config.person_radius * 2).pow(2);
        for a in game.people.iter().filter(|p| p.alive()) {
            for b in game.people.iter().filter(|p| p.alive() && p.id > a.id) {
                assert!(
                    a.pos.distance2(b.pos) >= spacing2,
                    "tick {}: physical overlap between {} and {}",
                    game.tick,
                    a.id,
                    b.id
                );
            }
        }
        crossed |= game
            .people
            .iter()
            .any(|p| p.side == 1 && p.role == Role::Line && p.pos.x < 44000);
        took_ford |= game
            .people
            .iter()
            .any(|p| p.side == 1 && p.role == Role::Light && p.pos.x < 46000 && p.pos.y > 47000);
    }
    assert!(orders >= 3);
    assert!(crossed, "The bot must actually cross the bridge");
    assert!(took_ford, "The light formation must actually use the ford");
    assert!(
        game.people
            .iter()
            .any(|p| p.side == 0 && p.hp < game.config.person_hp),
        "A passive human should face an actual attack"
    );
    game.audit().unwrap();
    assert_eq!(
        Game::replay_run(&game.replay()).unwrap().hash(),
        game.hash()
    );
}
