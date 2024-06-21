use crate::prelude::*;

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread, target: &mut RenderTexture2D) {
    // Setup
    let mut world = world();
    let assets = Assets::new(rl, thread);

    // Debug pause
    let mut paused = false;

    // Main game loop
    while !rl.window_should_close() {
        // Input
        update_inputs(&mut world, rl);

        // Debug frame advance
        let mut advance = false;
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            paused = !paused;
        } else if rl.is_key_pressed(KeyboardKey::KEY_BACKSLASH) {
            advance = true;
        }

        if !paused || advance {
            // Logic
            frame_count(&mut world);
            update_physics(&mut world);
            update_state(&mut world);
        }

        // Calculate window
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let scale = (width / WIDTH).min(height / HEIGHT) as f32;

        // Drawing
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        {
            // Render to texture
            let mut d = d.begin_texture_mode(thread, target);
            d.clear_background(Color::BLACK);
            draw_player(&mut d, &world, &assets);

            // Debug
            d.draw_fps(WIDTH - 30, 20);
            show_frame_count(&world, &mut d);
            // show_inputs(&world, &mut d);
            show_state(&world, &mut d);
            show_position(&world, &mut d);
        }

        // Render texture to screen with proper scaling
        d.draw_texture_pro(
            target.texture(),
            rrect(0.0, 0.0, target.texture.width, -target.texture.height),
            rrect(
                (d.get_screen_width() as f32 - (FWIDTH * scale)) * 0.5,
                (d.get_screen_height() as f32 - (FHEIGHT * scale)) * 0.5,
                FWIDTH * scale,
                FHEIGHT * scale,
            ),
            rvec2(0, 0),
            0.0,
            Color::WHITE,
        );
    }
}
