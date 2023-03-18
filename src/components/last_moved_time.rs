use bevy::prelude::Component;

/// This component is to know when an entity was last moved.
/// Useful for limiting the number of times an entity can move in a duration.
#[derive(Component, Debug, Copy, Clone)]
pub struct LastMovedTime(pub f64);
