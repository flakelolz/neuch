mod buffer;
mod config;
mod inputs;
mod test;
mod utils;

pub use buffer::*;
pub use config::*;
pub use inputs::*;
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
        .query_mut::<(&mut Input, &mut InputBuffer, &mut StateMachine)>()
        .into_iter()
        .for_each(|(_, (input, buffer, machine))| {
            buffer.update(input);
        });
}
