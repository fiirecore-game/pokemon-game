use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::{engine::{game_context::GameContext, text::TextRenderer}, scene::scene::Scene};

pub struct MainMenuScene {
	scene_token: usize,
}

impl MainMenuScene {

	pub fn new() -> MainMenuScene {

		MainMenuScene {

			scene_token: 0,

		}

	}

}

impl Scene for MainMenuScene {

	// have normal main menu + video settings + controls + exit

	fn load(&mut self) {

	}
	
	fn update(&mut self, _context: &mut GameContext) {}
	
	fn render(&mut self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {}
	
	fn input(&mut self, _context: &mut GameContext) {}
	
	fn dispose(&mut self) {}
	
	fn name(&self) -> &str {
		&"Main Menu"
	}
	
	fn next_scene(&self) -> usize {
		self.scene_token
	}
	
}