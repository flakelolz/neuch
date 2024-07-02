mod config;
mod inputs;

pub use config::*;
pub use inputs::*;

use crate::prelude::*;

pub fn update_inputs(world: &mut World, rl: &mut RaylibHandle) {
    world
        .query_mut::<(&mut Input, &InputConfig, &Player)>()
        .into_iter()
        .for_each(|(_, (input, config, player))| {
            input.update(rl, config, player);
        });
}
