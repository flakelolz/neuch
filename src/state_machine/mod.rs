mod context;
mod states;
mod transitions;

pub use self::{context::*, states::*, transitions::*};
use crate::prelude::*;

pub fn update_state(world: &mut World) {
    for (_, (state, buffer, physics, character, animator)) in world.query_mut::<(
        &mut StateMachine,
        &InputBuffer,
        &mut Physics,
        &Character,
        &mut Animator,
    )>() {
        let processor = &mut state.processor;
        let context = &mut state.context;

        match find_action(character, &processor.current.name()) {
            Some(action) => {
                // FIX: Only needed at the start of the game right now.
                if animator.keyframes.is_empty() {
                    animator.keyframes.clone_from(&action.timeline);
                }

                if context.reaction.hitstop == 0 {
                    context.elapsed += 1;
                }

                if context.elapsed > action.total && action.looping {
                    context.elapsed = 1;
                }
            }
            None => {
                eprintln!("Action not found");
            }
        }

        processor.current.on_update(context, buffer, physics);

        handle_transition(processor, context, buffer, physics, character, animator);
        handle_modifiers(context, buffer, physics);
    }
}

pub trait State: Send + Sync {
    fn name(&self) -> String;
    fn on_enter(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics);
    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics);
    fn on_exit(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics);
}

#[derive(Default)]
pub struct StateMachine {
    pub processor: StateProcessor,
    pub context: Context,
}

pub struct StateProcessor {
    pub current: Box<dyn State>,
}

impl Default for StateProcessor {
    fn default() -> Self {
        Self {
            current: Box::new(standing::Idle),
        }
    }
}
