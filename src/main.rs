#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use prelude::{HEIGHT, WIDTH};

mod animations;
mod assets;
mod data;
mod debug;
mod game;
mod input;
mod physics;
mod state_machine;
mod utils;
mod world;

mod prelude {
    pub use crate::animations::*;
    pub use crate::assets::*;
    pub use crate::data::*;
    pub use crate::debug::*;
    pub use crate::input::*;
    pub use crate::physics::*;
    pub use crate::state_machine::*;
    pub use crate::utils::*;
    pub use crate::world::*;
    pub use hecs::World;
    pub use include_dir::{include_dir, Dir};
    pub use raylib::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;

    pub static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

    pub const WIDTH: i32 = 384;
    pub const HEIGHT: i32 = 216;
    pub const FWIDTH: f32 = WIDTH as f32;
    pub const FHEIGHT: f32 = HEIGHT as f32;
    pub const GROUND: i32 = 180;
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    if args.len() > 1 && args[1] == "--update" {
        utils::update_all_data();
    }

    let (mut rl, thread) = raylib::init()
        .size(WIDTH * 3, HEIGHT * 3)
        .title("Neuch")
        .resizable()
        .build();
    rl.set_target_fps(60);

    let mut target = rl
        .load_render_texture(&thread, WIDTH as u32, HEIGHT as u32)
        .unwrap();
    game::game(&mut rl, &thread, &mut target);
}
