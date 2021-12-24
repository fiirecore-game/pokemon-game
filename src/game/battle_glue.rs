use std::fmt::Debug;

use crate::pokedex::pokemon::{owned::SavedPokemon, party::Party};

use worldlib::{
    character::{
        npc::{trainer::BadgeId, NpcId, group::NpcGroupId},
        Worth,
    },
    map::TransitionId,
};

/***********************/

#[derive(Debug, Clone)]
pub struct BattleEntry {
    pub id: BattleId,
    pub party: Party<SavedPokemon>,
    pub trainer: Option<BattleTrainerEntry>,
    pub active: usize,
}

#[derive(Debug, Clone)]
pub struct BattleTrainerEntry {
    pub name: String,
    pub badge: Option<BadgeId>,
    pub sprite: NpcGroupId,
    pub transition: TransitionId,
    pub victory_message: Vec<Vec<String>>,
    pub worth: Worth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BattleId {
    Default,
    Player,
    Wild,
    Trainer(NpcId),
}

impl Default for BattleId {
    fn default() -> Self {
        Self::Default
    }
}

impl core::fmt::Display for BattleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}
