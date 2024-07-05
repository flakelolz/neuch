use crate::prelude::*;

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
    pub fn update(
        &mut self,
        context: &mut Context,
        buffer: &InputBuffer,
        physics: &mut Physics,
        character: &Character,
        animator: &mut Animator,
    ) {
        handle_transition(self, context, buffer, physics, character, animator);
        self.current.on_update(context, buffer, physics);
    }
}
