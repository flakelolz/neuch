use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub character: Option<CharacterInfo>,
    pub next: Option<Box<dyn State>>,
    pub elapsed: u32,
    pub duration: u32,
    pub modifiers: Instructions,
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
                    physics.position += position.value;
                    context.modifiers.index += 1;
                }
            }
        }

        if let Some(cancellable) = &instructions.cancellable {
            for action in cancellable {
                if context.elapsed >= action.on_frame {
                    for state in &action.states {
                        state.set(buffer, &mut context.next);
                    }
                }
            }
        }
    }
}
