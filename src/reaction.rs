use crate::{physics, prelude::*};

#[derive(Default, Debug, Clone, Copy)]
pub struct Reaction {
    pub has_hit: bool,
    pub hitstop: u32,
    pub hitstun: u32,
    pub blockstun: u32,
    pub knockback: IVec2,
}

impl Reaction {
    pub fn hit(&self) -> bool {
        if self.hitstun > 0 {
            return true;
        }
        false
    }

    pub fn block(&self) -> bool {
        if self.blockstun > 0 {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub properties: HitProperties,
    pub distance: Option<i32>,
}

pub fn reaction_system(world: &mut World, hit_events: &mut Vec<HitEvent>) {
    for (id, (state, physics)) in world.query_mut::<(&mut StateMachine, &mut Physics)>() {
        let reaction = &mut state.context.reaction;
        if reaction.hitstop > 0 {
            reaction.hitstop -= 1;
        }
        if reaction.hitstun > 0 && reaction.hitstop == 0 {
            reaction.hitstun -= 1;
        }
        if reaction.blockstun > 0 && reaction.hitstop == 0 {
            reaction.blockstun -= 1;
        }

        for hit_event in hit_events.iter() {
            if id == hit_event.attacker {
                reaction.hitstop = hit_event.properties.hitstop;
            }

            if id == hit_event.defender {
                // If hit
                reaction.hitstop = hit_event.properties.hitstop;
                reaction.hitstun = hit_event.properties.hitstun;
                reaction.knockback = hit_event.properties.knockback;

                match hit_event.properties.reaction_type {
                    ReactionType::StandMid => {
                        state.context.ctx.next = Some(Box::new(reacting::HitStandMid))
                    }
                    _ => (),
                }
            }

            // Handle push events
            // if let Some(push_distance) = hit_event.distance {
            //     physics.position.x = if physics.facing_left {
            //         physics.position.x + push_distance
            //     } else {
            //         physics.position.x - push_distance
            //     };
            // }
        }
    }

    hit_events.clear();
}

pub fn hit_animation(animator: &mut Animator, context: &mut Context, timeline: &[Keyframe]) {
    context.duration = context.reaction.hitstun;
    let length = timeline.len();
    let avg = context.reaction.hitstun / length as u32;
    let residue = context.reaction.hitstun - avg * length as u32;

    animator.reset();
    animator.keyframes.clear();
    for (i, frame) in timeline.iter().enumerate() {
        let keyframe = Keyframe {
            x: frame.x,
            y: frame.y,
            w: frame.w,
            h: frame.h,
            duration: {
                if i == 1 {
                    avg + residue
                } else {
                    avg
                }
            },
        };
        animator.keyframes.push(keyframe);
    }
}

pub fn block_animation(animator: &mut Animator, context: &mut Context, timeline: &[Keyframe]) {
    context.duration = context.reaction.blockstun;
    let length = timeline.len();
    let avg = context.reaction.blockstun / length as u32;
    let residue = context.reaction.blockstun - avg * length as u32;

    animator.reset();
    animator.keyframes.clear();
    for (i, frame) in timeline.iter().enumerate() {
        let keyframe = Keyframe {
            x: frame.x,
            y: frame.y,
            w: frame.w,
            h: frame.h,
            duration: {
                if i == 1 {
                    avg + residue
                } else {
                    avg
                }
            },
        };
        animator.keyframes.push(keyframe);
    }
}
