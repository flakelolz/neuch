use crate::prelude::*;

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread) {
    // Setup
    let mut world = world();
    let assets = Assets::new(rl, thread);

    // Main game loop
    while !rl.window_should_close() {
        // Input
        update_inputs(&mut world, rl);
        // Logic

        // Drawing
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::BLACK);
        d.draw_texture(&assets.ken, 0, 0, Color::WHITE);

        // Debug
        // show_inputs(&world, &mut d);
    }
}
