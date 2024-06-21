use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

pub fn world() -> World {
    let mut world = World::new();
    // Global
    world.spawn((GameData::new(),));

    // Player 1
    let _player1 = world.spawn((
        Player::One,
        Input::default(),
        InputConfig::one(),
        Physics::one(),
        StateMachine::default(),
    ));

    // Player 2
    let _player2 = world.spawn((
        Player::Two,
        Input::default(),
        InputConfig::two(),
        Physics::two(),
        // StateMachine::default(),
    ));

    world
}
