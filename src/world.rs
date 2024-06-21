use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

pub fn world() -> World {
    let mut world = World::new();

    // GameData components
    // let character_data = CharacterData::load("assets/data/data.json");
    // let action_map = generate_action_map(&character_data);
    //
    // let mut gamedata = GameData::new();
    // gamedata.add_character_data(character_data);
    // gamedata.add_action_map(action_map);

    // Global components
    // world.spawn((gamedata,));
    world.spawn((0,)); // Frame count

    // Player 1 components
    let _player1 = world.spawn((
        Player::One,
        Input::default(),
        InputConfig::one(),
        Physics::one(),
        StateMachine::default(),
        Character::ken()
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
