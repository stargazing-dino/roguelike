use bevy::prelude::Component;

// TODO: consider if opaque is a better name
/// This component is used to mark tiles that are walkable.
#[derive(Component)]
pub struct Obstacle {}
