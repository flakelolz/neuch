use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Reaction {
    /// Attacker's attack has made contact with hurtbox
    pub has_hit: bool,
    /// Attacker's attack was blocked
    pub blocked: bool,
    /// Attacker's attack can be canceled out of
    pub can_cancel: bool,
    /// Everyone's hitstop
    pub hitstop: u32,
    /// Everyone's hitstun
    pub hitstun: u32,
    /// Everyone's blockstun
    pub blockstun: u32,
    /// Everyone's knockback
    pub knockback: i32,
    /// The defender has blocked and attack
    pub blocking: bool,
    /// Which way the defender is blocking
    pub block_height: Block,
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

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Block {
    #[default]
    None,
    High,
    Low,
}

#[derive(Debug, Clone, Copy)]
pub struct HitEvent {
    pub attacker: Entity,
    pub defender: Entity,
    /// Height of the hurtbox that was hit, to know which animation to transition to
    pub height: Height,
    pub properties: HitProperties,
    /// Proximity block boxes
    pub proximity: Option<(ProximityBox, u32)>,
}

pub fn reaction_system(world: &mut World, hit_events: &mut Vec<HitEvent>) {
    for (_, state) in world.query_mut::<&mut StateMachine>() {
        let reaction = &mut state.context.ctx.reaction;
        if reaction.hitstop > 0 {
            reaction.hitstop -= 1;
        }
        if reaction.hitstun > 0 && reaction.hitstop == 0 {
            reaction.hitstun -= 1;
        }
        if reaction.blockstun > 0 && reaction.hitstop == 0 {
            reaction.blockstun -= 1;
        }
    }

    for event in hit_events.iter() {
        let mut players = world
            .query_many_mut::<(&mut StateMachine, &mut InputBuffer, &mut Physics), 2>([
                event.attacker,
                event.defender,
            ]);

        let (p1, p2) = players.split_at_mut(1);
        if let Some(Ok(p1)) = p1.get_mut(0) {
            if let Some(Ok(p2)) = p2.get_mut(0) {
                let (a_state, _a_buffer, _a_physics) = p1;
                let (b_state, b_buffer, b_physics) = p2;
                if event.proximity.is_none() {
                    a_state.context.ctx.reaction.hitstop = event.properties.hitstop;
                    b_state.context.ctx.reaction.hitstop = event.properties.hitstop;
                    // KNOCKBACK
                    if b_physics.cornered {
                        a_state.context.ctx.reaction.knockback = -event.properties.knockback;
                    } else {
                        b_state.context.ctx.reaction.knockback = event.properties.knockback;
                    }

                    // HIT
                    {
                        a_state.context.ctx.reaction.blocked = false;
                        b_state.context.ctx.reaction.hitstun = event.properties.hitstun;
                        hit_reaction_transitions(&mut b_state.context, b_buffer, event)
                    }
                }
                // GUARD
                if b_state.context.ctx.reaction.block_height == Block::High
                    || b_state.context.ctx.reaction.block_height == Block::Low
                {
                    a_state.context.ctx.reaction.blocked = true;
                    guard_reaction_transitions(&mut b_state.context, b_buffer, event);
                }
            }
        }
    }

    hit_events.clear();
}

pub fn hit_animation(animator: &mut Animator, context: &mut Context, timeline: &[Keyframe]) {
    context.duration = context.ctx.reaction.hitstun;
    let length = timeline.len();
    let avg = context.ctx.reaction.hitstun / length as u32;
    let residue = context.ctx.reaction.hitstun - avg * length as u32;

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
    context.duration = context.ctx.reaction.blockstun;
    let length = timeline.len();
    let avg = context.duration / length as u32;
    let residue = context.duration - avg * length as u32;

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

fn hit_reaction_transitions(context: &mut Context, buffer: &InputBuffer, hit_event: &HitEvent) {
    let next = &mut context.ctx.next;
    match hit_event.properties.strength {
        Strength::BackSpin => {
            if buffer.current().down {
                *next = Some(Box::new(reacting::CrouchStrong));
            } else {
                *next = Some(Box::new(reacting::BackSpin));
            }
        }
        Strength::FrontSpin => {
            if buffer.current().down {
                *next = Some(Box::new(reacting::CrouchStrong));
            } else {
                *next = Some(Box::new(reacting::FrontSpin));
            }
        }
        Strength::Rising => {
            if buffer.current().down {
                *next = Some(Box::new(reacting::CrouchStrong));
            } else {
                match hit_event.height {
                    Height::Upper => *next = Some(Box::new(reacting::UpperRising)),
                    Height::Lower => *next = Some(Box::new(reacting::LowerRising)),
                }
            }
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
}

fn guard_reaction_transitions(context: &mut Context, buffer: &InputBuffer, hit_event: &HitEvent) {
    let reaction = &mut context.ctx.reaction;
    let next = &mut context.ctx.next;
    if hit_event.proximity.is_some() {
        if !reaction.blocking {
            reaction.blockstun = hit_event.proximity.unwrap().1;
            if buffer.current().down {
                *next = Some(Box::new(reacting::GrdCrouchPre))
            } else {
                *next = Some(Box::new(reacting::GrdStandPre))
            }
        }
    } else {
        reaction.hitstop = hit_event.properties.hitstop;
        reaction.blockstun = hit_event.properties.blockstun;
        if buffer.current().down {
            *next = Some(Box::new(reacting::GrdCrouchEnd))
        } else {
            *next = Some(Box::new(reacting::GrdStandEnd))
        }
    }
}
