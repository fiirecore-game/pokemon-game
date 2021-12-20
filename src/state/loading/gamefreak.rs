use game::{
	play_music_named,
	input::{keyboard::keys, Control},
	text::TextColor,
	graphics::{byte_texture, draw_text_left, fade_in},
	macroquad::prelude::{Color, color_u8, Texture2D, draw_texture_ex, draw_rectangle, WHITE, DrawTextureParams},
};

use super::LoadingState;

const BACKGROUND_COLOR: Color = color_u8!(24, 40, 72, 255);

pub struct GamefreakLoadingScene {
	
    state: LoadingState,
	accumulator: f32,
	rect_size: f32,
	star: Texture2D,
	logo: Texture2D,
	text: Texture2D,

}

impl super::LoadingScene for GamefreakLoadingScene {

	fn new() -> Self {
		Self {
			state: LoadingState::Continue,
			rect_size: 0.0,
			accumulator: 0.0,
			star: Texture::new(include_bytes!("../../../build/assets/scenes/loading/star.png")),
			logo: Texture::new(include_bytes!("../../../build/assets/scenes/loading/logo.png")),
			text: Texture::new(include_bytes!("../../../build/assets/scenes/loading/text.png")),
		}
	}

	fn on_start(&mut self) {
		self.state = LoadingState::Continue;
		self.accumulator = 0.0;
		self.rect_size = 0.0;
		play_music_named("IntroGamefreak");
	}
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.rect_size < 96.0 {
			self.rect_size += delta * 480.0;
			if self.rect_size > 96.0 {
				self.rect_size = 96.0;
			}
		}
		if self.accumulator > 8.5 {
			self.state = LoadingState::End;
		}
	}
	
	fn render(&self) {
		draw_rectangle(0.0, (crate::HEIGHT - self.rect_size) / 2.0, 240.0, self.rect_size, BACKGROUND_COLOR);

		if self.accumulator < 2.0 {
			draw_texture_ex(self.star, crate::WIDTH - self.accumulator * 240.0, 64.0 + self.accumulator * 5.0, WHITE, DrawTextureParams {
			    rotation: self.accumulator * 2.0,
				..Default::default()
			})
		}

		fade_in(self.logo, 108.0, 45.0, self.accumulator - 6.0, 1.0); //108x, 12y
		fade_in(self.text, 51.0, 74.0, self.accumulator - 4.0, 1.0); //51x, 41y


		draw_text_left(1, &format!("A Button is{:?}Key", keys(&Control::A).expect("Could not get keys for A button")), &Message::White, 5.0, 5.0);
		draw_text_left(1, &format!("B Button is{:?}Key", keys(&Control::B).expect("Could not get keys for B button")), &Message::White, 125.0, 5.0);
		draw_text_left(1, "D-Pad is Arrow Keys", &Message::White, 5.0, 15.0);

	}

    fn state(&self) -> LoadingState {
        self.state
    }
	
}