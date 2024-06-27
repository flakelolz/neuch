use crate::prelude::*;

pub struct Assets {
    pub ken: Texture2D,
}

impl Assets {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let ken = rl.load_texture(thread, "assets/sprites/Ken.png").unwrap();
        Self { ken }
    }
}
