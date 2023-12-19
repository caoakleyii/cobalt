use bevy::prelude::{Bundle, Component, Transform};

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
