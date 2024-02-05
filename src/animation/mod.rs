use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::networking::is_client;

use self::{
    events::PlayAnimationEvent,
    systems::{animate_sprites, play_animation},
};

pub mod components;
pub mod events;
mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_sprites, play_animation).run_if(is_client()),
        );

        app.add_event::<PlayAnimationEvent>();
    }
}
