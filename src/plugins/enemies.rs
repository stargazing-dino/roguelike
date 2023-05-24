use bevy::prelude::{IntoSystemConfig, Plugin};
use big_brain::{BigBrainPlugin, BigBrainSet};

use crate::{
    basic_needs::drink::drink_action,
    combat::{attack::attack_action, attack_opportunity::attack_opportunity_scorer},
    vision::explore_tiles::explore_tiles,
};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BigBrainPlugin)
            .add_system(explore_tiles)
            .add_system(attack_action.in_set(BigBrainSet::Actions))
            .add_system(drink_action.in_set(BigBrainSet::Actions))
            .add_system(attack_opportunity_scorer.in_set(BigBrainSet::Scorers));
    }
}
