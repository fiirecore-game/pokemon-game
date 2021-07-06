use crate::{
    engine::{
        graphics::{draw_o_bottom, TextureManager},
        gui::TextDisplay,
        tetra::{graphics::Texture, Context},
        text::MessagePage,
        util::{Completable, Entity, Reset, WIDTH},
    },
    game::{battle_glue::BattleTrainerEntry, storage::data},
    pokedex::{
        texture::TrainerTextures,
        trainer::{TrainerData, TrainerId},
    },
};

use crate::battle::manager::transitions::BattleCloser;

use super::wild::WildBattleCloser;

pub struct TrainerBattleCloser {
    wild: WildBattleCloser,
    trainer: Option<Texture>,
    offset: f32,
}

impl TrainerBattleCloser {
    const XPOS: f32 = 172.0; // 144 = pokemon
}

impl Default for TrainerBattleCloser {
    fn default() -> Self {
        Self {
            wild: WildBattleCloser::default(),
            trainer: None,
            offset: WIDTH,
        }
    }
}

impl BattleCloser for TrainerBattleCloser {
    fn spawn(
        &mut self,
        winner: Option<&TrainerId>,
        trainer_data: Option<&TrainerData>,
        trainer_entry: Option<&BattleTrainerEntry>,
        text: &mut TextDisplay,
    ) {
        match winner {
            Some(winner) => match winner == &data().id {
                true => {
                    if let (Some(trainer_data), Some(trainer)) = (trainer_data, trainer_entry) {
                        self.trainer = Some(TrainerTextures::get(&trainer_data.npc_type).clone());

                        text.reset();
                        text.clear();

                        text.push(MessagePage::new(
                            vec![
                                String::from("Player defeated"),
                                format!("{} {}!", trainer_data.prefix, trainer_data.name),
                            ],
                            None,
                        ));

                        for message in trainer.victory_message.iter() {
                            text.push(MessagePage::new(message.clone(), None));
                        }

                        text.push(MessagePage::new(
                            vec![
                                format!("{} got ${}", data().name, trainer.worth),
                                String::from("for winning!"),
                            ],
                            None,
                        ));

                        text.spawn();
                    }
                }
                false => {
                    text.despawn();
                }
            },
            None => {
                text.despawn();
            }
        }
    }

    fn update(&mut self, ctx: &mut Context, delta: f32, text: &mut TextDisplay) {
        if text.alive() {
            text.update(ctx, delta);
            if text.current() == 1 && self.offset > Self::XPOS {
                self.offset -= 300.0 * delta;
                if self.offset < Self::XPOS {
                    self.offset = Self::XPOS;
                }
            }
            if text.finished() {
                text.despawn();
            }
        } else {
            self.wild.update(ctx, delta, text);
        }
    }

    fn world_active(&self) -> bool {
        self.wild.world_active()
    }

    fn draw(&self, ctx: &mut Context) {
        self.wild.draw(ctx);
    }

    fn draw_battle(&self, ctx: &mut Context) {
        draw_o_bottom(ctx, self.trainer.as_ref(), self.offset, 74.0);
    }
}

impl Completable for TrainerBattleCloser {
    fn finished(&self) -> bool {
        self.wild.finished()
    }
}

impl Reset for TrainerBattleCloser {
    fn reset(&mut self) {
        self.trainer = None;
        self.wild.reset();
        self.offset = WIDTH;
    }
}
