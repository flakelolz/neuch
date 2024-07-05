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
                let (x, y) = world_to_screen_vec(physics.position);
                d.draw_circle(x, y, 1., Color::WHITE);
                d.draw_text(format!("{}", x).as_str(), x, y + 2, 10, Color::WHITE);
            }
        });
}

pub fn show_state(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world
        .query::<(&StateMachine, &Physics)>()
        .into_iter()
        .for_each(|(_, (state, physics))| {
            let (x, y) = world_to_screen_vec(physics.position);
            let current = state.processor.current.as_ref();
            let timeline = state.context.elapsed;
            let duration = state.context.duration;

            d.draw_text(&current.name(), x - 30, y - 130, 10, Color::WHITE);
            d.draw_text(
                format!("{}", timeline + 1).as_str(),
                x - 10,
                y - 120,
                10,
                Color::WHITE,
            );
            d.draw_text(
                format!("{}", duration).as_str(),
                x - 30,
                y - 120,
                10,
                Color::WHITE,
            );
        });
}

pub fn show_frame_count(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    world.query::<&i32>().into_iter().for_each(|(_, frame)| {
        d.draw_text(&format!("{}", frame), 10, 10, 10, Color::WHITE);
    });
}

// pub fn show_context(world: &World, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
//     world
//         .query::<(&StateMachine, &Player)>()
//         .into_iter()
//         .for_each(|(_, (machine, player))| {
//             if player == &Player::One {
//                 d.draw_text(
//                     &format!("F: {}", machine.context.locked.dash_forward),
//                     10,
//                     20,
//                     10,
//                     Color::WHITE,
//                 );
//                 d.draw_text(
//                     &format!("B: {}", machine.context.locked.dash_backward),
//                     10,
//                     30,
//                     10,
//                     Color::WHITE,
//                 );
//             }
//         });
// }

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
