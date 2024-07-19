#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::Configs;
use prelude::*;

mod animations;
mod assets;
mod collision;
mod config;
mod data;
mod debug;
mod game;
mod input;
mod physics;
mod reaction;
mod rendering;
mod state_machine;
mod utils;
mod world;

mod prelude {
    pub use crate::animations::*;
    pub use crate::assets::*;
    pub use crate::collision::*;
    pub use crate::config::*;
    pub use crate::data::*;
    pub use crate::debug::*;
    pub use crate::input::*;
    pub use crate::physics::*;
    pub use crate::reaction::*;
    pub use crate::rendering::*;
    pub use crate::state_machine::*;
    pub use crate::utils::*;
    pub use crate::world::*;
    pub use hecs::{Entity, World};
    pub use include_dir::{include_dir, Dir};
    pub use raylib::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;

    pub static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

    pub const WIDTH: i32 = 640;
    pub const HEIGHT: i32 = 360;
    pub const FWIDTH: f32 = WIDTH as f32;
    pub const FHEIGHT: f32 = HEIGHT as f32;
    pub const WIDTH_3S: i32 = 416;
    pub const HEIGHT_3S: i32 = 234;
    pub const GROUND_OFFSET: i32 = 200;
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "--update" {
        utils::update_all_data();
        std::process::exit(0);
    }

    // App Setup
    let mut configs = Configs::default();

    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Neuch").build();
    rl.set_target_fps(60);

    let font = rl
        .load_font_ex(&thread, "assets/Kenney Mini.ttf", 512, None)
        .expect("Failed to load font");

    rl.gui_set_font(&font);

    game::game(&mut rl, &thread, &mut configs);
}
