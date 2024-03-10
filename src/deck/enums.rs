use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Decks {
    #[default]
    Default,
}
