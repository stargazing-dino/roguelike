use bevy::prelude::*;
use big_brain::{prelude::ScorerBuilder, scorers::Score, thinker::Actor};

use crate::components::target::Target;

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub struct AttackOpportunity;

#[derive(Debug, Clone, Component)]
pub struct AttackReadiness {
    pub value: f32,
    pub attack_range: f32,
}

pub fn attack_opportunity_scorer(
    targets: Query<&Transform>,
    readiness: Query<&AttackReadiness>,
    mut query: Query<(&Actor, &Target, &mut Score), With<AttackOpportunity>>,
) {
    for (Actor(actor), Target(target), mut score) in query.iter_mut() {
        if let Ok(target_transform) = targets.get(*target) {
            if let Ok(actor_readiness) = readiness.get(*actor) {
                let actor_transform = targets.get(*actor).unwrap();
                let distance = target_transform
                    .translation
                    .distance(actor_transform.translation);

                if distance <= actor_readiness.attack_range {
                    score.set(actor_readiness.value);
                } else {
                    score.set(0.0);
                }
            }
        }
    }
}
