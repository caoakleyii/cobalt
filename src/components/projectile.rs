use bevy::{
    math::Vec2,
    prelude::{Bundle, Component, Handle, Transform},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use bevy_2d_collisions::components::{CollisionBox, CollisionBundle, CollisionGroup};

use super::{
    Animated2DObjectBundle, AnimatedKineticBodyBundle, Animator, KineticBodyBundle, Object2DBundle,
    Velocity,
};

#[derive(Component, Debug, Default)]
pub struct Projectile;

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    pub projectile: Projectile,

    pub kinetic_body: AnimatedKineticBodyBundle,
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
            projectile: Projectile,
            kinetic_body: AnimatedKineticBodyBundle {
                velocity,
                animated_2d_object: Animated2DObjectBundle {
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
        }
    }
}

#[derive(Bundle, Default)]
pub struct ServerProjectileBundle {
    pub projectile: Projectile,

    pub kinetic_body: KineticBodyBundle,
}

impl ServerProjectileBundle {
    pub fn new(
        transform: Transform,
        velocity: Velocity,
        size: Vec2,
        collision_group: CollisionGroup,
    ) -> Self {
        Self {
            projectile: Projectile,
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
        }
    }
}
