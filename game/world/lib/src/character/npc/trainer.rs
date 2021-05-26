use serde::{Deserialize, Serialize};
use deps::{
    hash::HashSet,
    str::TinyStr8,
};
use firecore_pokedex::pokemon::party::PersistentParty;

use crate::default_true;
use super::NPCId;

type MessageSet = Vec<Vec<String>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trainer {

    #[serde(default = "default_true")]
    pub battle_on_interact: bool,
    pub tracking: Option<u8>,
    pub encounter_message: MessageSet,

    #[serde(default = "default_battle_transition")]
    pub battle_transition: TinyStr8,

    pub party: PersistentParty,

    #[serde(default)]
    pub victory_message: MessageSet,
    #[serde(default)]
    pub disable: HashSet<NPCId>,
    pub worth: u16,

}

fn default_battle_transition() -> TinyStr8 {
    "default".parse().unwrap()
}