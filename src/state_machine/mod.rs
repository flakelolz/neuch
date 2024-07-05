mod context;
mod processor;
mod states;
mod transitions;

pub use self::{context::*, processor::*, states::*, transitions::*};
use crate::prelude::*;

pub fn update_state(world: &mut World) {
    for (_, (state, buffer, physics, character, animator)) in world.query_mut::<(
        &mut StateMachine,
        &InputBuffer,
        &mut Physics,
        &Character,
        &mut Animator,
    )>() {
        state
            .processor
            .update(&mut state.context, buffer, physics, character, animator);
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
