use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::enums::GameState;

use self::systems::{health_bar_update, spawn_health_bar, spawn_hud};

mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_hud).run_if(in_state(GameState::Gameloop)));
        app.add_systems(
            Update,
            (health_bar_update, spawn_health_bar).run_if(in_state(GameState::Gameloop)),
        );
    }
}
