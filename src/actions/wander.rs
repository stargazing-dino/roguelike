use bevy::prelude::Component;
use big_brain::prelude::ActionBuilder;

/// Action that simply wanders randomly
#[derive(Component, Debug, Clone, ActionBuilder)]
pub(crate) struct WanderAction;
