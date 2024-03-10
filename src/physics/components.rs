use crate::animation::components::AnimatedBundle;
use crate::body::components::Object2DBundle;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy_2d_collisions::components::CollisionBundle;

use crate::stats::components::Speed;

/**
 * Velocity
 *
 * Component to define the velocity of an object
 */
#[derive(Component, Default, Debug)]
pub struct Velocity {
    pub base_speed: Speed,

    pub current_speed: Speed,

    pub rotation: f32,

    pub vector: Vec2,
}

impl From<[f32; 2]> for Velocity {
    fn from(vector: [f32; 2]) -> Self {
        let x = vector[0];
        let y = vector[1];
        let speed = f32::sqrt(x.powi(2) + y.powi(2));

        Self {
            vector: Vec2::from(vector),
            rotation: y.atan2(x),
            current_speed: Speed(speed),
            base_speed: Speed(speed),
        }
    }
}

/**
 * Kinetic Body
 *
 * A body that is intended to be moving without animation
 * can be used by server without rendering
 */
#[derive(Bundle, Default)]
pub struct KineticBodyBundle {
    pub velocity: Velocity,

    pub object_2d_bundle: Object2DBundle,

    pub collision_bundle: CollisionBundle,
}

/**
 * Animated KineticBody
 *
 * An animated body that is intended to be moving
 */
#[derive(Bundle, Default)]
pub struct AnimatedKineticBodyBundle {
    pub velocity: Velocity,

    pub animated_2d_object: AnimatedBundle,

    pub collision_bundle: CollisionBundle,
}
