use bevy::prelude::*;

use self::{card::CardPlugin, keyword::KeywordPlugin};

pub mod card;
pub mod keyword;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CardPlugin, KeywordPlugin));
    }
}
