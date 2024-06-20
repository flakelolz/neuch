use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

pub fn world() -> World {
    let mut world = World::new();

    // Player 1
    world.spawn((Player::One, Input::default(), InputConfig::one()));

    // Player 2
    world.spawn((Player::Two, Input::default(), InputConfig::two()));

    world
}
