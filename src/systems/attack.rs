use bevy::prelude::{Query, With};
use big_brain::{prelude::ActionState, thinker::Actor};

use crate::{
    actions::attack::AttackAction,
    components::{attributes::health::Health, target::Target},
};

pub fn attack_action(
    mut healths: Query<&mut Health>,
    mut query: Query<(&Actor, &Target, &mut ActionState), With<AttackAction>>,
) {
    for (Actor(actor), Target(target), mut state) in query.iter_mut() {
        if let Ok(mut target_health) = healths.get_mut(*target) {
            match *state {
                ActionState::Requested => {
                    target_health.value -= 10.0; // Deal 10 damage to the target.
                    *state = ActionState::Success;
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}
