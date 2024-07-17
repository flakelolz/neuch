#![allow(dead_code)]
use crate::prelude::*;

const TEXT_SIZE: i32 = 10;
const SCREEN_CENTER: i32 = WIDTH / 2;

pub fn show_position(world: &World, d: &mut impl RaylibDraw) {
    world
        .query::<(&Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (physics, player))| {
            let (screen_x, screen_y) = pos_to_screen(physics.position);
            let (screen_x, screen_y) = sprite_to_ui(screen_x, screen_y);
            let (pos_x, pos_y) = world_to_screen(physics.position);
            d.draw_circle(screen_x, screen_y, 1., Color::WHITE);
            d.draw_text(
                format!("{}:{}", pos_x, pos_y).as_str(),
                screen_x,
                screen_y + 2,
                TEXT_SIZE,
                Color::WHITE,
            );
        });
}

pub fn show_state(world: &World, d: &mut impl RaylibDraw) {
    world
        .query::<(&StateMachine, &Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (state, physics, player))| {
            if player == &Player::One {
                let (screen_x, screen_y) = pos_to_screen(physics.position);
                let (screen_x, screen_y) = sprite_to_ui(screen_x, screen_y);
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

pub fn show_hitboxes(world: &World, d: &mut impl RaylibDraw) {
    for (_, (character, physics, state)) in world
        .query::<(&Character, &Physics, &StateMachine)>()
        .iter()
    {
        if let Some(action) = find_action(character, &state.processor.current.name()) {
            if let Some(hitboxes) = &action.hitboxes {
                let offset = physics.position;
                for hitbox in hitboxes.iter() {
                    let translated = if physics.facing_left {
                        hitbox.value.translate_flipped(offset)
                    } else {
                        hitbox.value.translate(offset)
                    };

                    let left = world_to_sprite_to_ui_num(translated.left);
                    let top = world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
                    let width = world_to_sprite_to_ui_num(translated.right - translated.left);
                    let height = world_to_sprite_to_ui_num(translated.top - translated.bottom);
                    if hitbox.is_active(state.context.elapsed) {
                        d.draw_rectangle_lines(left, top, width, height, Color::RED);
                    }
                }
            }
        }
    }
}

pub fn show_hurtboxes(world: &World, d: &mut impl RaylibDraw) {
    for (_, (character, physics, state)) in world
        .query::<(&Character, &Physics, &StateMachine)>()
        .iter()
    {
        if let Some(action) = find_action(character, &state.processor.current.name()) {
            if let Some(hurtboxes) = &action.hurtboxes {
                for hurtbox in hurtboxes.iter() {
                    let offset = physics.position;
                    let translated = if physics.facing_left {
                        hurtbox.value.translate_flipped(offset)
                    } else {
                        hurtbox.value.translate(offset)
                    };

                    let left = world_to_sprite_to_ui_num(translated.left);
                    let top = world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
                    let width = world_to_sprite_to_ui_num(translated.right - translated.left);
                    let height = world_to_sprite_to_ui_num(translated.top - translated.bottom);
                    if hurtbox.is_active(state.context.elapsed) {
                        d.draw_rectangle_lines(left, top, width, height, Color::BLUE);
                    }
                }
            }
        }
    }
}

pub fn show_pushboxes(world: &World, d: &mut impl RaylibDraw) {
    for (_, (character, physics, state)) in world
        .query::<(&Character, &Physics, &StateMachine)>()
        .iter()
    {
        if let Some(action) = find_action(character, &state.processor.current.name()) {
            let offset = physics.position;

            if let Some(pushboxes) = &action.pushboxes {
                for pushbox in pushboxes.iter() {
                    let translated = if physics.facing_left {
                        pushbox.value.translate_flipped(offset)
                    } else {
                        pushbox.value.translate(offset)
                    };

                    let left = world_to_sprite_to_ui_num(translated.left);
                    let top = world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
                    let width = world_to_sprite_to_ui_num(translated.right - translated.left);
                    let height = world_to_sprite_to_ui_num(translated.top - translated.bottom);
                    if pushbox.is_active(state.context.elapsed) {
                        d.draw_rectangle_lines(left, top, width, height, Color::PURPLE);
                    }
                }
            } else {
                // Default pushbox
                let translated = if physics.facing_left {
                    character.info.pushbox.translate_flipped(offset)
                } else {
                    character.info.pushbox.translate(offset)
                };

                let left = world_to_sprite_to_ui_num(translated.left);
                let top = world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
                let width = world_to_sprite_to_ui_num(translated.right - translated.left);
                let height = world_to_sprite_to_ui_num(translated.top - translated.bottom);
                d.draw_rectangle_lines(left, top, width, height, Color::PURPLE);
            }
        }
    }
}

#[rustfmt::skip]
pub fn show_inputs(world: &World, d: &mut impl RaylibDraw) {
    world
        .query::<(&Input, &Physics, &Player)>()
        .iter()
        .for_each(|(_, (input, physics, player))| {
            let dir_size = 20.;
            let size = 10.;
            let pos = 75;
            let y = HEIGHT - 40;
            let font = d.gui_get_font();

            let left = {
                if physics.facing_left {
                    input.forward
                } else {
                    input.backward
                }
            };
            let right = {
                if physics.facing_left {
                    input.backward
                } else {
                    input.forward
                }
            };

            if player == &Player::One {
                // Up
                match input.up {
                    true => d.draw_text_pro( &font, ">", rvec2(40, y - 4), rvec2(0., 0.), 270., dir_size, 0., Color::WHITE),
                    false => d.draw_text_pro( &font, ">", rvec2(40, y - 4), rvec2(0., 0.), 270., dir_size, 0., Color::DARKGRAY),
                }
                // Down
                match input.down {
                    true => d.draw_text_pro( &font, ">", rvec2(58, y + 13), rvec2(0., 0.), 90., dir_size, 0., Color::WHITE),
                    false => d.draw_text_pro( &font, ">", rvec2(58, y + 13), rvec2(0., 0.), 90., dir_size, 0., Color::DARKGRAY),
                }
                // Left
                match left {
                    true => d.draw_text_pro( &font, ">", rvec2(40, y + 14), rvec2(0., 0.), 180., dir_size, 0., Color::WHITE),
                    false => d.draw_text_pro( &font, ">", rvec2(40, y + 14), rvec2(0., 0.), 180., dir_size, 0., Color::DARKGRAY),
                }
                // Right
                match right {
                    true => d.draw_text_pro( &font, ">", rvec2(58, y - 5), rvec2(0., 0.), 0., dir_size, 0., Color::WHITE),
                    false => d.draw_text_pro( &font, ">", rvec2(58, y - 5), rvec2(0., 0.), 0., dir_size, 0., Color::DARKGRAY),
                }
                // LP
                match input.lp {
                    true => d.draw_text_pro( &font, "LP", rvec2(pos, y - 5), rvec2(0., 0.), 0., size, 0., Color::CYAN),
                    false => d.draw_text_pro( &font, "LP", rvec2(pos, y - 5), rvec2(0., 0.), 0., size, 0., Color::DARKGRAY),
                }
                // MP
                match input.mp {
                    true => d.draw_text_pro( &font, "MP", rvec2(pos + 14, y - 5), rvec2(0., 0.), 0., size, 0., Color::YELLOW),
                    false => d.draw_text_pro( &font, "MP", rvec2(pos + 14, y - 5), rvec2(0., 0.), 0., size, 0., Color::DARKGRAY),
                }
                // HP
                match input.hp {
                    true => d.draw_text_pro( &font, "HP", rvec2(pos + 30, y - 5), rvec2(0., 0.), 0., size, 0., Color::RED),
                    false => d.draw_text_pro( &font, "HP", rvec2(pos + 30, y - 5), rvec2(0., 0.), 0., size, 0., Color::DARKGRAY),
                }
                // LK
                match input.lk {
                    true => d.draw_text_pro( &font, "LK", rvec2(pos, y + 5), rvec2(0., 0.), 0., size, 0., Color::CYAN),
                    false => d.draw_text_pro( &font, "LK", rvec2(pos, y + 5), rvec2(0., 0.), 0., size, 0., Color::DARKGRAY),
                }
                // MK
                match input.mk {
                    true => d.draw_text_pro( &font, "MK", rvec2(pos + 14, y + 5), rvec2(0., 0.), 0., size, 0., Color::YELLOW),
                    false => d.draw_text_pro( &font, "MK", rvec2(pos + 14, y + 5), rvec2(0., 0.), 0., size, 0., Color::DARKGRAY),
                }
                // HK
                match input.hk {
                    true => d.draw_text_pro( &font, "HK", rvec2(pos + 30, y + 5), rvec2(0., 0.), 0., size, 0., Color::RED),
                    false => d.draw_text_pro( &font, "HK", rvec2(pos + 30, y + 5), rvec2(0., 0.), 0., size, 0., Color::DARKGRAY),
                }
            }
        });
}
