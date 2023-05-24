use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::movement::path::Path;

use super::{obstacle::Obstacle, tile_type::TileGroup, Tilemap};

/// Adds entities to the world from a tilemap
pub fn entities_from_tilemap(
    tilemap: &Tilemap,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    for x in 0..tilemap.width {
        for y in 0..tilemap.height {
            let tile = tilemap.get(x, y);
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

            match tile.group() {
                TileGroup::Ground => {
                    entity_commands.insert(Path);
                }
                TileGroup::Other => {
                    entity_commands.insert(Obstacle);
                }
                _ => {}
            }

            let tile_entity = entity_commands.id();

            commands.entity(tilemap_id.0).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}
