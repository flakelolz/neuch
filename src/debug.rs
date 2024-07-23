#![allow(dead_code)]
use crate::prelude::*;

const TEXT_SIZE: f32 = 10.0;
const SCREEN_CENTER: i32 = WIDTH / 2;

pub struct Debug {
    pub information: bool,
    pub position: bool,
    pub state: bool,
    pub inputs: bool,
    pub hitboxes: bool,
    pub hurtboxes: bool,
    pub pushboxes: bool,
    pub proximity: bool,
    pub editor: bool,
    pub all: bool,
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            information: false,
            position: true,
            state: true,
            inputs: true,
            hitboxes: true,
            hurtboxes: true,
            pushboxes: true,
            proximity: false,
            editor: false,
            all: true,
        }
    }
}

impl Debug {
    pub fn toggle(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            self.editor = !self.editor;
            println!("Editor: {}", self.editor);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F2) {
            self.hitboxes = !self.hitboxes;
            println!("Hitboxes: {}", self.hitboxes);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F3) {
            self.hurtboxes = !self.hurtboxes;
            println!("Hurtboxes: {}", self.hurtboxes);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F4) {
            self.pushboxes = !self.pushboxes;
            println!("Pushboxes: {}", self.pushboxes);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F5) {
            self.proximity = !self.proximity;
            println!("Proximity: {}", self.proximity);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F6) {
            self.state = !self.state;
            println!("State: {}", self.state);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F7) {
            self.position = !self.position;
            println!("Position: {}", self.position);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F8) {
            self.information = !self.information;
            println!("Information: {}", self.information);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F9) {
            self.inputs = !self.inputs;
            println!("Inputs: {}", self.inputs);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F11) {
            if self.all {
                self.all_false();
                println!("All debug options off");
            } else {
                self.all_true();
                println!("All debug options on");
            }
        }
    }

    fn all_false(&mut self) {
        *self = Self {
            information: false,
            position: false,
            state: false,
            inputs: false,
            hitboxes: false,
            hurtboxes: false,
            pushboxes: false,
            proximity: false,
            editor: false,
            all: false,
        };
    }

    fn all_true(&mut self) {
        *self = Self {
            information: true,
            position: true,
            state: true,
            inputs: true,
            hitboxes: true,
            hurtboxes: true,
            pushboxes: true,
            proximity: true,
            editor: true,
            all: true,
        };
    }
}

pub fn move_player(world: &mut World, rl: &mut RaylibHandle) {
    for (_, (physics, player)) in world.query_mut::<(&mut Physics, &Player)>() {
        if player == &Player::One {
            if rl.is_key_down(KeyboardKey::KEY_EIGHT) {
                physics.position.y -= 1000;
            }
            if rl.is_key_down(KeyboardKey::KEY_NINE) {
                physics.position.y += 1000;
            }

            if rl.is_key_down(KeyboardKey::KEY_SEVEN) {
                physics.position.x -= 1000;
            }
            if rl.is_key_down(KeyboardKey::KEY_ZERO) {
                physics.position.x += 1000;
            }
        }
    }
}

pub fn show_position(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    let font = d.gui_get_font();
    if !debug.position {
        return;
    }
    world
        .query::<&Physics>()
        .into_iter()
        .for_each(|(_, physics)| {
            let (screen_x, screen_y) = pos_to_screen(physics.position.rev_y());
            let (screen_x, screen_y) = sprite_to_ui(screen_x, screen_y);
            let (pos_x, pos_y) = world_to_screen(physics.position);
            d.draw_circle(screen_x, screen_y, 1., Color::WHITE);
            d.draw_text_ex(
                &font,
                &format!("{}:{}", pos_x, pos_y),
                rvec2(screen_x, screen_y + 2),
                TEXT_SIZE,
                0.,
                Color::WHITE,
            );
        });
}

