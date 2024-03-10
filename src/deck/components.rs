use bevy::{
    ecs::{bundle::Bundle, component::Component},
    prelude::{Deref, DerefMut},
};

use super::card::components::Card;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Deck(pub Vec<Card>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Library(pub Vec<Card>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Hand(pub Vec<Card>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Graveyard(pub Vec<Card>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct InPlay(pub Vec<Card>);

#[derive(Bundle, Debug, Default)]
pub struct DeckBundle {
    pub deck: Deck,
    pub library: Library,
    pub hand: Hand,
    pub graveyard: Graveyard,
    pub in_play: InPlay,
}

impl DeckBundle {
    pub fn new(cards: Vec<Card>) -> Self {
        Self {
            deck: Deck(cards.clone()),
            library: Library(cards),
            ..Default::default()
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Shuffled;
