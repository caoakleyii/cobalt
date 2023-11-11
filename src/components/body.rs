use bevy::prelude::{Bundle, Component, Deref, DerefMut, Vec2};

use super::{Animated2DObjectBundle, Object2DBundle};

/**
 * Speed
 *
 * Container for speed floats
 * ? Perhaps can be removed as a container since we have a velocity
 * ? component now.
 */
#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(100.0)
    }
}

/**
 * Velocity
 *
 * Component to define the velocity of an object
 */
#[derive(Component, Default)]
pub struct Velocity {
    pub base_speed: Speed,

    pub current_speed: Speed,

    pub rotation: f32,

    pub vector: Vec2,
}

// ! TODO: Define or use a bevy/library hitbox bundle

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
}

/**
 * Animated KineticBody
 *
 * An animated body that is intended to be moving
 */
#[derive(Bundle, Default)]
pub struct AnimatedKineticBodyBundle {
    pub velocity: Velocity,

    pub animated_2d_object: Animated2DObjectBundle,
}

/**
 * Static Body
 *
 * A body that is intended to be stationary without animation
 * cna be used by server without rendering
 */
#[derive(Bundle, Default)]
pub struct StaticBodyBundle {
    pub object_2d_bundle: Object2DBundle,
}

/**
 * Animated Static Body
 *
 * An animated body that is intended to be stationary
 */
#[derive(Bundle, Default)]
pub struct AnimatedStaticBodyBundle {
    pub animated_2d_object: Animated2DObjectBundle,
}
