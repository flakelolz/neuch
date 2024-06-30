use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub next: Option<Box<dyn State>>,
    pub elapsed: i32,
    pub duration: i32,
    pub modifier: Instructions,
}

// Naming is hard
#[derive(Default)]
pub struct Instructions {
    pub index: usize,
    pub instructions: Option<Modifiers>,
}

pub fn handle_modifiers(context: &mut Context, physics: &mut Physics) {
    if let Some(instructions) = &context.modifier.instructions {
        if let Some(position) = &instructions.potisions {
            if let Some(position) = position.get(context.modifier.index) {
                if position.on_frame == context.elapsed {
                    physics.position += position.value;
                    context.modifier.index += 1;
                }
            }
        }
    }
}

pub fn handle_chainable(context: &mut Context, input: &Input) {
    if let Some(instructions) = &context.modifier.instructions {
        if let Some(chainable) = &instructions.chainable {
            if chainable.on_frame - 1 == context.elapsed {
                if chainable.st_lk && !input.down && input.lk {
                    context.next = Some(Box::new(standing::LightKick));
                    return;
                }
                if chainable.cr_lk && input.down && input.lk {
                    context.next = Some(Box::new(crouching::LightKick));
                    return;
                }
                if chainable.st_lp && !input.down && input.lp {
                    context.next = Some(Box::new(standing::LightPunch));
                    return;
                }
                if chainable.cr_lp && input.down && input.lp {
                    context.next = Some(Box::new(crouching::LightPunch));
                }
            }
        }
    }
}
