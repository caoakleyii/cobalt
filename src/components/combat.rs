use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
};

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Damage(pub f32);

impl Default for Damage {
    fn default() -> Self {
        Self(10.0)
    }
}
