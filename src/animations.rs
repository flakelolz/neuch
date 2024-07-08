use crate::prelude::*;

pub struct Animator {
    /// Internal timer for each keyframe
    tick: u32,
    /// Keyframe array index
    index: usize,
    /// Current keyframe duration
    duration: u32,
    /// Width scale of entity being drawn
    w_scale: f32,
    /// Height scale of entity being drawn
    h_scale: f32,
    /// Whether the animation should be flipped
    flip: bool,
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

impl Default for Animator {
    fn default() -> Self {
        Self {
            tick: 0,
            index: 0,
            duration: 0,
            w_scale: 1.,
            h_scale: 1.,
            flip: false,
            keyframes: vec![],
        }
    }
}

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

                draw(d, animator, physics, keyframe, texture);

                animator.tick += 1;
            }

            if animator.tick >= animator.duration {
                animator.tick = 0;
                animator.index += 1;

                // Wrap around for looping actions
                if animator.index >= animator.keyframes.len() {
                    animator.index = 0;
                }
            }
        });
}

fn draw(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    animator: &Animator,
    physics: &Physics,
    keyframe: &Keyframe,
    texture: &Texture2D,
) {
    let (screen_x, screen_y) = pos_to_screen(physics.position);
    let (width, height) = (keyframe.w, keyframe.h);

    let source_rec = rrect(
        keyframe.x,
        keyframe.y,
        {
            if animator.flip {
                -width * animator.w_scale
            } else {
                width * animator.w_scale
            }
        },
        height * animator.h_scale,
    );
    let dest_rec = rrect(screen_x, screen_y, width, height);
    let origin = rvec2(width / 2., height);
    let rotation = 0.;
    let tint = Color::WHITE;

    d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint);
}
