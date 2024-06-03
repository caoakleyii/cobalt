use bevy::{
    math::Vec2,
    prelude::{Bundle, Component, Handle, Transform},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use bevy_2d_collisions::components::{CollisionBox, CollisionBundle, CollisionGroup};
use serde::{Deserialize, Serialize};

use crate::body::components::Object2DBundle;
use crate::physics::components::{AnimatedKineticBodyBundle, KineticBodyBundle, Velocity};
use crate::{
    animation::components::{AnimatedBundle, Animator},
    deck::card::enums::ResourceTypes,
};

use super::enums::ProjectileType;

#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, Copy)]
pub struct AOE {
    pub radius: f32,
}

/// DAMAGE Keyword Component
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Damage {
    pub amount: f32,
}

impl Default for Damage {
    fn default() -> Self {
        Self { amount: 10.0 }
    }
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Discard {
    pub amount: u32,
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Draw {
    pub amount: u32,
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Cast {
    pub cast: u32,
    pub max_cast: u32,
    pub cast_time: f32,
    pub recovery_time: f32,
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Gun {
    pub magazine: u32,
    pub max_magazine: u32,
    pub fire_rate: f32,
    pub reload_time: f32,
    pub spray: f32,
    pub projectiles_per_shot: u32,
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Resource {
    pub pool: u32,
    pub max_pool: u32,
    pub resource_type: ResourceTypes,
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Projectile {
    pub projectile_type: ProjectileType,

    pub speed: f32,

    pub range: f32,
}

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    pub projectile: Projectile,

    pub kinetic_body: AnimatedKineticBodyBundle,

    pub damage: Damage,
}

impl ProjectileBundle {
    pub fn new(
        animator: Animator,
        texture_atlas: Handle<TextureAtlas>,
        transform: Transform,
        velocity: Velocity,
        size: Vec2,
        collision_group: CollisionGroup,
    ) -> Self {
        let mut animator = animator;
        let sprite = TextureAtlasSprite {
            index: animator.current_animation().first,
            ..Default::default()
        };

        Self {
            projectile: Projectile::default(),
            kinetic_body: AnimatedKineticBodyBundle {
                velocity,
                animated_2d_object: AnimatedBundle {
                    animator,
                    sprite_sheet_bundle: SpriteSheetBundle {
                        sprite,
                        texture_atlas,
                        transform,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                collision_bundle: CollisionBundle {
                    collision_box: CollisionBox {
                        size,
                        ..Default::default()
                    },
                    collision_group,
                    ..Default::default()
                },
            },
            ..Default::default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct ServerProjectileBundle {
    pub projectile: Projectile,

    pub kinetic_body: KineticBodyBundle,

    pub damage: Damage,
}

impl ServerProjectileBundle {
    pub fn new(
        transform: Transform,
        velocity: Velocity,
        size: Vec2,
        collision_group: CollisionGroup,
    ) -> Self {
        Self {
            projectile: Projectile::default(),
            kinetic_body: KineticBodyBundle {
                object_2d_bundle: Object2DBundle {
                    transform,
                    ..Default::default()
                },
                velocity,
                collision_bundle: CollisionBundle {
                    collision_box: CollisionBox {
                        size,
                        ..Default::default()
                    },
                    collision_group,
                    ..Default::default()
                },
            },
            ..Default::default()
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Shuffle;

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Throw {
    pub range: f32,
    pub speed: f32,
}
