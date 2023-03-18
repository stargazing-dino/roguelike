use bevy::prelude::{IntoSystemConfig, Plugin};
use big_brain::{BigBrainPlugin, BigBrainSet};

use crate::systems::attack::attack_action;

pub(crate) struct ThinkersPlugin;

impl Plugin for ThinkersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BigBrainPlugin)
            .add_system(attack_action.in_set(BigBrainSet::Actions));
    }
}
