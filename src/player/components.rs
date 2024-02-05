use bevy::{
    prelude::{Bundle, Component, Deref, Handle, Transform, Vec2},
    sprite::{SpriteSheetBundle, TextureAtlas},
};
use bevy_2d_collisions::components::{CollisionBox, CollisionBundle, CollisionGroup};
use bevy_renet::renet::ClientId;

use crate::{
    animation::components::{Animated2DObjectBundle, Animator},
    body::components::Object2DBundle,
    enums::CollisionGroups,
    input::components::Aim,
    networking::components::NetworkedEntityBundle,
    physics::components::{AnimatedKineticBodyBundle, KineticBodyBundle},
    stats::components::Health,
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
* Team
*
* The team the player is on
*/
#[derive(Component, Deref, Default)]
pub struct Team(pub CollisionGroups);

/**
 * Death State
*/
#[derive(Component, Debug, Default)]
pub struct Death;

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

    pub team: Team,

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
        size: Vec2,
        collision_group: CollisionGroup,
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
                collision_bundle: CollisionBundle {
                    collision_box: CollisionBox {
                        size,
                        ..Default::default()
                    },
                    collision_group,
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

    pub health: Health,

    pub team: Team,

    pub kinetic_body: KineticBodyBundle,

    pub network_entity: NetworkedEntityBundle,
}

impl ServerPlayerBundle {
    pub fn new(
        id: ClientId,
        transform: Transform,
        size: Vec2,
        collision_group: CollisionGroup,
        team: Team,
    ) -> Self {
        Self {
            player: Player { id },
            kinetic_body: KineticBodyBundle {
                object_2d_bundle: Object2DBundle {
                    transform,
                    ..Default::default()
                },
                collision_bundle: CollisionBundle {
                    collision_box: CollisionBox {
                        size,
                        ..Default::default()
                    },
                    collision_group,
                    ..Default::default()
                },
                ..Default::default()
            },
            team,
            ..Default::default()
        }
    }
}
