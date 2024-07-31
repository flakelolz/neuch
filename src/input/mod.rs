mod buffer;
mod config;
mod dashes;
mod inputs;
mod motions;
mod test;
mod utils;

pub use buffer::*;
pub use config::*;
pub use dashes::*;
pub use inputs::*;
pub use motions::*;
pub use utils::*;

use crate::prelude::*;

pub fn update_inputs(world: &mut World, rl: &mut RaylibHandle) {
    world
        .query_mut::<(&mut Input, &InputConfig, &Player)>()
        .into_iter()
        .for_each(|(_, (input, config, player))| {
            input.update(rl, config, player);
        });
}

pub fn update_input_buffers(world: &mut World) {
    world
        .query_mut::<(&mut Input, &mut InputBuffer, &mut StateMachine, &Physics)>()
        .into_iter()
        .for_each(|(_, (input, buffer, machine, physics))| {
            input.facing_left = physics.facing_left;
            input.facing_opponent = physics.facing_opponent;
            buffer.update(input);
            buffer.lockout_dash(&mut machine.context.ctx, &physics.facing_left, 6);
        });
}
