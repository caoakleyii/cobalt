use bevy::prelude::*;

use crate::{
    enums::GameState,
    networking::{is_client, is_server},
};

use self::{
    events::{DamageEntityEvent, SpawnProjectileEvent},
    systems::{damage_collision, on_damage_entity, spawn_projectile},
};

pub mod components;
pub mod events;
mod systems;

pub struct KeywordPlugin;

impl Plugin for KeywordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (damage_collision)
                .run_if(is_server())
                .run_if(in_state(GameState::Gameloop)),
        );

        app.add_systems(
            Update,
            (on_damage_entity, spawn_projectile)
                .run_if(is_client())
                .run_if(in_state(GameState::Gameloop)),
        );

        app.add_event::<SpawnProjectileEvent>();
        app.add_event::<DamageEntityEvent>();
    }
}
