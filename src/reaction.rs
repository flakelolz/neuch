use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Reaction {
    pub has_hit: bool,
    pub hitstop: u32,
    pub hitstun: u32,
    pub blockstun: u32,
    pub knockback: i32,
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
    pub height: Height,
    pub properties: HitProperties,
    pub proximity: Option<ProximityBox>,
}

pub fn reaction_system(world: &mut World, hit_events: &mut Vec<HitEvent>) {
    for (id, (state, buffer)) in world.query_mut::<(&mut StateMachine, &mut InputBuffer)>() {
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
            let next = &mut state.context.ctx.next;
            if id == hit_event.attacker {
                if hit_event.proximity.is_some() {
                    continue;
                }

                reaction.hitstop = hit_event.properties.hitstop;
            }

            if id == hit_event.defender {
                if !buffer.current().backward {
                    if hit_event.proximity.is_some() {
                        continue;
                    }
                    // If hit
                    reaction.hitstop = hit_event.properties.hitstop;
                    reaction.hitstun = hit_event.properties.hitstun;
                    reaction.knockback = hit_event.properties.knockback;

                    match hit_event.properties.strength {
                        Strength::Spin => {
                            todo!();
                        }
                        Strength::Rising => {
                            todo!();
                        }
                        Strength::Strong => {
                            if buffer.current().down {
                                *next = Some(Box::new(reacting::CrouchStrong));
                            } else {
                                match hit_event.height {
                                    Height::Upper => *next = Some(Box::new(reacting::UpperStrong)),
                                    Height::Lower => *next = Some(Box::new(reacting::LowerStrong)),
                                }
                            }
                        }
                        Strength::Mid => {
                            if buffer.current().down {
                                *next = Some(Box::new(reacting::CrouchMid));
                            } else {
                                match hit_event.height {
                                    Height::Upper => *next = Some(Box::new(reacting::UpperMid)),
                                    Height::Lower => *next = Some(Box::new(reacting::LowerMid)),
                                }
                            }
                        }
                        Strength::Weak => {
                            if buffer.current().down {
                                *next = Some(Box::new(reacting::CrouchWeak));
                            } else {
                                match hit_event.height {
                                    Height::Upper => *next = Some(Box::new(reacting::UpperWeak)),
                                    Height::Lower => *next = Some(Box::new(reacting::LowerWeak)),
                                }
                            }
                        }
                    }

                // Proximity block
                } else if hit_event.proximity.is_some() {
                    if !reaction.block() {
                        reaction.blockstun = hit_event.properties.blockstun;
                        if buffer.current().down {
                            state.context.ctx.next = Some(Box::new(reacting::GrdCrouchPre));
                        } else {
                            state.context.ctx.next = Some(Box::new(reacting::GrdStandPre));
                        }
                    }
                // Guard
                } else {
                    reaction.hitstop = hit_event.properties.hitstop;
                    reaction.blockstun = hit_event.properties.blockstun;
                    reaction.knockback = hit_event.properties.knockback;
                    if buffer.current().down {
                        *next = Some(Box::new(reacting::GrdCrouchEnd))
                    } else {
                        *next = Some(Box::new(reacting::GrdStandEnd))
                    }
                }
            }
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

pub fn guard_animation(animator: &mut Animator, context: &mut Context, timeline: &[Keyframe]) {
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
