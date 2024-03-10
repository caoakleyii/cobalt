use std::fmt::Display;

use bevy::ecs::component::Component;
use bevy_enum_filter::EnumFilter;
use serde::{Deserialize, Serialize};

#[derive(
    Component, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, EnumFilter,
)]
pub enum Cards {
    AK47,
    MP5,
    M1216,
    EMP,
    FragGrenade,
    SupplyDrop,
    Metal,
}

impl Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cards::AK47 => write!(f, "AK47"),
            Cards::MP5 => write!(f, "MP5"),
            Cards::M1216 => write!(f, "M1216"),
            Cards::EMP => write!(f, "EMP"),
            Cards::FragGrenade => write!(f, "Frag Grenade"),
            Cards::SupplyDrop => write!(f, "Supply Drop"),
            Cards::Metal => write!(f, "Metal"),
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Component, EnumFilter,
)]
pub enum CardTypes {
    Weapon,
    Utility,
    Material,
}

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Component, EnumFilter,
)]
pub enum SubTypes {
    Rifle,
    SMG,
    Shotgun,
    Disruption,
    Offensive,
    Tactical,
    Metal,
}
