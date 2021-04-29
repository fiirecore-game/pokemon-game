use game::text::MessagePage;
use game::{
    util::{
        Entity,
        Reset,
        Completable,
    },
    storage::{get, player::PlayerSaves},
    audio::{play_sound, Sound},
    macroquad::prelude::{Vec2, warn, Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect},
    text::{Message, process_messages},
    gui::text::DynamicText,
    graphics::{byte_texture, draw_bottom},
};

use crate::{
    Battle,
    gui::{
        BattleGui,
        pokemon::PokemonGui,
    },
    transitions::{
        BattleTransition,
        BattleTransitionGui,
        BattleIntroduction,
    }
};

pub struct BasicBattleIntroduction {

    alive: bool,
    finished: bool,

    pub text: DynamicText,
    
    player: Texture2D,
	counter: f32,

    finished_panel: bool,

}

impl BasicBattleIntroduction {

    pub fn new(panel: Vec2) -> Self {
        Self {
            alive: false,
            finished: false,

            text: DynamicText::empty(Vec2::new(11.0, 11.0), panel),

            player: byte_texture(include_bytes!("../../../assets/player.png")),
			counter: 0.0,
            
            finished_panel: false,
        }
    }

    pub fn common_setup(&mut self, battle: &Battle) {
        if let Some(message) = self.text.message.as_mut() {
            message.message_set.push(
                MessagePage::new(
                    vec![
                        format!("Go! {}!", battle.player.active().name())
                    ],
                    Some(0.5),
                )
            );
            if let Some(saves) = get::<PlayerSaves>() {
                process_messages(saves.get(), message);
            }
            
        }
    }

    pub fn render_player(&self, battle: &Battle, offset: f32) {
        if self.counter < 104.0 {
            draw_texture_ex(self.player, 41.0 + offset - self.counter, 49.0, WHITE, DrawTextureParams {
                source: Some(
                    Rect::new(
                        0.0, 
                        if self.counter >= 78.0 {
                            256.0
                        } else if self.counter >= 60.0 {
                            192.0
                        } else if self.counter >= 42.0 {
                            128.0
                        } else if self.counter > 0.0 {
                            64.0
                        } else {
                            0.0
                        }, 
                        64.0, 
                        64.0
                    )
                ),
                ..Default::default()
            });
        } else {
            draw_bottom(battle.player.active_texture(), 40.0 + offset, 113.0);
        }
    }

}

impl BattleTransitionGui for BasicBattleIntroduction {

    fn input(&mut self) {
        self.text.input();
    }

}

impl BattleIntroduction for BasicBattleIntroduction {

    fn setup(&mut self, battle: &Battle) {
        self.text.message = Some(
            Message::single(
                vec![
                    format!("Wild {} appeared!", battle.opponent.active().pokemon.data.name.to_ascii_uppercase())
                ],
                game::text::TextColor::White,
                None,
            )
        );
        self.common_setup(battle);
    }

    fn update_gui(&mut self, battle: &Battle, battle_gui: &mut BattleGui, delta: f32) {
        if self.text.can_continue() {
            if let Some(message) = self.text.message.as_ref() {
                if self.text.current_message() >= message.message_set.len() - 2 && !battle_gui.opponent.is_alive() {
                    battle_gui.opponent.spawn();
                    if let Err(err) = play_sound(Sound::variant("Cry", Some(battle.opponent.active().pokemon.data.id))) {
                        match err {
                            game::audio::error::PlayAudioError::Uninitialized => (),
                            _ => warn!("Could not play opponent pokemon cry with error {}", err),
                        }
                    }
                }
            }            
        }

        if self.counter >= 104.0 && !battle_gui.player.is_alive() {
            battle_gui.player.spawn();
            if let Err(err) = play_sound(Sound::variant("Cry", Some(battle.player.active().pokemon.data.id))) {
                match err {
                    game::audio::error::PlayAudioError::Uninitialized => (),
                    _ => warn!("Could not play player pokemon cry with error {}", err),
                }
            }
        }

        battle_gui.opponent.offset(delta);
        
        if battle_gui.player.offset(delta) {
            self.finished_panel = true;
        }
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        draw_bottom(battle.opponent.active_texture(), 144.0 - offset, 74.0);
        self.render_player(battle, offset);
    }

}

impl BattleTransition for BasicBattleIntroduction {

    fn on_start(&mut self) {}

    fn update(&mut self, delta: f32) {
        self.text.update(delta);
        if let Some(message) = self.text.message.as_ref() {
            if self.text.current_message() + 1 == message.message_set.len() {
                if self.counter < 104.0 {
                    self.counter += delta * 180.0;                
                } else if self.text.is_finished() {
                    self.text.despawn();
                    self.finished = true;
                }
            }
        }
        
	}

    fn render(&self) {
        self.text.render();
	}
    
}

impl Completable for BasicBattleIntroduction {

    fn is_finished(&self) -> bool {
        self.finished && self.counter >= 104.0 && self.finished_panel
    }

}

impl Reset for BasicBattleIntroduction {

    fn reset(&mut self) {
        self.counter = 0.0;
        self.text.reset();
        self.finished_panel = false;
    }

}

impl Entity for BasicBattleIntroduction {

    fn spawn(&mut self) {
        self.alive = true;
        self.finished = false;
        self.text.spawn();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.finished = false;
        self.text.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

}