use bevy::prelude::{warn, Commands, Query, With};
use big_brain::{prelude::ActionState, thinker::Actor};

use crate::{actions::wander::WanderAction, components::attributes::move_ability::MoveAbility};

pub fn wander_action(
    mut commands: Commands,
    mut move_query: Query<&mut MoveAbility>,
    mut actions: Query<(&Actor, &mut ActionState), With<WanderAction>>,
) {
    todo!()
    // for (Actor(actor), mut state) in actions.iter_mut() {
    //     match *state {
    //         ActionState::Requested => *state = ActionState::Executing,
    //         ActionState::Cancelled => *state = ActionState::Failure,
    //         ActionState::Executing => {
    //             if let Ok(agent_index) = agents.get(*actor) {
    //                 // Find a random valid spot on the map within some radius of agent
    //                 // find a navigation path to it
    //                 let query = TileQuery {
    //                     walkable: Some(true),
    //                     distance: Some((3.0, agent_index.0)),
    //                     exclude: Some(vec![agent_index.0]),
    //                     ..default()
    //                 };
    //                 let target = map.rand_from_query(rng.get_mut(), &query);

    //                 if let Some(target_location) = target {
    //                     let path = a_star_search(agent_index.0, target_location.0, &*map);

    //                     if path.success {
    //                         commands
    //                             .entity(*actor)
    //                             .insert(MovementPath { path: path.steps });
    //                         *state = ActionState::Success;
    //                     } else {
    //                         warn!("Unable to find a valid path to target");
    //                         *state = ActionState::Failure;
    //                     }
    //                 } else {
    //                     warn!("Unable to find a random walkable tile in range.");
    //                     *state = ActionState::Failure;
    //                 }
    //             } else {
    //                 warn!("No agent to perform the action");
    //                 *state = ActionState::Failure;
    //             }
    //         }
    //         _ => {}
    //     }
    // }
}
