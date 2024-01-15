use bevy::{
    ecs::system::Res,
    prelude::{Children, Quat, Query, Transform, Without},
    sprite::TextureAtlasSprite,
    time::Time,
};

use crate::{
    input::components::Aim,
    math::{angle_between, vec2_from_vec3},
};

use super::components::Velocity;

pub fn apply_velocity(dt: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in &mut query {
        transform.translation.x += vel.vector.x * dt.delta_seconds();
        transform.translation.y += vel.vector.y * dt.delta_seconds();
    }
}

pub fn apply_direction(
    mut query: Query<(&Children, &mut TextureAtlasSprite, &Transform, &Aim)>,
    mut child_transform_query: Query<(&mut TextureAtlasSprite, &mut Transform), Without<Aim>>,
) {
    for (children, mut sprite, transform, aim) in &mut query {
        let from = vec2_from_vec3(&transform.translation);
        let angle = angle_between(&from, &*aim);

        sprite.flip_x = angle > std::f32::consts::PI / 2.0 || angle < -std::f32::consts::PI / 2.0;

        for child in children.iter() {
            if let Ok((mut child_sprite, mut transform)) = child_transform_query.get_mut(*child) {
                transform.translation.x = if sprite.flip_x { -5.0 } else { 5.0 };
                // rotate the sprite to the angle
                transform.rotation = Quat::from_rotation_z(angle);
                child_sprite.flip_y = sprite.flip_x;
            }
        }
    }
}
