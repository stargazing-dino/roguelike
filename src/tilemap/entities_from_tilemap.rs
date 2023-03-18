use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    components::{obstacle::Obstacle, walkable::Walkable},
    constants::TileType,
    tilemap::Tilemap,
};

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

            // Woah! Check this out :]
            // matches!(tile, TileType::GroundPathPartial | TileType::GroundPathSmall | TileType::GroundPathLarge)

            // TODO: Implement groups
            match tile {
                TileType::Ground
                | TileType::GroundWithDirt
                | TileType::GroundPathPartial
                | TileType::GroundPathSmall
                | TileType::GroundPathLarge
                | TileType::GroundWithGrass
                | TileType::GroundWithWeeds
                | TileType::GroundWithLatticedGrass => {
                    entity_commands.insert(Walkable);
                }
                TileType::Cactus => {
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
