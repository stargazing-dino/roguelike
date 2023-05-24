use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileVisible};

use crate::{plugins::player::Player, Map};

use super::{explored_tiles::ExploredTiles, viewshed::Viewshed};

// TODO: Is there a way to only run this when the viewshed changes rather than every frame?
pub fn viewable_tiles_for_player(
    mut query: Query<(&Viewshed, &ExploredTiles), With<Player>>,
    map_storage_query: Query<&TileStorage, With<Map>>,
    mut tile_query: Query<(&mut TileVisible, &TilePos)>,
) {
    let map = map_storage_query.single();
    let mut visible_tiles: Vec<TilePos> = Vec::new();

    // collect all the visible tiles inside the viewshed query
    for (viewshed, explored_tiles) in query.iter_mut() {
        visible_tiles.extend(viewshed.visible_tiles.iter());
        visible_tiles.extend(explored_tiles.0.iter());
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
