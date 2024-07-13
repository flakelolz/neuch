#![allow(dead_code)]
use crate::prelude::*;

const TEXT_SIZE: i32 = 10;

pub fn show_inputs(world: &World, d: &mut impl RaylibDraw) {
    world
        .query::<(&Input, &Player)>()
        .into_iter()
        .for_each(|(_, (input, player))| {
            if player == &Player::One {
                d.draw_text(&format!("{:#?}", input), 10, 100, 10, Color::WHITE);
            }
        });
}

pub fn show_position(world: &World, d: &mut impl RaylibDraw) {
    world
        .query::<(&Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (physics, player))| {
            let (screen_x, screen_y) = pos_to_screen(physics.position);
            let (screen_x, screen_y) = sprite_to_native(screen_x, screen_y);
            let (pos_x, pos_y) = world_to_screen(physics.position);
            d.draw_circle(screen_x, screen_y, 1., Color::WHITE);
            if player == &Player::One {
                d.draw_text(
                    format!("{}:{}", pos_x, pos_y).as_str(),
                    screen_x,
                    screen_y + 2,
                    TEXT_SIZE,
                    Color::WHITE,
                );
            }
        });
}

pub fn show_state(world: &World, d: &mut impl RaylibDraw) {
    world
        .query::<(&StateMachine, &Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (state, physics, player))| {
            if player == &Player::One {
                let (screen_x, screen_y) = pos_to_screen(physics.position);
                let (screen_x, screen_y) = sprite_to_native(screen_x, screen_y);
                let current = state.processor.current.as_ref();
                let timeline = state.context.elapsed;
                let duration = state.context.duration;
                let top = 200;
                let offset = 10;

                d.draw_text(
                    &current.name(),
                    screen_x - 30,
                    screen_y - top,
                    TEXT_SIZE,
                    Color::WHITE,
                );
                d.draw_text(
                    format!("{}", duration).as_str(),
                    screen_x - 30,
                    screen_y - top + offset,
                    TEXT_SIZE,
                    Color::WHITE,
                );
                d.draw_text(
                    format!("{}", timeline).as_str(),
                    screen_x,
                    screen_y - top + offset,
                    TEXT_SIZE,
                    Color::WHITE,
                );
            }
        });
}

pub fn show_frame_count(world: &World, d: &mut impl RaylibDraw) {
    let y = 10;
    world.query::<&u32>().into_iter().for_each(|(_, frame)| {
        d.draw_text(&format!("{}", frame), 10, y, TEXT_SIZE, Color::WHITE);
    });
}

pub fn show_context(world: &World, d: &mut impl RaylibDraw) {
    let y = 30;
    world
        .query::<(&StateMachine, &Player)>()
        .into_iter()
        .for_each(|(_, (machine, player))| {
            if player == &Player::One {
                d.draw_text(
                    &format!("F: {}", machine.context.ctx.can_dash_f),
                    10,
                    y,
                    TEXT_SIZE,
                    Color::WHITE,
                );
                d.draw_text(
                    &format!("B: {}", machine.context.ctx.can_dash_b),
                    10,
                    y + TEXT_SIZE,
                    TEXT_SIZE,
                    Color::WHITE,
                );
                d.draw_text(
                    &format!("Jump: {:#?}", machine.context.ctx.flags.jump),
                    10,
                    y + TEXT_SIZE * 2,
                    TEXT_SIZE,
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
            .for_each(|(_, (physics, player))| match player {
                Player::One => *physics = Physics::one(),
                Player::Two => *physics = Physics::two(),
            });
    }
}

pub fn change_resolution(rl: &mut RaylibHandle, configs: &mut Configs, camera: &mut Camera2D) {
    if rl.is_key_pressed(KeyboardKey::KEY_F1) {
        configs.display.set_360(rl, camera);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_F2) {
        configs.display.set_720(rl, camera);
    }
}
