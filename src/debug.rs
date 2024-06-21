#![allow(dead_code)]
use crate::prelude::*;

pub fn show_inputs(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world
        .query::<(&Input, &Player)>()
        .into_iter()
        .for_each(|(_, (input, player))| {
            if player == &Player::One {
                d.draw_text(&format!("{:#?}", input), 12, 12, 20, Color::WHITE);
            }
        });
}

pub fn show_position(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world
        .query::<(&Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (physics, player))| {
            if player == &Player::One {
                let (x, y) = world_to_screen_vec(physics.position);
                d.draw_circle(x, y, 1., Color::WHITE);
                d.draw_text(format!("{}", x).as_str(), x - 30, y - 140, 10, Color::WHITE);
            }
        });
}

pub fn show_state(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world
        .query::<(&StateMachine, &Physics)>()
        .into_iter()
        .for_each(|(_, (state, physics))| {
            let (x, y) = world_to_screen_vec(physics.position);
            let state = state.processor.current.as_ref();

            d.draw_text(state.name(), x - 30, y - 130, 10, Color::WHITE);
        });
}

pub fn show_frame_count(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world.query::<&i32>().into_iter().for_each(|(_, frame)| {
        d.draw_text(&format!("{}", frame), 10, 10, 10, Color::WHITE);
    });
}
