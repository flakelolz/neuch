use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Animator {
    /// Origin
    origin: Vec2,
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
    /// Z index
    z_index: i32,
    /// Collection of all the keyframes on an action
    pub keyframes: Vec<Keyframe>,
}

struct Draw {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    flip: bool,
    w_scale: f32,
    h_scale: f32,
    origin: Vec2,
    z_index: i32,
    pos: IVec2,
}

impl Animator {
    pub fn new(origin: Vec2, z_index: i32) -> Self {
        Self {
            origin,
            z_index,
            ..Default::default()
        }
    }
    pub fn reset(&mut self) {
        self.duration = 0;
        self.index = 0;
        self.tick = 0;
    }
}

impl Default for Animator {
    fn default() -> Self {
        Self {
            origin: Vec2 { x: 1., y: 1. },
            tick: 0,
            index: 0,
            duration: 0,
            w_scale: 1.,
            h_scale: 1.,
            flip: false,
            z_index: 0,
            keyframes: vec![],
        }
    }
}

pub fn animation(d: &mut impl RaylibDraw, world: &World, assets: &Assets) {
    let mut buffer: Vec<Draw> = Vec::new();
    world
        .query::<(&Physics, &Player, &mut Animator)>()
        .into_iter()
        .for_each(|(_, (physics, player, animator))| {
            match player {
                Player::One => {
                    animator.flip = false;
                }
                Player::Two => {
                    animator.flip = true;
                }
            }

            let keyframe = animator.keyframes[animator.index];
            animator.duration = keyframe.duration;

            let draw = Draw {
                x: keyframe.x,
                y: keyframe.y,
                w: keyframe.w,
                h: keyframe.h,
                flip: animator.flip,
                w_scale: animator.w_scale,
                h_scale: animator.h_scale,
                origin: animator.origin,
                z_index: animator.z_index,
                pos: physics.position,
            };

            buffer.push(draw);
            animator.tick += 1;

            if animator.tick >= animator.duration {
                animator.tick = 0;
                animator.index += 1;

                // Wrap around for looping actions
                if animator.index >= animator.keyframes.len() {
                    animator.index = 0;
                }
            }
        });

    draw(d, buffer, &assets.ken);
}

fn draw(d: &mut impl RaylibDraw, mut commands: Vec<Draw>, texture: &Texture2D) {
    commands.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    for command in commands {
        let (screen_x, screen_y) = pos_to_screen(command.pos);
        let (width, height) = (command.w, command.h);

        let source_rec = rrect(
            command.x,
            command.y,
            {
                if command.flip {
                    -width * command.w_scale
                } else {
                    width * command.w_scale
                }
            },
            height * command.h_scale,
        );
        let dest_rec = rrect(screen_x, screen_y, width, height);
        let origin = rvec2(width * command.origin.x, height * command.origin.y);
        let rotation = 0.;
        let tint = Color::WHITE;

        d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint);
    }
}
