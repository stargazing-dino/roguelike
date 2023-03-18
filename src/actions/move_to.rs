use bevy::prelude::Component;
use big_brain::prelude::ActionBuilder;

/// Action that moves to a target.
#[derive(Component, Debug, Clone, ActionBuilder)]
pub(crate) struct MoveToAction;
