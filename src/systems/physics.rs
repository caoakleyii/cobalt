use bevy::{
    ecs::system::Res,
    prelude::{Query, Transform},
    time::Time,
};

use crate::components::Velocity;

pub fn apply_velocity(dt: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in &mut query {
        transform.translation.x += vel.vector.x * dt.delta_seconds();
        transform.translation.y += vel.vector.y * dt.delta_seconds();
    }
}
