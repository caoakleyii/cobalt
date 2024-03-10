use bevy::prelude::{Bundle, Component, Transform};
use bevy_2d_collisions::components::CollisionBundle;

use crate::animation::components::AnimatedBundle;

/**
 * Object2D
 *
 * Component to define and query  2D Objects
 */
#[derive(Clone, Component, Default)]
pub struct Object2D;

/**
 * Size
 *
 * Width and Height
 */
#[derive(Clone, Component, Default)]
pub struct Size {
    pub width: f32,

    pub height: f32,
}

/**
 * Object 2D Bundle
 *
 * Base Bundle for any object that will exist in the game world without
 * animations/sprites
 */
#[derive(Clone, Bundle, Default)]
pub struct Object2DBundle {
    pub transform: Transform,

    pub size: Size,

    pub object_2d: Object2D,
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

    pub collision_bundle: CollisionBundle,
}

/**
 * Animated Static Body
 *
 * An animated body that is intended to be stationary
 */
#[derive(Bundle, Default)]
pub struct AnimatedStaticBodyBundle {
    pub animated_2d_object: AnimatedBundle,

    pub collision_bundle: CollisionBundle,
}
