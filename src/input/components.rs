use bevy::{ecs::component::Component, math::Vec2, prelude::Deref};

/**
 * Controllable
 *
 * Component stating that the entity is controllable by current user
 */
#[derive(Component, Default)]
pub struct Controllable;

/**
 * Player Camera
 *
 * Component stating that the player entity is the camera target
 */
#[derive(Component, Default)]
pub struct PlayerCamera;

#[derive(Component, Default)]
pub struct FollowCamera;

/**
 * Aim
 *
 * The world position of the player's aim
 */
#[derive(Component, Deref, Default)]
pub struct Aim(pub Vec2);
