use crate::{engine::{gui::MessageBox, EngineContext}, game::battle_glue::{BattleId, BattleTrainerEntry}};

use battlelib::data::BattleType;
use pokedex::context::PokedexClientContext;

use crate::battle::manager::transitions::{
    closers::{Closers, TrainerBattleCloser, WildBattleCloser},
    BattleCloser,
};

use firecore_battle_gui::transition::TransitionState;

#[derive(Default)]
pub struct BattleCloserManager {
    pub state: TransitionState,
    current: Closers,

    wild: WildBattleCloser,
    trainer: TrainerBattleCloser,
}

impl BattleCloserManager {
    pub fn begin<'d>(
        &mut self,
        ctx: &PokedexClientContext<'d>,
        battle_type: BattleType,
        player: &BattleId,
        player_name: &str,
        winner: Option<&BattleId>,
        trainer_entry: Option<&BattleTrainerEntry>,
        text: &mut MessageBox,
    ) {
        self.state = TransitionState::Run;
        match battle_type {
            BattleType::Wild => self.current = Closers::Wild,
            _ => self.current = Closers::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(ctx, player, player_name, winner, trainer_entry, text);
    }

    pub fn end(&mut self) {
        self.state = TransitionState::Begin;
    }

    pub fn update(&mut self, ctx: &mut EngineContext, delta: f32, text: &mut MessageBox) {
        let current = self.get_mut();
        current.update(ctx, delta, text);
        if current.finished() {
            self.state = TransitionState::End;
        }
    }

    pub fn draw(&self, ctx: &mut EngineContext) {
        self.get().draw(ctx);
    }

    pub fn draw_battle(&self, ctx: &mut EngineContext) {
        self.get().draw_battle(ctx);
    }

    pub fn world_active(&self) -> bool {
        matches!(self.state, TransitionState::Run) && self.get().world_active()
    }

    fn get(&self) -> &dyn BattleCloser {
        match self.current {
            Closers::Wild => &self.wild,
            Closers::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleCloser {
        match self.current {
            Closers::Wild => &mut self.wild,
            Closers::Trainer => &mut self.trainer,
        }
    }
}
