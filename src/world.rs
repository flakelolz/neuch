use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

pub fn world() -> World {
    let mut world = World::new();

    world.spawn((0,)); // Frame count

    // Player 1 components
    let _player1 = world.spawn((
        Player::One,
        Input::default(),
        InputConfig::one(),
        Physics::one(),
        StateMachine::default(),
        Character::ken(),
        Animator::default(),
    ));

    // Player 2 components
    let _player2 = world.spawn((
        Player::Two,
        Input::default(),
        InputConfig::two(),
        Physics::two(),
        // StateMachine::default(),
    ));

    world
}

pub fn frame_count(world: &mut World) {
    world
        .query_mut::<&mut i32>()
        .into_iter()
        .for_each(|(_, frame)| {
            *frame += 1;
        });
}
