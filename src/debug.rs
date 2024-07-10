#![allow(dead_code)]
use crate::prelude::*;

pub fn show_inputs(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world
        .query::<(&Input, &Player)>()
        .into_iter()
        .for_each(|(_, (input, player))| {
            if player == &Player::One {
                d.draw_text(&format!("{:#?}", input), 12, 24, 10, Color::WHITE);
            }
        });
}

pub fn show_position(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world
        .query::<(&Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (physics, player))| {
            if player == &Player::One {
                let (screen_x, screen_y) = pos_to_screen(physics.position);
                let (pos_x, pos_y) = world_to_screen(physics.position);
                d.draw_circle(screen_x, screen_y, 1., Color::WHITE);
                d.draw_text(
                    format!("{}:{}", pos_x, pos_y).as_str(),
                    screen_x,
                    screen_y + 2,
                    10,
                    Color::WHITE,
                );
            }
        });
}

pub fn show_state(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world
        .query::<(&StateMachine, &Physics)>()
        .into_iter()
        .for_each(|(_, (state, physics))| {
            let (screen_x, screen_y) = pos_to_screen(physics.position);
            let current = state.processor.current.as_ref();
            let timeline = state.context.elapsed;
            let duration = state.context.duration;

            d.draw_text(
                &current.name(),
                screen_x - 30,
                screen_y - 130,
                10,
                Color::WHITE,
            );
            d.draw_text(
                format!("{}", timeline).as_str(),
                screen_x - 10,
                screen_y - 120,
                10,
                Color::WHITE,
            );
            d.draw_text(
                format!("{}", duration).as_str(),
                screen_x - 30,
                screen_y - 120,
                10,
                Color::WHITE,
            );
        });
}

pub fn show_frame_count(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world.query::<&u32>().into_iter().for_each(|(_, frame)| {
        d.draw_text(&format!("{}", frame), 10, 10, 10, Color::WHITE);
    });
}

pub fn show_context(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world
        .query::<(&StateMachine, &Player)>()
        .into_iter()
        .for_each(|(_, (machine, player))| {
            if player == &Player::One {
                d.draw_text(
                    &format!("F: {}", machine.context.ctx.can_dash_f),
                    10,
                    20,
                    10,
                    Color::WHITE,
                );
                d.draw_text(
                    &format!("B: {}", machine.context.ctx.can_dash_b),
                    10,
                    30,
                    10,
                    Color::WHITE,
                );
                d.draw_text(
                    &format!("Jump: {:#?}", machine.context.ctx.flags.jump),
                    10,
                    40,
                    10,
                    Color::WHITE,
                );
            }
        });
}

pub fn reset_position(world: &mut World, rl: &mut RaylibHandle) {
    if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        world
            .query_mut::<(&mut Physics, &Player)>()
            .into_iter()
            .for_each(|(_, (physics, player))| {
                if player == &Player::One {
                    *physics = Physics::one();
                }
            });
    }
}

pub fn forced_move(world: &mut World, rl: &mut RaylibHandle) {
    world
        .query_mut::<(&mut Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (physics, player))| {
            if player == &Player::One {
                if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                    physics.position.x -= 10000;
                }

                if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                    physics.position.x += 10000;
                }

                if rl.is_key_down(KeyboardKey::KEY_UP) {
                    physics.position.y -= 10000;
                }

                if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                    physics.position.y += 10000;
                }
            }
        });
}
