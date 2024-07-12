use crate::prelude::*;
pub fn create_render_targets(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    configs: &Configs,
) -> (RenderTexture2D, RenderTexture2D) {
    let sprite_target = rl
        .load_render_texture(thread, WIDTH_3S as u32, HEIGHT_3S as u32)
        .unwrap();
    let ui_target = rl
        .load_render_texture(thread, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    (sprite_target, ui_target)
}

pub fn reset_render_targets(ui: &mut RenderTexture2D, configs: &Configs) {
    ui.texture.width = WIDTH;
    ui.texture.height = HEIGHT;
}

pub fn rendering(
    sprite: &mut RenderTexture2D,
    ui: &mut RenderTexture2D,
    d: &mut impl RaylibDraw,
    configs: &Configs,
) {
    d.draw_texture_pro(
        sprite.texture(),
        rrect(0.0, 0.0, sprite.texture.width, -sprite.texture.height),
        rrect(0, 0, WIDTH, HEIGHT),
        rvec2(0, 0),
        0.0,
        Color::WHITE,
    );

    d.draw_texture_pro(
        ui.texture(),
        rrect(0.0, 0.0, ui.texture.width, -ui.texture.height),
        rrect(0, 0, WIDTH, HEIGHT),
        rvec2(0, 0),
        0.0,
        Color::WHITE,
    );
}
