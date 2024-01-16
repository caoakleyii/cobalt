use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::enums::GameState;

use self::systems::{apply_direction, apply_velocity};

pub mod components;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_velocity, apply_direction).run_if(in_state(GameState::Gameloop)),
        );
    }
}
