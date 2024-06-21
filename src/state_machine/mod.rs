mod context;
mod states;
use crate::prelude::*;
pub use context::*;

use self::states::*;

pub fn update_state(world: &mut World) {
    for (_, state) in world.query_mut::<&mut StateMachine>() {
        state.processor.update(&mut state.context);
    }
}

pub trait State: Send + Sync {
    fn on_enter(&mut self, context: &mut Context);
    fn on_update(&mut self, context: &mut Context);
    fn on_exit(&mut self, context: &mut Context);
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
    fn update(&mut self, context: &mut Context) {
        self.current.on_update(context);

        if let Some(mut next) = context.next.take() {
            self.current.on_exit(context);
            next.on_enter(context);
            self.current = next;
        }
    }
}
