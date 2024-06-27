use crate::prelude::*;

#[derive(Default)]
pub struct Animator {
    /// Current keyframe duration
    pub duration: i32,
    /// Internal timer for each keyframe
    pub tick: i32,
    /// Keyframe array index
    pub index: usize,
    /// Collection of all the keyframes on an action
    pub keyframes: Vec<Keyframe>,
}

impl Animator {
    pub fn reset(&mut self) {
        self.duration = 0;
        self.index = 0;
        self.tick = 0;
    }
}

// FIX: Animations are one frame behind
pub fn animation(d: &mut RaylibTextureMode<RaylibDrawHandle>, world: &World, assets: &Assets) {
    world
        .query::<(&Physics, &Player, &mut Animator)>()
        .into_iter()
        .for_each(|(_, (physics, player, animator))| {
            let texture = match player {
                Player::One => Some(&assets.ken),
                Player::Two => None,
            };

            if let Some(texture) = texture {
                if animator.tick > animator.duration {
                    animator.tick = 0;
                    animator.index += 1;
                    if animator.index >= animator.keyframes.len() {
                        animator.index = 0;
                    }
                }

                if let Some(keyframe) = animator.keyframes.get(animator.index) {
                    animator.duration = keyframe.duration;
                    draw(d, physics, keyframe, texture);
                    animator.tick += 1;
                }
            }
        });
}

fn draw(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    physics: &Physics,
    keyframe: &Keyframe,
    texture: &Texture2D,
) {
    let (pos_x, pos_y) = world_to_screen_vec(physics.position);
    let source_rec = rrect(keyframe.x, keyframe.y, keyframe.w, keyframe.h);
    let dest_rec = rrect(pos_x, pos_y, keyframe.w, keyframe.h);
    let origin = rvec2(180, 190);
    let rotation = 0.;
    let tint = Color::WHITE;

    d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint);
}

// pub fn animation2(d: &mut RaylibTextureMode<RaylibDrawHandle>, world: &World, assets: &Assets) {
//     world
//         .query::<(&Physics, &Player, &Character, &mut Animator)>()
//         .into_iter()
//         .for_each(|(_, (physics, player, character, animator))| {
//             let texture = match player {
//                 Player::One => Some(&assets.ken),
//                 Player::Two => None,
//             };
//
//             if let Some(texture) = texture {
//                 if animator.total == 0 {
//                     animator.total = character.data.actions[0].total;
//                     animator.current = character.data.actions[0].timeline[0].duration / 16;
//                 }
//
//                 let frame = character.data.actions[0].timeline[animator.total as usize];
//
//                 let (pos_x, pos_y) = world_to_screen_vec(physics.position);
//                 let source_rec = rrect(frame.x, frame.y, frame.w, frame.h);
//                 let dest_rec = rrect(pos_x, pos_y, frame.w, frame.h);
//                 let origin = rvec2(180, 190);
//                 let rotation = 0.;
//                 let tint = Color::WHITE;
//
//                 d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint);
//                 animator.current -= 1;
//                 if animator.current == 0 {
//                     animator.total -= 1;
//
//                     animator.current = character.data.actions[0].timeline[0].duration / 16;
//                 }
//             }
//         });
// }
