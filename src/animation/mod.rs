use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{enums::GameState, networking::is_client};

use self::{
    events::{PlayAnimationEvent, SpawnSpriteEvent},
    systems::{animate_sprites, play_animation, spawn_animation},
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

        app.add_systems(
            Update,
            (spawn_animation)
                .run_if(is_client())
                .run_if(in_state(GameState::Gameloop)),
        );

        app.add_event::<PlayAnimationEvent>();
        app.add_event::<SpawnSpriteEvent>();
    }
}