pub fn show_state(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    if !debug.state {
        return;
    }
    let font = d.gui_get_font();
    world
        .query::<(&StateMachine, &Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (state, physics, player))| {
            if player == &Player::One {
                let (screen_x, screen_y) = pos_to_screen(physics.position.rev_y());
                let (screen_x, screen_y) = sprite_to_ui(screen_x, screen_y);
                let current = state.processor.current.as_ref();
                let elapsed = state.context.elapsed;
                let duration = state.context.duration;
                let top = 200;
                let offset = 10;

                // State name
                d.draw_text_ex(
                    &font,
                    &current.name(),
                    rvec2(screen_x - 30, screen_y - top),
                    TEXT_SIZE,
                    0.,
                    Color::WHITE,
                );
                // Total state duration
                d.draw_text_ex(
                    &font,
                    &format!("{}", duration),
                    rvec2(screen_x - 30, screen_y - top + offset),
                    TEXT_SIZE,
                    0.,
                    Color::WHITE,
                );
                // Frames elapsed
                d.draw_text_ex(
                    &font,
                    &format!("{}", elapsed),
                    rvec2(screen_x - 10, screen_y - top + offset),
                    TEXT_SIZE,
                    0.,
                    Color::WHITE,
                );
            }
        });
}

pub fn show_frame_count(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    if !debug.information {
        return;
    }
    let font = d.gui_get_font();
    world.query::<&u32>().into_iter().for_each(|(_, frame)| {
        d.draw_text_ex(
            &font,
            &format!("{}", frame),
            rvec2(5, 2),
            TEXT_SIZE,
            0.,
            Color::WHITE,
        );
    });
}

