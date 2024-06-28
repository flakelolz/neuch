use crate::prelude::*;

#[derive(Default)]
pub struct Animator {
    /// Internal timer for each keyframe
    tick: i32,
    /// Keyframe array index
    index: usize,
    /// Current keyframe duration
    duration: i32,
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

            let Some(texture) = texture else { return };

            if let Some(keyframe) = animator.keyframes.get(animator.index) {
                animator.duration = keyframe.duration;

                draw(d, physics, keyframe, texture);

                animator.tick += 1;
            }

            if animator.tick >= animator.duration {
                animator.tick = 0;
                animator.index += 1;
                if animator.index >= animator.keyframes.len() {
                    animator.index = 0;
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
    let dest_rec = rrect(pos_x, pos_y, source_rec.width, source_rec.height);
    let origin = rvec2(dest_rec.width / 2., dest_rec.height);
    let rotation = 0.;
    let tint = Color::WHITE;

    d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint);
}
