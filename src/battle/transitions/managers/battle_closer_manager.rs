use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::util::context::GameContext;
use crate::util::text_renderer::TextRenderer;
use crate::entity::Entity;
use crate::entity::Ticking;
use crate::battle::transitions::battle_transition_traits::BattleCloser;
use crate::battle::transitions::battle_transition_traits::BattleTransitionManager;
use crate::battle::transitions::closers::basic_battle_closer::BasicBattleCloser;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

pub struct BattleCloserManager {
    
    alive: bool,

    pub closers: Vec<Box<dyn BattleCloser>>,
    pub current_closer_id: usize,

}

impl BattleCloserManager { // return player data

    pub fn new() -> Self {

        Self {

            alive: false,

            closers: Vec::new(),

            current_closer_id: 0,

        }

    }

    pub fn load_closers(&mut self) {
        self.closers.push(Box::new(BasicBattleCloser::new()));
    }

    pub fn world_active(&self) -> bool {
        self.closers[self.current_closer_id].world_active()
    }

    fn reset(&mut self) {
        self.closers[self.current_closer_id].reset();
    }

}

impl Ticking for BattleCloserManager {

    fn update(&mut self, context: &mut GameContext) {
        if self.is_alive() {
            self.closers[self.current_closer_id].update(context);
        }
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        self.closers[self.current_closer_id].render(ctx, g, tr);
    }

}

impl BattleTransitionManager for BattleCloserManager {

}

impl Loadable for BattleCloserManager {

    fn load(&mut self) {
        self.closers[self.current_closer_id].load();
    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.closers[self.current_closer_id].on_start(context);
    }

}

impl Completable for BattleCloserManager {

    fn is_finished(&self) -> bool {
        return self.closers[self.current_closer_id].is_finished();
    }

}

impl Entity for BattleCloserManager {

    fn spawn(&mut self) {
        self.alive = true;
        self.closers[self.current_closer_id].spawn();
        self.reset();
    }    

    fn despawn(&mut self) {
        self.alive = false;
        self.closers[self.current_closer_id].despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }

}