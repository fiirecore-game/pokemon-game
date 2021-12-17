use crate::engine::{Context, graphics::Color};

use worldlib::map::{manager::state::WorldMapState, TileId, WorldMap};

use crate::world::RenderCoords;

use self::texture::WorldTextures;

use super::npc::NpcTypes;

pub mod input;
pub mod manager;

pub mod texture;
pub mod warp;

pub fn draw(
    map: &WorldMap,
    world: &WorldMapState,
    ctx: &mut Context,
    textures: &WorldTextures,
    npc_types: &NpcTypes,
    screen: &RenderCoords,
    border: bool,
    color: Color,
) {
    let primary = textures
        .tiles
        .palettes
        .get(&map.palettes[0])
        .expect("Could not get primary palette for map!");
    let length = primary.height() as TileId;
    let secondary = textures
        .tiles
        .palettes
        .get(&map.palettes[1])
        .expect("Could not get secondary palette for map!");

    for yy in screen.top..screen.bottom {
        let y = yy - screen.offset.y;
        let render_y = (yy << 4) as f32 - screen.focus.y; // old = y_tile w/ offset - player x pixel
        let row = y.saturating_mul(map.width);

        for xx in screen.left..screen.right {
            let x = xx - screen.offset.x;
            let render_x = (xx << 4) as f32 - screen.focus.x;

            if !(x < 0 || y < 0 || y >= map.height as _ || x >= map.width as _) {
                let index = x as usize + row as usize;
                let tile = map.tiles[index];
                let (texture, tile) = if length > tile {
                    (primary, tile)
                } else {
                    (secondary, tile - length)
                };
                textures
                    .tiles
                    .draw_tile(ctx, texture, tile, render_x, render_y, color);
                // if let Some(door) = door {
                //     if door.position == index {
                //         textures.tiles.draw_door(ctx, door, render_x, render_y);
                //     }
                // }
            } else if border {
                let tile = map.border[if x % 2 == 0 {
                    //  x % 2 + if y % 2 == 0 { 0 } else { 2 }
                    if y % 2 == 0 {
                        0
                    } else {
                        2
                    }
                } else if y % 2 == 0 {
                    1
                } else {
                    3
                }];
                let (texture, tile) = if length > tile {
                    (primary, tile)
                } else {
                    (secondary, tile - length)
                };
                textures
                    .tiles
                    .draw_tile(ctx, texture, tile, render_x, render_y, color);
            }
        }
    }
    for npc in map.npcs.values().filter(|npc| !npc.character.hidden).chain(
        world
            .scripts
            .npcs
            .values()
            .filter(|(loc, ..)| loc == &map.id)
            .map(|(.., n)| n),
    ) {
        textures.npcs.draw(ctx, npc_types, npc, screen);
    }
    // for script in map.scripts.iter() {
    //     if script.alive() {
    //         if let Some(action) = script.actions.front() {
    //             match action {
    //                 WorldActionKind::Conditional { .. } => {
    //                     if script.option > 1 {
    //                         textures
    //                             .gui
    //                             .get(&texture::gui::GuiTexture::Condition)
    //                             .draw(ctx, position(162.0, 66.0));
    //                         draw_cursor(ctx, 170.0, 77.0 + (script.option - 2) as f32 * 16.0);
    //                     }
    //                 }
    //                 _ => (),
    //             }
    //         }
    //     }
    // }
}
