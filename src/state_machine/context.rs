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
    pub instructions: Vec<Modifiers>,
}

pub fn handle_modifiers(context: &mut Context, physics: &mut Physics) {
    if let Some(instruction) = context.modifier.instructions.get(context.modifier.index) {
        if instruction.on_frame == context.elapsed {
            physics.position += instruction.position;
            context.modifier.index += 1;
        }
    }
}
