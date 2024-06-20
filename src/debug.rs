#![allow(dead_code)]
use crate::prelude::*;

pub fn show_inputs(world: &World, d: &mut RaylibDrawHandle) {
    world
        .query::<(&Input, &Player)>()
        .into_iter()
        .for_each(|(_, (input, player))| {
            if player == &Player::One {
                d.draw_text(&format!("{:#?}", input), 12, 12, 20, Color::WHITE);
            }
        });
}
