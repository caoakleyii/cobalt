use bevy::prelude::*;

use crate::{enums::GameState, networking::is_server};

use self::{
    card::CardPlugin,
    events::{DrawCardEvent, ShuffleEvent},
    keyword::KeywordPlugin,
    systems::{player_spawned_spawn_deck, shuffle_deck},
};

pub mod card;
pub mod components;
pub mod enums;
pub mod events;
pub mod keyword;
mod systems;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shuffle_deck).run_if(in_state(GameState::Gameloop)));
        app.add_systems(Update, (player_spawned_spawn_deck).run_if(is_server()));
        app.add_plugins((CardPlugin, KeywordPlugin));
        app.add_event::<DrawCardEvent>();
        app.add_event::<ShuffleEvent>();
    }
}
