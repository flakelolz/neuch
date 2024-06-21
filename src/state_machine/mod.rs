mod context;
mod states;
mod transitions;
use crate::prelude::*;
pub use context::*;

use self::{states::*, transitions::handle_transition};

pub fn update_state(world: &mut World) {
    for (_, (state, input, physics)) in
        world.query_mut::<(&mut StateMachine, &Input, &mut Physics)>()
    {
        state.processor.update(&mut state.context, input, physics);
    }
}

pub trait State: Send + Sync {
    fn name(&self) -> &'static str;
    fn on_enter(&mut self, context: &mut Context, input: &Input, physics: &mut Physics);
    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics);
    fn on_exit(&mut self, context: &mut Context, input: &Input, physics: &mut Physics);
}

#[derive(Default)]
pub struct StateMachine {
    pub processor: StateProcessor,
    context: Context,
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

impl StateProcessor {
    fn update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        self.current.on_update(context, input, physics);

        handle_transition(self, context, input, physics);
    }
}
