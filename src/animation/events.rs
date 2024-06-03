use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec3,
};

use crate::asset::enums::Sprites;

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

#[derive(Debug, Event)]
pub struct SpawnSpriteEvent {
    pub sprite: Sprites,
    pub entity: Option<Entity>,
    pub parent: Option<Entity>,
    pub animation: Option<AnimationName>,
    pub translation: Option<Vec3>,
}

impl Default for SpawnSpriteEvent {
    fn default() -> Self {
        Self {
            sprite: Sprites::Skeleton,
            entity: None,
            parent: None,
            animation: None,
            translation: None,
        }
    }
}
