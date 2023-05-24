use bevy::prelude::Component;

#[derive(Debug, Clone, Component)]
pub struct AttackReadiness {
    pub value: f32,
    pub attack_range: f32,
}
