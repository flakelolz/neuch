use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

pub fn world() -> (World, Collisions) {
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
        Reaction::default(),
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
        Reaction::default(),
    ));

    let hit_events: Vec<HitEvent> = Vec::new();
    let hit_events_id = world.spawn((hit_events,));
    let collisions = Collisions::new(hit_events_id);

    (world, collisions)
}

pub fn frame_count(world: &mut World) {
    world
        .query_mut::<&mut u32>()
        .into_iter()
        .for_each(|(_, frame)| {
            *frame += 1;
        });
}

#[derive(Debug, Clone, Copy)]
pub struct HitEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub properties: HitProperties,
}
