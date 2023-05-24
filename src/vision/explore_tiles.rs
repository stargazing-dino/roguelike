use bevy::prelude::*;

use super::{explored_tiles::ExploredTiles, viewshed::Viewshed};

pub fn explore_tiles(mut query: Query<(&mut ExploredTiles, &Viewshed), Changed<Viewshed>>) {
    for (mut explored_tiles, viewshed) in query.iter_mut() {
        for tile in viewshed.visible_tiles.iter() {
            if !explored_tiles.0.contains(tile) {
                explored_tiles.0.push(*tile);
            }
        }
    }
}
