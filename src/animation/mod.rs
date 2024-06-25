use crate::prelude::*;

#[derive(Default)]
pub struct Animator {
    pub total: i32,
    pub current: i32,
    pub index: usize,
    pub frame: i32,
    pub keyframes: Vec<Keyframe>,
    finished: bool,
}

impl Animator {
    pub fn reset(&mut self) {
        self.total = 0;
        self.current = 0;
        self.index = 0;
        self.frame = 0;
        self.finished = false;
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
                if animator.finished {
                    animator.finished = false;
                    animator.index += 1;
                }

                if animator.index >= animator.keyframes.len() {
                    animator.index = 0;
                }

                let sprite = &animator.keyframes[animator.index];

                let (pos_x, pos_y) = world_to_screen_vec(physics.position);
                let source_rec = rrect(sprite.x, sprite.y, sprite.w, sprite.h);
                let dest_rec = rrect(pos_x, pos_y, sprite.w, sprite.h);
                let origin = rvec2(180, 190);
                let rotation = 0.;
                let tint = Color::WHITE;

                d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint);

                if animator.frame == 0 {
                    animator.frame = sprite.duration;
                    animator.finished = true;
                }

                animator.frame -= 1;
            }
        });
}

pub fn animation2(d: &mut RaylibTextureMode<RaylibDrawHandle>, world: &World, assets: &Assets) {
    world
        .query::<(&Physics, &Player, &Character, &mut Animator)>()
        .into_iter()
        .for_each(|(_, (physics, player, character, animator))| {
            let texture = match player {
                Player::One => Some(&assets.ken),
                Player::Two => None,
            };

            if let Some(texture) = texture {
                if animator.total == 0 {
                    animator.total = character.data.actions[0].total;
                    animator.current = character.data.actions[0].timeline[0].duration / 16;
                }

                let frame = character.data.actions[0].timeline[animator.total as usize];

                let (pos_x, pos_y) = world_to_screen_vec(physics.position);
                let source_rec = rrect(frame.x, frame.y, frame.w, frame.h);
                let dest_rec = rrect(pos_x, pos_y, frame.w, frame.h);
                let origin = rvec2(180, 190);
                let rotation = 0.;
                let tint = Color::WHITE;

                d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint);
                animator.current -= 1;
                if animator.current == 0 {
                    animator.total -= 1;

                    animator.current = character.data.actions[0].timeline[0].duration / 16;
                }
            }
        });
}
