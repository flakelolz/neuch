mod context;
mod states;
mod transitions;

pub use self::{context::*, states::*, transitions::*};
use crate::prelude::*;

pub fn update_state(world: &mut World, collisions: &mut Collisions) {
    for (_, (state, buffer, physics, character, animator)) in world.query_mut::<(
        &mut StateMachine,
        &mut InputBuffer,
        &mut Physics,
        &Character,
        &mut Animator,
    )>() {
        state.context.ctx.reaction.block_height = Block::None;
        if down(buffer) {
            state.context.ctx.reaction.crouching = true;

            if backward(buffer, &physics.facing_left) {
                state.context.ctx.reaction.block_height = Block::Low;
            }
        } else if backward(buffer, &physics.facing_left) {
            state.context.ctx.reaction.block_height = Block::High;
            state.context.ctx.reaction.crouching = false;
        } else {
            state.context.ctx.reaction.block_height = Block::None;
        }

        match find_action(character, &state.processor.current.name()) {
            Some(action) => {
                // FIX: Only needed at the start of the game right now.
                if animator.keyframes.is_empty() {
                    animator.keyframes.clone_from(&action.timeline);
                }

                if state.context.ctx.reaction.hitstop == 0 {
                    state.context.elapsed += 1;
                }

                if state.context.elapsed > action.total && action.looping {
                    state.context.elapsed = 1;
                }
            }
            None => {
                eprintln!("Action not found!!!");
                state.context.ctx.next = Some(Box::new(standing::Idle));
            }
        }

        state
            .processor
            .current
            .on_update(&mut state.context, buffer, physics);

        handle_transition(state, buffer, physics, character, animator, collisions);
        // handle_spawns(state, physics, character, animator);
        handle_modifiers(&mut state.context, buffer, physics);
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

impl StateProcessor {
    pub fn new(state: Box<dyn State>) -> Self {
        Self { current: state }
    }
}
