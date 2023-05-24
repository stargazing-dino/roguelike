use bevy::prelude::Component;

#[derive(Component, Debug, Copy, Clone)]
pub struct Thirst {
    /// How fast the entity gets thirsty.
    pub per_second: f32,

    /// Current value of the thirst.
    pub value: f32,
}
