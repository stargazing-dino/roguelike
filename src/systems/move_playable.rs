use bevy::{
    prelude::{Query, Res, With},
    time::Time,
};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use leafwing_input_manager::prelude::ActionState;
use num_integer::Roots;

use crate::{
    components::{last_moved_time::LastMovedTime, path::Path, player::Player},
    constants::MAP_SIZE,
    input_managers::players::PlayerAction,
    Map,
};

pub trait TileDistance {
    fn distance(&self, other: &Self) -> u32;
}

impl TileDistance for TilePos {
    fn distance(&self, other: &Self) -> u32 {
        let x = (self.x as i32 - other.x as i32).abs();
        let y = (self.y as i32 - other.y as i32).abs();

        (x.pow(2) + y.pow(2)).sqrt() as u32
    }
}

// TODO: Component
const MOVE_COOLDOWN: f64 = 0.1;

pub fn move_player(
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut query: Query<(&mut TilePos, &mut LastMovedTime), With<Player>>,
    map_storage_query: Query<&TileStorage, With<Map>>,
    walkable_query: Query<With<Path>>,
    time: Res<Time>,
) {
    let action_state = action_query.single();
    let map = map_storage_query.single();

    for (mut tile_pos, mut last_moved_time) in query.iter_mut() {
        let current_time = time.elapsed_seconds_f64();
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
        let passed_threshold = current_time - last_moved_time.0 > MOVE_COOLDOWN;

        if walkable_query.get(next).is_ok() && passed_threshold {
            // The move cooldown has expired, so move the player
            // Update the last moved time to the current time
            last_moved_time.0 = current_time;

            tile_pos.x = temp.x;
            tile_pos.y = temp.y;
        }
    }
}
