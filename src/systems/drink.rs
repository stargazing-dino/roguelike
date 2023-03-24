use bevy::prelude::{Query, With};
use big_brain::{prelude::ActionState, thinker::Actor};

use crate::{actions::drink::DrinkAction, components::attributes::thirst::Thirst};

pub fn drink_action(
    mut thirsts: Query<&mut Thirst>,
    mut query: Query<(&Actor, &mut ActionState), With<DrinkAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        todo!()
    }
}
