use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileVisible};

use crate::{
    components::{obstacle::Obstacle, player::Player, viewshed::Viewshed},
    line_of_sight::Visibility,
    Map,
};

use super::move_playable::TileDistance;

pub fn update_viewshed(
    mut query: Query<(&mut Viewshed, &TilePos), Changed<TilePos>>,
    map_storage_query: Query<&TileStorage, With<Map>>,
    obstacle_query: Query<With<Obstacle>>,
) {
    let map = map_storage_query.single();

    for (mut viewshed, current_tile_pos) in query.iter_mut() {
        let viewable_range = viewshed.range;

        // viewshed has a list of visible tiles, so we need to clear it
        viewshed.visible_tiles.clear();

        // TODO: This is a naive implementation. We should instead iterate only those tiles within
        // the viewable range of the player, and then check if they are visible. This will be

        // We need to iterate over all tiles in the map
        for x in 0..map.size.x {
            for y in 0..map.size.y {
                // Get the tile position
                let tile_pos = TilePos { x, y };

                // Get the distance between the current tile and the player
                let distance = current_tile_pos.distance(&tile_pos);

                // If the distance is less than the viewable range, then we can see it
                if distance < viewable_range {
                    // If the tile is not an obstacle, then we can see it
                    let is_visible = map.is_visible(current_tile_pos, &tile_pos, |entity, _| {
                        obstacle_query.get(*entity).is_err()
                    });

                    if is_visible {
                        viewshed.visible_tiles.push(tile_pos);
                    }
                }
            }
        }
    }
}

// TODO: Is there a way to only run this when the viewshed changes rather than every frame?
pub fn update_tiles_for_viewshed(
    mut query: Query<&Viewshed, With<Player>>,
    map_storage_query: Query<&TileStorage, With<Map>>,
    mut tile_query: Query<(&mut TileVisible, &TilePos)>,
) {
    let map = map_storage_query.single();
    let mut visible_tiles: Vec<TilePos> = Vec::new();

    // collect all the visible tiles inside the viewshed query
    for viewshed in query.iter_mut() {
        visible_tiles.extend(viewshed.visible_tiles.iter());
    }

    for tile_pos in map.iter() {
        let tile_entity = tile_pos.unwrap();
        let (mut visibility, tile_pos) = tile_query.get_mut(tile_entity).unwrap();

        if visible_tiles.contains(tile_pos) {
            visibility.0 = true;
        } else {
            visibility.0 = false;
        }
    }
}
