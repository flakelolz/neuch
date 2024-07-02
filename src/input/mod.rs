mod buffer;
mod config;
mod inputs;

pub use buffer::*;
pub use config::*;
pub use inputs::*;

use crate::prelude::*;

pub fn update_inputs(world: &mut World, rl: &mut RaylibHandle) {
    world
        .query_mut::<(&mut Input, &InputConfig, &mut InputBuffer, &Player)>()
        .into_iter()
        .for_each(|(_, (input, config, buffer, player))| {
            input.update(rl, config, player);
            buffer.update(input);
        });
}
