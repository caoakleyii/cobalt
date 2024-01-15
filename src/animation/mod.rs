use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::networking::is_client;

use self::systems::{animate_sprites, sync_animation_state};

pub mod components;
mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_sprites, sync_animation_state).run_if(is_client()),
        );
    }
}
