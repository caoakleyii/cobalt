use bevy::prelude::*;

use crate::{
    enums::GameState,
    networking::{is_client, is_server},
};

use self::{
    events::EquippedUse,
    systems::{equipment_use_system, tick_equipment_system},
};

pub mod components;
pub mod events;
mod systems;

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (equipment_use_system, tick_equipment_system)
                .run_if(is_server())
                .run_if(in_state(GameState::Gameloop)),
        );

        app.add_systems(
            Update,
            (tick_equipment_system)
                .run_if(is_client())
                .run_if(in_state(GameState::Gameloop)),
        );

        app.add_event::<EquippedUse>();
    }
}
