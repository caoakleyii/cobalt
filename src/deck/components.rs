use bevy::{
    ecs::{bundle::Bundle, component::Component},
    prelude::{Deref, DerefMut},
};

use super::card::components::CardEntity;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Deck(pub Vec<CardEntity>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Library(pub Vec<CardEntity>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Hand(pub Vec<CardEntity>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Graveyard(pub Vec<CardEntity>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct InPlay(pub Vec<CardEntity>);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct HandSize(pub usize);

impl Default for HandSize {
    fn default() -> Self {
        Self(4)
    }
}

#[derive(Bundle, Debug, Default)]
pub struct DeckBundle {
    pub deck: Deck,
    pub library: Library,
    pub hand: Hand,
    pub graveyard: Graveyard,
    pub in_play: InPlay,
    pub hand_size: HandSize,
}

impl DeckBundle {
    pub fn new(cards: Vec<CardEntity>) -> Self {
        Self {
            deck: Deck(cards.clone()),
            library: Library(cards),
            ..Default::default()
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Shuffled;
