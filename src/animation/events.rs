use bevy::ecs::{entity::Entity, event::Event};

use super::components::AnimationName;

#[derive(Debug, Event)]
pub struct PlayAnimationEvent {
    pub entity: Entity,
    pub animation: AnimationName,
    pub reset: bool,
}

impl PlayAnimationEvent {
    pub fn new(entity: Entity, animation: &str) -> Self {
        Self {
            entity,
            animation: AnimationName(animation.to_string()),
            reset: false,
        }
    }
}
