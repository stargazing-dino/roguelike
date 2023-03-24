use bevy::prelude::Component;
use big_brain::prelude::ActionBuilder;

/// An action that represents an attack
#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct AttackAction;
