use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub character: Option<CharacterInfo>,
    pub elapsed: u32,
    pub duration: u32,
    pub modifiers: Instructions,
    pub spawn: Option<Projectile>,
    pub ctx: SubContext,
}

pub struct SubContext {
    pub name: String,
    pub next: Option<Box<dyn State>>,
    pub can_dash_f: bool,
    pub can_dash_b: bool,
    pub reaction: Reaction,
    pub flags: Flags,
}

impl Default for SubContext {
    fn default() -> Self {
        Self {
            name: String::new(),
            next: None,
            can_dash_f: true,
            can_dash_b: true,
            reaction: Reaction::default(),
            flags: Flags {
                jump: JumpFlags::None,
            },
        }
    }
}

#[derive(Default, Clone)]
pub struct Projectile {
    pub owner: Option<Entity>,
    pub name: String,
    pub physics: Physics,
    pub timing: u32,
    pub duration: u32,
}

#[derive(Debug, Default)]
pub struct Flags {
    pub jump: JumpFlags,
}

#[derive(Debug, Default, PartialEq)]
pub enum JumpFlags {
    #[default]
    None,
    Forward,
    Backward,
}

// Naming is hard
#[derive(Default)]
pub struct Instructions {
    pub index: usize,
    pub instructions: Option<Modifiers>,
}

pub fn handle_modifiers(context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
    if let Some(instructions) = &context.modifiers.instructions {
        if let Some(position) = &instructions.potisions {
            if let Some(position) = position.get(context.modifiers.index) {
                if position.on_frame == context.elapsed {
                    physics.set_forward_position(position.value.x);
                    physics.position.y = position.value.y;
                    context.modifiers.index += 1;
                }
            }
        }

        if let Some(cancels) = &instructions.cancels {
            for action in cancels {
                if context.elapsed >= action.after_frame
                    && context.elapsed <= action.until_frame.unwrap_or(u32::MAX)
                {
                    for state in &action.states {
                        if action.on.is_empty() {
                            if state.set(buffer, &mut context.ctx, physics) {
                                turn_transition(&mut context.ctx, buffer, physics);
                                return;
                            }
                        } else {
                            for collision in &action.on {
                                match collision {
                                    CollisionType::Hit => {
                                        if context.ctx.reaction.can_cancel
                                            && !context.ctx.reaction.blocked
                                            && context.ctx.reaction.hitstop == 0
                                            && state.set(buffer, &mut context.ctx, physics)
                                        {
                                            turn_transition(&mut context.ctx, buffer, physics);
                                            return;
                                        }
                                    }
                                    CollisionType::Block => {
                                        if context.ctx.reaction.can_cancel
                                            && context.ctx.reaction.blocked
                                            && context.ctx.reaction.hitstop == 0
                                            && state.set(buffer, &mut context.ctx, physics)
                                        {
                                            turn_transition(&mut context.ctx, buffer, physics);
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
