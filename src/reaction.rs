use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Reaction {
    pub has_hit: bool,
    pub hitstop: u32,
    pub hitstun: u32,
    pub blockstun: u32,
    pub knockback: IVec2,
    pub air_knockback: IVec2,
}

impl Reaction {
    pub fn reset_all(&mut self) {
        self.has_hit = false;
        self.hitstop = 0;
        self.hitstun = 0;
        self.blockstun = 0;
        self.knockback = IVec2::zero();
        self.air_knockback = IVec2::zero();
    }
    pub fn reset_atk(&mut self) {
        self.has_hit = false;
        self.hitstop = 0;
    }
    pub fn reset_def(&mut self) {
        self.hitstop = 0;
        self.hitstun = 0;
        self.blockstun = 0;
        self.knockback = IVec2::zero();
        self.air_knockback = IVec2::zero();
    }

    pub fn reacting(&self) -> bool {
        if self.hitstun > 0 || self.blockstun > 0 {
            return true;
        }
        false
    }

    pub fn set_all(&mut self, event: &HitEvent) {
        self.hitstop = event.properties.hitstop;
        self.hitstun = event.properties.hitstun;
        self.blockstun = event.properties.blockstun;
        self.knockback = event.properties.knockback;
        self.air_knockback = event.properties.air_knockback;
    }
    pub fn set_atk(&mut self, event: &HitEvent) {
        self.hitstop = event.properties.hitstop;
    }
    pub fn set_def(&mut self, event: &HitEvent) {
        self.hitstop = event.properties.hitstop;
        self.hitstun = event.properties.hitstun;
        self.blockstun = event.properties.blockstun;
        self.knockback = event.properties.knockback;
        self.air_knockback = event.properties.air_knockback;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub properties: HitProperties,
}

pub fn reaction_system(world: &mut World, hit_events: &mut Vec<HitEvent>) {
    for (id, (state, physics, buffer)) in
        world.query_mut::<(&mut StateMachine, &mut Physics, &InputBuffer)>()
    {
        if state.context.reaction.hitstop > 0 {
            state.context.reaction.hitstop -= 1;
        } else if state.context.reaction.hitstun > 0 {
            state.context.reaction.hitstun -= 1;
        }

        for hit_event in hit_events.iter() {
            if id == hit_event.attacker {
                println!("attacker: {:?}", id);
                state.context.reaction.set_atk(hit_event);
            }

            if id == hit_event.defender {
                println!("defender: {:?}", id);
                state.context.reaction.set_def(hit_event);

                match hit_event.properties.reaction_type {
                    ReactionType::StandMid => {
                        state.context.ctx.next = Some(Box::new(reacting::HitStandMid));
                    }
                    _ => (),
                }
            }
        }
    }

    hit_events.clear();
}

pub fn set_hit_state(animator: &mut Animator, context: &mut Context, timeline: &[Keyframe]) {
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
