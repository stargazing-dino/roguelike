use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

use crate::components::{obstruct::Obstruct, walkable::Walkable};

pub fn fill_tilemap_and_walls(
    floor_texture_index: TileTextureIndex,
    wall_texture_index: TileTextureIndex,
    player_pos: TilePos,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    let mut rng = rand::thread_rng();

    for x in 0..size.x {
        for y in 0..size.y {
            let is_horizontal_wall = x == 0 || x == size.x - 1;
            let is_vertical_wall = y == 0 || y == size.y - 1;
            let is_on_player = player_pos.x == x && player_pos.y == y;
            let is_inner_wall = if is_horizontal_wall || is_vertical_wall || is_on_player {
                false
            } else {
                rng.gen_bool(0.1)
            };
            let is_wall_piece = is_horizontal_wall || is_vertical_wall || is_inner_wall;
            let tile_pos = TilePos { x, y };
            let mut entity_commands = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id,
                texture_index: if is_wall_piece {
                    wall_texture_index
                } else {
                    floor_texture_index
                },
                ..Default::default()
            });

            if is_wall_piece {
                entity_commands.insert(Obstruct {});
            } else {
                entity_commands.insert(Walkable {});
            }

            let tile_entity = entity_commands.id();

            commands.entity(tilemap_id.0).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}
