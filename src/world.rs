use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

pub fn world() -> World {
    let mut world = World::new();
    // Player 1
    world.spawn((Input::default(), InputConfig::one(), Player::One));

    // Player 2
    world.spawn((Input::default(), InputConfig::two(), Player::Two));

    world
}
