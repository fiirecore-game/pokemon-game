use firecore_util::Entity;
use crate::battle::battle::Battle;
use crate::gui::GuiComponent;
use crate::gui::dynamic_text::DynamicText;
use firecore_util::Reset;
use super::battle_background::BattleBackground;
use super::panels::player_panel::PlayerPanel;
use super::player_bounce::PlayerBounce;
use super::pokemon_gui::OpponentPokemonGui;
use super::pokemon_gui::PlayerPokemonGui;
use super::pokemon_gui::PokemonGui;

pub struct BattleGui {

	alive: bool,

	pub battle_background: BattleBackground,

	pub player_pokemon_gui: PlayerPokemonGui,
	pub opponent_pokemon_gui: OpponentPokemonGui,
	pub player_panel: PlayerPanel,

	pub battle_text: DynamicText,

	pub player_bounce: PlayerBounce,

}

impl BattleGui {

	pub fn new() -> Self {

		Self {

			alive: false,

			battle_background: BattleBackground::new(),

			player_panel: PlayerPanel::new(0.0, 113.0),
			player_pokemon_gui: PlayerPokemonGui::new(127.0, 75.0),
			opponent_pokemon_gui: OpponentPokemonGui::new(14.0, 18.0),

			battle_text: DynamicText::new(11.0, 11.0, 0.0, 113.0),

			player_bounce: PlayerBounce::new(),

		}

	}

	pub fn on_battle_start(&mut self, battle: &Battle) {
		self.update_gui(battle);
	}

	pub fn update_gui(&mut self, battle: &Battle) {

		self.player_panel.update_text(battle.player());

		// update health
		
		self.opponent_pokemon_gui.update_gui(battle);
		self.player_pokemon_gui.update_gui(battle);
	}

	pub fn update(&mut self, delta: f32) {
		self.player_bounce.update(delta, &mut self.player_pokemon_gui);
		self.player_panel.update(delta);
		self.player_pokemon_gui.update(delta);
		self.opponent_pokemon_gui.update(delta);
		self.battle_text.update(delta);
	}

	pub fn input(&mut self, delta: f32, battle: &mut Battle) {
		self.player_panel.input(delta, battle);
	}

	pub fn render_background(&self, offset: f32) {
		self.battle_background.render(offset);
	}

	pub fn render_panel(&self) {
		self.player_panel.render();
		self.battle_text.render();
	}

	pub fn render(&self) {
		self.opponent_pokemon_gui.render();
		self.player_pokemon_gui.render();
	}

}

impl Reset for BattleGui {
    fn reset(&mut self) {
		self.player_pokemon_gui.reset();
		self.opponent_pokemon_gui.reset();
        self.player_bounce.reset();
    }
}

impl Entity for BattleGui {

    fn spawn(&mut self) {
		self.alive = true;
        self.player_panel.spawn();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.player_panel.despawn();
		self.player_pokemon_gui.despawn();
		self.opponent_pokemon_gui.despawn();
		self.battle_text.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}
