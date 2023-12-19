use bevy::{
    prelude::{Bundle, Component, Deref, Handle, Transform, Vec2},
    sprite::{SpriteSheetBundle, TextureAtlas},
};
use bevy_renet::renet::ClientId;

use super::{
    Animated2DObjectBundle, AnimatedKineticBodyBundle, Animator, Health, KineticBodyBundle,
    NetworkedEntityBundle, Object2DBundle,
};

/**
 * Player
 *
 * Component stating an entity is a player
 */
#[derive(Component, Debug)]
pub struct Player {
    pub id: ClientId,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: ClientId::from_raw(0),
        }
    }
}

/**
 * Controllable
 *
 * Component stating that the player entity is controllable by current user
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

/**
 * Aim
 *
 * The world position of the player's aim
 */
#[derive(Component, Deref, Default)]
pub struct Aim(pub Vec2);

/**
 * Player Bundle
 *
 * Contains a player component, kinetic body bundle
 * and network entity
 */
#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,

    pub health: Health,

    pub aim: Aim,

    pub kinetic_body: AnimatedKineticBodyBundle,

    pub network_entity: NetworkedEntityBundle,
}

impl PlayerBundle {
    pub fn new(
        id: ClientId,
        animator: Animator,
        texture_atlas: Handle<TextureAtlas>,
        transform: Transform,
    ) -> Self {
        Self {
            player: Player { id },
            kinetic_body: AnimatedKineticBodyBundle {
                animated_2d_object: Animated2DObjectBundle {
                    animator,
                    sprite_sheet_bundle: SpriteSheetBundle {
                        texture_atlas,
                        transform,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

/**
 * Server Player Bundle
 *
 */
#[derive(Bundle, Default)]
pub struct ServerPlayerBundle {
    pub player: Player,

    pub kinetic_body: KineticBodyBundle,

    pub network_entity: NetworkedEntityBundle,
}

impl ServerPlayerBundle {
    pub fn new(id: ClientId, transform: Transform) -> Self {
        Self {
            player: Player { id },
            kinetic_body: KineticBodyBundle {
                object_2d_bundle: Object2DBundle {
                    transform,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
