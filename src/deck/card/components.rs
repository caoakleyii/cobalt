use bevy::ecs::component::Component;

use serde::{Deserialize, Serialize};

use crate::deck::keyword::enums::Keywords;

use super::enums::{CardTypes, Cards, SubTypes};

#[derive(Serialize, Deserialize, Component, Debug, Clone)]
pub struct Card {
    pub name: Cards,
    pub resource_cost: u32,
    pub card_type: CardTypes,
    pub sub_type: Option<SubTypes>,
    pub keywords: Vec<Keywords>,
}

// TODO: Figure out how a player would cast a card
// a system to query the selected card
// itearte over the keywords and apply the effects/spawn entities
// remove the card from the hand, put it into play
// when the card is done, put it into the graveyard

// TODO:
// when the deck is empty, shuffle the graveyard into the deck
