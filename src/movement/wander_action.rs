// use bevy::prelude::{warn, Commands, Query, With};
// use bevy_ecs_tilemap::tiles::TileStorage;
// use big_brain::{prelude::ActionState, thinker::Actor};
// use bracket_pathfinding::prelude::a_star_search;

// use crate::{ai::wander_action::WanderAction, map::tile_query::GridQuery};

// use super::move_ability::MoveAbility;

// pub fn wander_action(
//     mut commands: Commands,
//     mut move_query: Query<&mut MoveAbility>,
//     mut actions: Query<(&Actor, &mut ActionState), With<WanderAction>>,
//     tilemap_storage_query: Query<&TileStorage, With<MapTilemap>>,
// ) {
//     for (Actor(actor), mut state) in actions.iter_mut() {
//         match *state {
//             ActionState::Requested => *state = ActionState::Executing,
//             ActionState::Cancelled => *state = ActionState::Failure,
//             ActionState::Executing => {
//                 if let Ok(agent_index) = agents.get(*actor) {
//                     let tile_storage = tilemap_storage_query.single();
//                     // Find a random valid spot on the map within some radius of agent
//                     // find a navigation path to it
//                     let query = GridQuery::new()
//                         .add_filter(&WalkableFilter(true))
//                         .add_filter(&DistanceFilter {
//                             origin: tile_storage.index_to_point2d(agent_index.0),
//                             max_distance: 3.0,
//                         })
//                         .add_filter(&ExcludeFilter {
//                             exclude: vec![agent_index.0],
//                         });
//                     let walkable_tiles = query.query(&grid);
//                     let target = tile_storage.rand_from_query(rng.get_mut(), &query);

//                     if let Some(target_location) = target {
//                         let path = a_star_search(agent_index.0, target_location.0, &*tile_storage);

//                         if path.success {
//                             commands
//                                 .entity(*actor)
//                                 .insert(MovementPath { path: path.steps });
//                             *state = ActionState::Success;
//                         } else {
//                             warn!("Unable to find a valid path to target");
//                             *state = ActionState::Failure;
//                         }
//                     } else {
//                         warn!("Unable to find a random walkable tile in range.");
//                         *state = ActionState::Failure;
//                     }
//                 } else {
//                     warn!("No agent to perform the action");
//                     *state = ActionState::Failure;
//                 }
//             }
//             _ => {}
//         }
//     }
// }
