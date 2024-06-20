use prelude::{HEIGHT, WIDTH};

mod assets;
mod debug;
mod game;
mod inputs;
mod world;

mod prelude {
    pub use crate::assets::*;
    pub use crate::debug::*;
    pub use crate::inputs::*;
    pub use crate::world::*;
    pub use hecs::World;
    pub use raylib::prelude::*;

    pub const WIDTH: i32 = 384;
    pub const HEIGHT: i32 = 216;
    pub const FWIDTH: f32 = WIDTH as f32;
    pub const FHEIGHT: f32 = HEIGHT as f32;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Neuch")
        .resizable()
        .build();
    rl.set_target_fps(60);

    let mut target = rl
        .load_render_texture(&thread, WIDTH as u32, HEIGHT as u32)
        .unwrap();
    game::game(&mut rl, &thread, &mut target);
}
