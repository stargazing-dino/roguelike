use bevy::prelude::{Input, KeyCode, Query, Res, With};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    components::{
        player::{Player, PlayerAction},
        walkable::Walkable,
    },
    constants::MAP_SIZE,
    Map,
};

pub fn move_player(
    // keys: Res<Input<KeyCode>>,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut query: Query<&mut TilePos, With<Player>>,
    map_storage_query: Query<&TileStorage, With<Map>>,
    walkable_query: Query<With<Walkable>>,
) {
    let action_state = action_query.single();
    let map = map_storage_query.single();

    for mut tile_pos in query.iter_mut() {
        let mut temp = *tile_pos;

        if action_state.pressed(PlayerAction::Left) && tile_pos.x > 0 {
            temp.x -= 1;
        }
        if action_state.pressed(PlayerAction::Right) && tile_pos.x < MAP_SIZE.x - 1 {
            temp.x += 1;
        }
        if action_state.pressed(PlayerAction::Up) && tile_pos.y < MAP_SIZE.y - 1 {
            temp.y += 1;
        }
        if action_state.pressed(PlayerAction::Down) && tile_pos.y > 0 {
            temp.y -= 1;
        }

        let next = map.get(&temp).unwrap();

        if walkable_query.get(next).is_ok() {
            tile_pos.x = temp.x;
            tile_pos.y = temp.y;
        }
    }
}
