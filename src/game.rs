use crate::prelude::*;

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread, configs: &mut Configs) {
    // Render targets
    let (mut target, mut ui_target) = create_render_targets(rl, thread, configs);
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
            frame_count(&mut world);
            physics_system(&mut world);
            collisions.update(&mut world, &mut hit_events);
            reaction_system(&mut world, &mut hit_events);
            update_state(&mut world);
        }
        // Debug
        reset_position(&mut world, rl);
        change_resolution(rl, configs, &mut camera);

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
                show_frame_count(&world, &mut d);
                show_state(&world, &mut d);
                show_position(&world, &mut d);
                show_context(&world, &mut d);
                show_inputs(&world, &mut d);
                show_hurtboxes(&world, &mut d);
                show_hitboxes(&world, &mut d);
            }
            d.draw_fps(WIDTH - 100, 10);
        }

        let mut d = d.begin_mode2D(camera);
        rendering(&mut target, &mut ui_target, &mut d);
    }
}
