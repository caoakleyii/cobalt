use std::fmt::Display;

use bevy::ecs::component::Component;
use bevy_enum_filter::EnumFilter;
use serde::{Deserialize, Serialize};

#[derive(
    Component, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, EnumFilter,
)]
pub enum Cards {
    Fireball,
    CrudeAmber,
    Amber,
}

impl Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cards::Fireball => write!(f, "Fireball"),
            Cards::CrudeAmber => write!(f, "Crude Amber"),
            Cards::Amber => write!(f, "Amber"),
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Component, EnumFilter,
)]
pub enum CardTypes {
    Spell,
    Mana,
}

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Component, EnumFilter,
)]
pub enum SubTypes {
    Sorcery,
    Disruption,

    // MANA
    Red,
    Blue,
    Yellow,
}

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Component, EnumFilter, Default,
)]
pub enum ResourceTypes {
    #[default]
    Red,
    Blue,
    Yellow,
}
