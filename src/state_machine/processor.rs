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
        input: &Input,
        physics: &mut Physics,
        character: &Character,
        animator: &mut Animator,
    ) {
        self.current.on_update(context, input, physics);

        handle_transition(self, context, input, physics, character, animator);
    }
}
