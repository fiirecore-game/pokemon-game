use core::ops::Deref;
use pokedex::{
    engine::{utils::HashMap, EngineContext},
    item::Item,
    moves::Move,
    pokemon::Pokemon,
};

use pokedex::{
    engine::{utils::Completable, Context},
    PokedexClientData,
};

use battle::data::BattleType;

use crate::{
    context::BattleGuiData,
    ui::{
        text::BattleText,
        view::{ActivePokemonRenderer, GuiLocalPlayer, GuiRemotePlayer},
    },
};

use super::TransitionState;

mod basic;
mod trainer;

pub use basic::*;
pub use trainer::*;

pub enum Introductions {
    Basic,
    Trainer,
}

impl Default for Introductions {
    fn default() -> Self {
        Self::Basic
    }
}

pub(crate) trait BattleIntroduction<
    ID,
    P: Deref<Target = Pokemon>,
    M: Deref<Target = Move>,
    I: Deref<Target = Item>,
>: Completable
{
    fn spawn(
        &mut self,
        ctx: &PokedexClientData,
        local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &HashMap<ID, GuiRemotePlayer<ID, P>>,
        text: &mut BattleText,
    );

    fn update(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        delta: f32,
        player: &mut GuiLocalPlayer<ID, P, M, I>,
        opponent: &mut GuiRemotePlayer<ID, P>,
        text: &mut BattleText,
    );

    fn draw(
        &self,
        ctx: &mut Context,
        eng: &EngineContext,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    );
}

pub struct BattleIntroductionManager {
    current: Introductions,

    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,
}

impl BattleIntroductionManager {
    pub fn new(ctx: &BattleGuiData) -> Self {
        Self {
            current: Introductions::default(),

            basic: BasicBattleIntroduction::new(ctx),
            trainer: TrainerBattleIntroduction::new(ctx),
        }
    }

    // pub fn update_with_state<ID, P, M, I>(&mut self, dex: &PokedexClientData, text: &mut MessageBox, delta: f32, local: &GuiLocalPlayer<ID, P, M, I>, remotes: &HashMap<ID, GuiRemotePlayer<ID, P>>, state: &TransitionState) {
    //     match state {
    //         TransitionState::Begin => {
    //             self.begin(
    //                 dex,
    //                 state,
    //                 local,
    //                 self.remotes.values().next().unwrap(),
    //                 &mut self.gui.text,
    //             );
    //             TransitionResult::Rerun
    //         }
    //         TransitionState::Run => {
    //             self.update(
    //                 state,
    //                 ctx,
    //                 delta,
    //                 local,
    //                 self.remotes.values_mut().next().unwrap(),
    //                 &mut self.gui.text,
    //             );
    //             if self.gui.text.page() > 0
    //                 && !self.gui.trainer.ending()
    //                 && !matches!(local.data.type_, BattleType::Wild)
    //             {
    //                 self.gui.trainer.end();
    //             }
    //         }
    //         TransitionState::End => {
    //             self.end(&mut self.gui.text);
    //             TransitionResult::Next
    //         }
    //     }
    // }

    pub fn begin<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        ctx: &PokedexClientData,
        state: &mut TransitionState,
        local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &HashMap<ID, GuiRemotePlayer<ID, P>>,
        text: &mut BattleText,
    ) {
        *state = TransitionState::Run;
        match local.data.type_ {
            BattleType::Wild => self.current = Introductions::Basic,
            _ => self.current = Introductions::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(ctx, local, opponents, text);
    }

    pub fn update<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        state: &mut TransitionState,
        ctx: &mut Context,
        eng: &mut EngineContext,
        delta: f32,
        player: &mut GuiLocalPlayer<ID, P, M, I>,
        opponent: &mut GuiRemotePlayer<ID, P>,
        text: &mut BattleText,
    ) {
        let current = self.get_mut::<ID, P, M, I>();
        current.update(ctx, eng, delta, player, opponent, text);
        if current.finished() {
            *state = TransitionState::End;
        }
    }

    pub fn draw<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &self,
        ctx: &mut Context,
        eng: &EngineContext,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        self.get::<ID, P, M, I>().draw(ctx, eng, player, opponent);
    }

    fn get<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>(
        &self,
    ) -> &dyn BattleIntroduction<ID, P, M, I> {
        match self.current {
            Introductions::Basic => &self.basic,
            Introductions::Trainer => &self.trainer,
        }
    }

    fn get_mut<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>(
        &mut self,
    ) -> &mut dyn BattleIntroduction<ID, P, M, I> {
        match self.current {
            Introductions::Basic => &mut self.basic,
            Introductions::Trainer => &mut self.trainer,
        }
    }
}
