use crate::prelude::*;

pub fn draw_player(d: &mut RaylibTextureMode<RaylibDrawHandle>, world: &World, assets: &Assets) {
    world
        .query::<(&Physics, &Player)>()
        .into_iter()
        .for_each(|(_, (physics, player))| {
            let texture = match player {
                Player::One => Some(&assets.ken),
                Player::Two => None,
            };

            if let Some(texture) = texture {
                let source_rec = rrect(0, 0, texture.width, texture.height);
                let dest_rec = rrect(
                    physics.position.x,
                    physics.position.y,
                    texture.width,
                    texture.height,
                );
                let origin = rvec2(texture.width / 2, texture.height / 2);
                let rotation = 0.;
                let tint = Color::WHITE;

                d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint)
            }
        })
}
