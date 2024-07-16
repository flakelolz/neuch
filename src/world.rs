use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

pub fn world() -> (World, Collisions, Vec<HitEvent>) {
    let mut world = World::new();

    world.spawn((0u32,)); // Frame count

    // Player 1 components
    let character = Character::ken();
    let origin = character.data.origin;
    let _player1 = world.spawn((
        Player::One,
        Input::default(),
        InputBuffer::default(),
        InputConfig::one(),
        Physics::one(),
        StateMachine::default(),
        character,
        Animator::new(origin, 1, false),
    ));

    // Player 2 components
    let character = Character::ken();
    let origin = character.data.origin;
    let _player2 = world.spawn((
        Player::Two,
        Input::default(),
        InputBuffer::default(),
        InputConfig::two(),
        Physics::two(),
        StateMachine::default(),
        character,
        Animator::new(origin, 0, true),
    ));

    let hit_events: Vec<HitEvent> = Vec::new();
    let collisions = Collisions::default();

    (world, collisions, hit_events)
}

pub fn frame_count(world: &mut World) {
    world
        .query_mut::<&mut u32>()
        .into_iter()
        .for_each(|(_, frame)| {
            *frame += 1;
        });
}
