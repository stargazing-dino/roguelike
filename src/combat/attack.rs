use bevy::prelude::Component;
use bevy::prelude::{Query, With};
use big_brain::prelude::ActionBuilder;
use big_brain::{prelude::ActionState, thinker::Actor};

use super::health::Health;
use super::target::Target;

/// An action that represents an attack
#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct AttackAction;

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
