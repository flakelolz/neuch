use prelude::{HEIGHT, WIDTH};

mod assets;
mod data;
mod debug;
mod draw;
mod game;
mod inputs;
mod physics;
mod utils;
mod world;

mod prelude {
    pub use crate::assets::*;
    pub use crate::data::*;
    pub use crate::debug::*;
    pub use crate::draw::*;
    pub use crate::inputs::*;
    pub use crate::physics::*;
    pub use crate::utils::*;
    pub use crate::world::*;
    pub use hecs::World;
    pub use miniserde::{json, Deserialize};
    pub use raylib::prelude::*;

    pub const WIDTH: i32 = 384;
    pub const HEIGHT: i32 = 216;
    pub const FWIDTH: f32 = WIDTH as f32;
    pub const FHEIGHT: f32 = HEIGHT as f32;
}

fn main() {
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
