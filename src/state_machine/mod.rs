mod context;
mod states;
use crate::prelude::*;
pub use context::*;

use self::states::*;

pub fn update_state(world: &mut World) {
    for (_, (state, input, physics)) in
        world.query_mut::<(&mut StateMachine, &Input, &mut Physics)>()
    {
        state.processor.update(&mut state.context, input, physics);
    }
}

pub trait State: Send + Sync {
    fn on_enter(&mut self, context: &mut Context, input: &Input, physics: &mut Physics);
    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics);
    fn on_exit(&mut self, context: &mut Context, input: &Input, physics: &mut Physics);
}

#[derive(Default)]
pub struct StateMachine {
    processor: StateProcessor,
    context: Context,
}

pub struct StateProcessor {
    current: Box<dyn State>,
}

impl Default for StateProcessor {
    fn default() -> Self {
        Self {
            current: Box::new(Standing),
        }
    }
}

impl StateProcessor {
    fn update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        self.current.on_update(context, input, physics);

        if let Some(mut next) = context.next.take() {
            self.current.on_exit(context, input, physics);
            next.on_enter(context, input, physics);
            self.current = next;
        }
    }
}
