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
    pub flipped: bool,
    /// Z index
    pub z_index: i32,
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
    pub fn new(origin: Vec2, z_index: i32, flipped: bool) -> Self {
        Self {
            origin,
            z_index,
            flipped,
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
            flipped: false,
            z_index: 0,
            keyframes: vec![],
        }
    }
}

pub fn animation(d: &mut impl RaylibDraw, world: &World, assets: &Assets) {
    let mut buffer: Vec<Draw> = Vec::new();
    world
        .query::<(&Physics, &mut Animator, &StateMachine)>()
        .into_iter()
        .for_each(|(_, (physics, animator, state))| {
            let keyframe = animator.keyframes[animator.index];
            animator.flipped = physics.facing_left;
            animator.duration = keyframe.duration;
            let reaction = &state.context.ctx.reaction;

            let pos_x = physics.position.x;
            let pos_y = -physics.position.y;
            let mut draw = Draw {
                x: keyframe.x,
                y: keyframe.y,
                w: keyframe.w,
                h: keyframe.h,
                flip: animator.flipped,
                w_scale: animator.w_scale,
                h_scale: animator.h_scale,
                origin: animator.origin,
                z_index: animator.z_index,
                pos: IVec2::new(pos_x, pos_y),
            };

            if reaction.hitstop == 0 {
                animator.tick += 1;
            }
            if animator.tick >= animator.duration {
                animator.tick = 0;
                animator.index += 1;

                if animator.index >= animator.keyframes.len() {
                    animator.index = 0;
                }
            }

            if reaction.hitstop > 0 && (reaction.hitstun > 0 || reaction.blockstun > 0) {
                let hitshake_dist: i32 = 2;
                let hitshake = -(hitshake_dist / 2) + hitshake_dist * (reaction.hitstop as i32 % 2);
                draw.x += hitshake as f32;
            }

            buffer.push(draw);
        });

    draw(d, buffer, &assets.ken);
}

pub fn animation2(d: &mut impl RaylibDraw, world: &World, assets: &Assets) {
    let mut buffer: Vec<Draw> = Vec::new();
    world
        .query::<(&Physics, &mut Animator)>()
        .into_iter()
        .for_each(|(_, (physics, animator))| {
            let keyframe = animator.keyframes[animator.index];
            animator.flipped = physics.facing_left;
            animator.duration = keyframe.duration;

            let pos_x = physics.position.x;
            let pos_y = -physics.position.y;
            let draw = Draw {
                x: keyframe.x,
                y: keyframe.y,
                w: keyframe.w,
                h: keyframe.h,
                flip: animator.flipped,
                w_scale: animator.w_scale,
                h_scale: animator.h_scale,
                origin: animator.origin,
                z_index: animator.z_index,
                pos: IVec2::new(pos_x, pos_y),
            };

            if animator.tick >= animator.duration {
                animator.tick = 0;
                animator.index += 1;

                if animator.index >= animator.keyframes.len() {
                    animator.index = 0;
                }
            }

            buffer.push(draw);
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
