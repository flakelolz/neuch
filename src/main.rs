use prelude::{HEIGHT, WIDTH};

mod debug;
mod game;
mod inputs;
mod world;

mod prelude {
    pub use crate::debug::*;
    pub use crate::inputs::*;
    pub use crate::world::*;
    pub use hecs::World;
    pub use raylib::prelude::*;

    pub const WIDTH: i32 = 384;
    pub const HEIGHT: i32 = 216;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH * 3, HEIGHT * 3)
        .title("Neuch")
        .build();
    rl.set_target_fps(60);
    game::game(&mut rl, &thread);
}
