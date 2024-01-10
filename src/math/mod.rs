use bevy::math::{Vec2, Vec3};

pub fn angle_between(from: &Vec2, to: &Vec2) -> f32 {
    let y = to.y - from.y;
    let x = to.x - from.x;
    y.atan2(x)
}

pub fn vec2_from_vec3(from: &Vec3) -> Vec2 {
    Vec2::new(from.x, from.y)
}
