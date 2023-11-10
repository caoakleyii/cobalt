use bevy::{
    prelude::{Bundle, Component, Handle, Transform},
    sprite::{SpriteSheetBundle, TextureAtlas},
};

use crate::resources::ClientId;

use super::{
    Animated2DObjectBundle, AnimatedKineticBodyBundle, Animator, KineticBodyBundle,
    NetworkedEntityBundle, Object2DBundle,
};

/**
 * Player
 *
 * Component stating an entity is a player
 */
#[derive(Component, Debug, Default)]
pub struct Player {
    pub id: ClientId,
}

/**
 * Controllable
 *
 * Component stating that the player entity is controllable by current user
 */
#[derive(Component, Default)]
pub struct Controllable;

/**
 * Player Bundle
 *
 * Contains a player component, kinetic body bundle
 * and network entity
 */
#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,

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
