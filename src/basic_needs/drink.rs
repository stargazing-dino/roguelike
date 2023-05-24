use bevy::prelude::{Query, With};
use big_brain::{prelude::ActionState, thinker::Actor};

use super::drink_action::DrinkAction;
use super::thirst::Thirst;

pub fn drink_action(
    mut thirsts: Query<&mut Thirst>,
    mut query: Query<(&Actor, &mut ActionState), With<DrinkAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        todo!()
    }
}
