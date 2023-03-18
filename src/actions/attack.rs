use bevy::prelude::Component;
use big_brain::prelude::ActionBuilder;

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct AttackAction;
