use bevy::prelude::Component;

/// This component is used to mark tiles that are not walkable.
#[derive(Component, Debug, Copy, Clone)]
pub struct Obstacle;
