use crate::*;

/// Renders ascii characters.
pub fn render_ascii<'a>(
    ctx: &mut BTerm,
    camera: &Camera,
    positions: &Components<Point>,
    multi_sprites: &Components<MultiSprite>,
    sprites: &Components<Sprite>,
) {
    #[cfg(not(feature = "headless"))]
    {
        for (pos, sprite) in join!(&positions && &multi_sprites) {
            sprite.unwrap().tile.render(
                ctx,
                Point::new(
                    pos.unwrap().x - camera.position.x,
                    pos.unwrap().y - camera.position.y,
                ),
            );
        }
        for (pos, sprite) in join!(&positions && &sprites) {
            let pos = pos.unwrap();
            let sprite = sprite.unwrap();
            ctx.set(
                pos.x - camera.position.x,
                pos.y - camera.position.y,
                sprite.fg,
                sprite.bg,
                sprite.glyph,
            );
        }
    }
}

/// Renders 2d tile sprites characters.
/// Only available using the `opengl` feature.
#[cfg(feature = "opengl")]
pub fn render_sprites<'a>(
    ctx: &mut BTerm,
    camera: &Camera,
    positions: &Components<Point>,
    sprites: &Components<SpriteIndex>,
    viewshed: Option<&Viewshed>,
) {
    #[cfg(not(feature = "headless"))]
    for (pos, sprite) in join!(&positions && &sprites) {
        let pos = pos.unwrap();
        let sprite = sprite.unwrap();

        if viewshed.is_none() || viewshed.unwrap().visible_tiles.contains(&pos) {
            ctx.add_sprite(
                Rect::with_size(
                    (pos.x - camera.position.x) * 1,
                    (pos.y - camera.position.y) * 1,
                    // TODO make this dynamic.
                    1,
                    1,
                ),
                0,
                RGBA::named(WHITE),
                sprite.0,
            );
        // TODO this will not hide units that are not creeps, use a better way of checking for enemy units.
        } else if sprite.0 != 9 {
            ctx.add_sprite(
                Rect::with_size(
                    (pos.x - camera.position.x) * 1,
                    (pos.y - camera.position.y) * 1,
                    // TODO make this dynamic.
                    1,
                    1,
                ),
                0,
                RGBA::from_u8(160, 160, 160, 255),
                sprite.0,
            );
        }
    }
}
