use bevy::ecs::component::Component;
use bevy_enum_filter::EnumFilter;
use serde::{Deserialize, Serialize};

use crate::asset::resources::CollisionGroupConfig;

use super::components::{Cast, Damage, Discard, Draw, Projectile, Resource, Shuffle, Throw, AOE};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Keywords {
    AOE {
        component: AOE,
        collision: CollisionGroupConfig,
    },
    Damage {
        component: Damage,
    },
    Discard {
        component: Discard,
    },
    Draw {
        component: Draw,
    },
    Cast {
        component: Cast,
    },
    Resource {
        component: Resource,
    },
    Projectile {
        component: Projectile,
        collision: CollisionGroupConfig,
    },
    Shuffle {
        component: Shuffle,
    },
    Throw {
        component: Throw,
    },
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, Component, EnumFilter,
)]
pub enum ProjectileType {
    #[default]
    Bullet,
    SmgBullet,
    ShotgunBullet,
}
