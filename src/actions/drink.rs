use bevy::prelude::Component;
use big_brain::prelude::ActionBuilder;

/// An action that represents a drink
#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct DrinkAction;
