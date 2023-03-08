use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

use crate::{
    components::walkable::Walkable,
    constants::TileType,
    tilemap::{Tilemap, URect},
};

pub fn generate_tilemap(size: TilemapSize) -> (Tilemap, (usize, usize)) {
    let mut tilemap = Tilemap::new(size.x as usize, size.y as usize);
    let rng = &mut rand::thread_rng();

    let mut rooms: Vec<URect> = Vec::new();

    const MAX_ROOMS: i32 = 6;
    const MIN_SIZE: i32 = 4;
    const MAX_SIZE: i32 = 10;

    for _ in 0..MAX_ROOMS {
        let w = rng.gen_range(MIN_SIZE..=MAX_SIZE) as usize;
        let h = rng.gen_range(MIN_SIZE..=MAX_SIZE) as usize;
        let x = rng.gen_range(1..size.x as usize - w - 1);
        let y = rng.gen_range(1..size.y as usize - h - 1);
        let new_room = URect::new(x, y, w, h);
        let mut ok = true;

        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }

        if ok {
            tilemap.apply_rect_border_to_map(&new_room, TileType::Cactus);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                // let fifty_fifty = rng.gen_range(0..2);
                let (start_x, end_x) = if new_x < prev_x {
                    (new_x, prev_x)
                } else {
                    (prev_x, new_x)
                };

                tilemap.apply_line_to_map(
                    (start_x, new_y),
                    (end_x, new_y),
                    TileType::GroundPathLarge,
                );
            }

            rooms.push(new_room);
        }
    }

    tilemap.apply_border_to_map(TileType::GroundPathLarge);

    (tilemap, rooms[0].center())
}

/// Adds entities to the world from a tilemap
pub fn entities_from_tilemap(
    tilemap: Tilemap,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    for x in 0..tilemap.width() {
        for y in 0..tilemap.height() {
            let tile = tilemap.get_tile(x, y);
            let tile_texture_index = tile.index();
            let tile_pos = TilePos {
                x: x as u32,
                y: y as u32,
            };

            let mut entity_commands = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id,
                texture_index: tile_texture_index,
                ..Default::default()
            });

            // TODO: Implement groups
            match tile {
                TileType::Ground => {
                    entity_commands.insert(Walkable {});
                }
                _ => {}
            }

            let tile_entity = entity_commands.id();

            commands.entity(tilemap_id.0).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}
