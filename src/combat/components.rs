use bevy::{
    asset::Handle,
    ecs::{
        bundle::{self, Bundle},
        component::Component,
        entity::Entity,
    },
    math::Vec3,
    prelude::{Deref, DerefMut},
    render::texture::Image,
    time::Timer,
    transform::{self, components::Transform},
};
use bevy_health_bar::{ProgressBar, ProgressBarBundle};

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

#[derive(Component, Debug)]
pub struct HealthBar;

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct Casting(pub Entity);

#[derive(Component, Debug)]
pub struct CastBar;

#[derive(Bundle)]
pub struct CastingBundle {
    pub cast_time: CastTime,

    pub progress_bar: ProgressBarBundle,

    pub castable: Castable,

    pub cast_bar: CastBar,
}

impl CastingBundle {
    pub fn new(cast_time: f32, cast_bar_image: Handle<Image>, castable_entity: Entity) -> Self {
        let transform = Transform::from_xyz(-15.0, 14.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5));
        Self {
            cast_time: CastTime {
                timer: Timer::from_seconds(cast_time, bevy::time::TimerMode::Once),
            },
            progress_bar: ProgressBarBundle::new(cast_time, cast_bar_image)
                .with_transform(transform),
            castable: Castable {
                entity: castable_entity,
            },
            cast_bar: CastBar,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct CastTime {
    pub timer: Timer,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Castable {
    pub entity: Entity,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Casted {
    pub casted_by: Entity,
}
