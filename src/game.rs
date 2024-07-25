use crate::prelude::*;

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread, configs: &mut Configs) {
    // Render targets
    let (mut target, mut ui_target) = create_render_targets(rl, thread);
    // Camera
    let mut camera = Camera2D {
        target: rvec2(0., 0.),
        offset: rvec2(0., 0.),
        rotation: 0.,
        zoom: 1.,
    };
    // World Setup
    let (mut world, mut collisions, mut hit_events) = world();
    let assets = Assets::new(rl, thread);

    // Debug pause
    let mut paused = false;
    let mut debug = Debug::default();
    let mut editor = Editor::new();

    configs.display.set_720(rl, &mut camera);
    // Main game loop
    while !rl.window_should_close() {
        // Input
        update_inputs(&mut world, rl);

        // Debug frame advance
        let mut advance = false;
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            paused = !paused;
            println!("Paused");
        } else if rl.is_key_pressed(KeyboardKey::KEY_BACKSLASH) {
            advance = true;
        }

        if !paused || advance {
            // Logic
            update_input_buffers(&mut world);
            physics_system(&mut world);
            update_state(&mut world);
            collisions.update(&mut world, &mut hit_events);
            reaction_system(&mut world, &mut hit_events);
            frame_count(&mut world);
        }
        // Debug
        move_player(&mut world, rl);
        reset_position(&mut world, rl);
        debug.toggle(rl);

        // Calculate window
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let scale = (width / WIDTH).min(height / HEIGHT) as f32;

        // Mouse scale
        rl.set_mouse_scale(1.0 / scale, 1.0 / scale);
        rl.set_mouse_offset(rvec2(
            -(rl.get_screen_width() as f32 - (FWIDTH * scale)) * 0.5,
            -(rl.get_screen_height() as f32 - (FHEIGHT * scale)) * 0.5,
        ));

        // Drawing
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        {
            // Render to sprite texture target
            let mut d = d.begin_texture_mode(thread, &mut target);

            if !paused || advance {
                d.clear_background(Color::BLANK);
                animation(&mut d, &world, &assets);
            }
        }
        {
            // Render to UI texture target
            let mut d = d.begin_texture_mode(thread, &mut ui_target);
            // Debug
            if !paused || advance {
                d.clear_background(Color::BLANK);
                show_frame_count(&world, &mut d, &debug);
                show_state(&world, &mut d, &debug);
                show_context(&world, &mut d, &debug);
                show_inputs(&world, &mut d, &debug);
            }
        }

        let mut d = d.begin_mode2D(camera);
        rendering(&mut target, &mut ui_target, &mut d);
        d.draw_fps(WIDTH - 100, 10);
        show_proximity_boxes(&world, &mut d, &debug);
        show_pushboxes(&world, &mut d, &debug);
        show_hurtboxes(&world, &mut d, &debug);
        show_hitboxes(&world, &mut d, &debug);
        show_position(&world, &mut d, &debug);
        editor.show_editor(&mut world, &mut d, &debug);
    }
}
