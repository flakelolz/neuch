use crate::prelude::*;

pub fn draw_player(d: &mut RaylibTextureMode<RaylibDrawHandle>, world: &World, assets: &Assets) {
    world
        .query::<(&Physics, &Player, &Character)>()
        .into_iter()
        .for_each(|(_, (physics, player, character))| {
            let texture = match player {
                Player::One => Some(&assets.ken),
                Player::Two => None,
            };

            if let Some(texture) = texture {
                let (pos_x, pos_y) = world_to_screen_vec(physics.position);
                let source_rec = rrect(0, 0, texture.width, texture.height);
                let dest_rec = rrect(pos_x, pos_y, texture.width, texture.height);
                let origin = rvec2(180, 190);
                let rotation = 0.;
                let tint = Color::WHITE;

                d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint)
            }
        });
}
