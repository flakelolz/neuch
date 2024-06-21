use crate::prelude::*;

// This should be the duration of the player entity
#[derive(Default)]
pub struct Context {
    pub physics: Physics,
    pub input: Input,
    pub next: Option<Box<dyn State>>,
}

pub fn update_context(world: &mut World) {
    for (_, (state, physics, input)) in
        world.query_mut::<(&mut StateMachine, &mut Physics, &Input)>()
    {
        state.context.input = *input;
        physics.velocity = state.context.physics.velocity;
    }
}
