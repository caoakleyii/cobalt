use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
};

/**
 * Speed
 *
 * Container for speed floats
 */
#[derive(Component, Deref, DerefMut, Debug)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(100.0)
    }
}

/**
 * Health
 *
 * Component to define the health of an object
 */
#[derive(Component, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}
