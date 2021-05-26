use serde::{Deserialize, Serialize};

use crate::pokemon::Level;
use crate::moves::MoveId;

pub mod training;
pub mod breeding;


#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Gender {
	None,
	Male,
	Female,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokedexData {
	pub species: String,
	pub height: u8,
	pub weight: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LearnableMove {
	#[serde(rename = "move")]
	pub move_id: MoveId,
	pub level: Level,
}