pub fn show_context(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    if !debug.information {
        return;
    }
    let font = d.gui_get_font();
    let y = 12.;
    world
        .query::<(&StateMachine, &Player)>()
        .into_iter()
        .for_each(|(_, (machine, player))| {
            if player == &Player::One {
                d.draw_text_ex(
                    &font,
                    &format!("F: {}", machine.context.ctx.can_dash_f),
                    rvec2(5., y),
                    TEXT_SIZE,
                    0.,
                    Color::WHITE,
                );
                d.draw_text_ex(
                    &font,
                    &format!("B: {}", machine.context.ctx.can_dash_b),
                    rvec2(5., y + TEXT_SIZE),
                    TEXT_SIZE,
                    0.,
                    Color::WHITE,
                );
                d.draw_text_ex(
                    &font,
                    &format!("Jump: {:#?}", machine.context.ctx.flags.jump),
                    rvec2(5., y + TEXT_SIZE * 2.),
                    TEXT_SIZE,
                    0.,
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

pub fn show_hitboxes(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    if !debug.hitboxes {
        return;
    }
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
                    let top = -world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
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

pub fn show_proximity_boxes(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    if !debug.proximity {
        return;
    }
    for (_, (character, physics, state)) in world
        .query::<(&Character, &Physics, &StateMachine)>()
        .iter()
    {
        if let Some(action) = find_action(character, &state.processor.current.name()) {
            let offset = physics.position;
            if let Some(modifiers) = &action.modifiers {
                if let Some(proximity) = modifiers.proximity {
                    let translated = if physics.facing_left {
                        proximity.value.translate_flipped(offset)
                    } else {
                        proximity.value.translate(offset)
                    };

                    let left = world_to_sprite_to_ui_num(translated.left);
                    let top = -world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
                    let width = world_to_sprite_to_ui_num(translated.right - translated.left);
                    let height = world_to_sprite_to_ui_num(translated.top - translated.bottom);
                    if proximity.is_active(state.context.elapsed) {
                        d.draw_rectangle_lines(left, top, width, height, Color::YELLOW);
                    }
                }
            }
        }
    }
}

pub fn show_hurtboxes(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    if !debug.hurtboxes {
        return;
    }
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
                    let top = -world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
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

pub fn show_pushboxes(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    if !debug.pushboxes {
        return;
    }
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
                    let top = -world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
                    let width = world_to_sprite_to_ui_num(translated.right - translated.left);
                    let height = world_to_sprite_to_ui_num(translated.top - translated.bottom);
                    if pushbox.is_active(state.context.elapsed) {
                        d.draw_rectangle_lines(left, top, width, height, Color::MAGENTA);
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
                let top = -world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
                let width = world_to_sprite_to_ui_num(translated.right - translated.left);
                let height = world_to_sprite_to_ui_num(translated.top - translated.bottom);
                d.draw_rectangle_lines(left, top, width, height, Color::MAGENTA);
            }
        }
    }
}

#[rustfmt::skip]
pub fn show_inputs(world: &World, d: &mut impl RaylibDraw, debug: &Debug) {
    if !debug.inputs {
        return;
    }
    world
        .query::<(&Input, &Physics, &Player)>()
        .iter()
        .for_each(|(_, (input, physics, player))| {
            let dir_size = 20.;
            let size = 10.;
            let pos = 70;
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
                let gray = Color::new(169, 169, 169, 150);
                // Up
                match input.up {
                    true => d.draw_text_pro( &font, ">", rvec2(40, y - 1), rvec2(0., 0.), 270., dir_size, 0., Color::WHITE),
                    false => d.draw_text_pro( &font, ">", rvec2(40, y - 1), rvec2(0., 0.), 270., dir_size, 0., gray),
                }
                // Down
                match input.down {
                    true => d.draw_text_pro( &font, ">", rvec2(62, y + 13), rvec2(0., 0.), 90., dir_size, 0., Color::WHITE),
                    false => d.draw_text_pro( &font, ">", rvec2(62, y + 13), rvec2(0., 0.), 90., dir_size, 0., gray),
                }
                // Left
                match left {
                    true => d.draw_text_pro( &font, ">", rvec2(44, y + 17), rvec2(0., 0.), 180., dir_size, 0., Color::WHITE),
                    false => d.draw_text_pro( &font, ">", rvec2(44, y + 17), rvec2(0., 0.), 180., dir_size, 0., gray),
                }
                // Right
                match right {
                    true => d.draw_text_pro( &font, ">", rvec2(58, y - 5), rvec2(0., 0.), 0., dir_size, 0., Color::WHITE),
                    false => d.draw_text_pro( &font, ">", rvec2(58, y - 5), rvec2(0., 0.), 0., dir_size, 0., gray),
                }
                // LP
                match input.lp {
                    true => d.draw_text_pro( &font, "LP", rvec2(pos, y - 5), rvec2(0., 0.), 0., size, 0., Color::CYAN),
                    false => d.draw_text_pro( &font, "LP", rvec2(pos, y - 5), rvec2(0., 0.), 0., size, 0., gray),
                }
                // MP
                match input.mp {
                    true => d.draw_text_pro( &font, "MP", rvec2(pos + 10, y - 5), rvec2(0., 0.), 0., size, 0., Color::YELLOW),
                    false => d.draw_text_pro( &font, "MP", rvec2(pos + 10, y - 5), rvec2(0., 0.), 0., size, 0., gray),
                }
                // HP
                match input.hp {
                    true => d.draw_text_pro( &font, "HP", rvec2(pos + 22, y - 5), rvec2(0., 0.), 0., size, 0., Color::RED),
                    false => d.draw_text_pro( &font, "HP", rvec2(pos + 22, y - 5), rvec2(0., 0.), 0., size, 0., gray),
                }
                // LK
                match input.lk {
                    true => d.draw_text_pro( &font, "LK", rvec2(pos, y + 5), rvec2(0., 0.), 0., size, 0., Color::CYAN),
                    false => d.draw_text_pro( &font, "LK", rvec2(pos, y + 5), rvec2(0., 0.), 0., size, 0., gray),
                }
                // MK
                match input.mk {
                    true => d.draw_text_pro( &font, "MK", rvec2(pos + 10, y + 5), rvec2(0., 0.), 0., size, 0., Color::YELLOW),
                    false => d.draw_text_pro( &font, "MK", rvec2(pos + 10, y + 5), rvec2(0., 0.), 0., size, 0., gray),
                }
                // HK
                match input.hk {
                    true => d.draw_text_pro( &font, "HK", rvec2(pos + 22, y + 5), rvec2(0., 0.), 0., size, 0., Color::RED),
                    false => d.draw_text_pro( &font, "HK", rvec2(pos + 22, y + 5), rvec2(0., 0.), 0., size, 0., gray),
                }
            }
        });
}
