use bevy::prelude::*;

use crate::enums::GameState;
use crate::networking::{is_client, is_server};
use crate::player::systems::spawn_player;

use self::events::{DamageEntityEvent, SpawnProjectileEvent};
use self::systems::draw::draw_card_to_hand;
use self::systems::{
    damage::{damage_collision, on_damage_entity},
    projectile::spawn_projectile,
};

use super::events::CardDrawnEvent;

pub mod components;
pub mod enums;
pub mod events;
mod systems;

pub struct KeywordPlugin;

impl Plugin for KeywordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (damage_collision, draw_card_to_hand)
                .before(spawn_player)
                .run_if(is_server())
                .run_if(in_state(GameState::Gameloop)),
        );

        app.add_systems(
            Update,
            (on_damage_entity, spawn_projectile, draw_card_to_hand)
                .before(spawn_player)
                .run_if(is_client())
                .run_if(in_state(GameState::Gameloop)),
        );

        app.add_event::<SpawnProjectileEvent>();
        app.add_event::<DamageEntityEvent>();
        app.add_event::<CardDrawnEvent>();
    }
}
