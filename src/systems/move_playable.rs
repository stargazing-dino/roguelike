use bevy::prelude::{Input, KeyCode, Query, Res, With};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::{
    components::{playable::Playable, walkable::Walkable},
    constants::MAP_SIZE,
    Map,
};

pub fn move_playable(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut TilePos, With<Playable>>,
    map_storage_query: Query<&TileStorage, With<Map>>,
    // This says the player can only walk on floor, but like, is that true bruh?
    floor_query: Query<With<Walkable>>,
) {
    let map = map_storage_query.single();

    for mut tile_pos in query.iter_mut() {
        let mut temp = *tile_pos;

        if keys.pressed(KeyCode::Left) && tile_pos.x > 0 {
            temp.x -= 1;
        }
        if keys.pressed(KeyCode::Right) && tile_pos.x < MAP_SIZE.x - 1 {
            temp.x += 1;
        }
        if keys.pressed(KeyCode::Up) && tile_pos.y < MAP_SIZE.y - 1 {
            temp.y += 1;
        }
        if keys.pressed(KeyCode::Down) && tile_pos.y > 0 {
            temp.y -= 1;
        }

        let next = map.get(&temp).unwrap();

        if floor_query.get(next).is_ok() {
            tile_pos.x = temp.x;
            tile_pos.y = temp.y;
        }
    }
}
