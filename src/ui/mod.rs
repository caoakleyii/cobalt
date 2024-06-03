use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{enums::GameState, player::systems::spawn_player};

use self::systems::{
    card_drawn, draw_hand_prompt, flip_card, health_bar_update, spawn_health_bar, spawn_hud,
};

pub mod components;
pub mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_hud.before(spawn_player), // wtf, no idea why this is working; should be opposite
                health_bar_update.after(spawn_player),
                spawn_health_bar.after(spawn_player),
                card_drawn.after(spawn_player),
                draw_hand_prompt.after(spawn_player),
                flip_card.after(spawn_player),
            )
                .run_if(in_state(GameState::Gameloop)),
        );
    }
}
