use bevy::prelude::Component;

/// This component is used to represent the health of an entity.
#[derive(Component, Debug, Copy, Clone)]
pub struct Health {
    pub value: f32,
}
