use bevy::prelude::Component;

#[derive(Component, Debug, Copy, Clone)]
pub(crate) struct Hunger {
    /// How fast the entity gets hungry.
    pub per_second: f32,

    /// Current value of the hunger.
    pub value: f32,
}
