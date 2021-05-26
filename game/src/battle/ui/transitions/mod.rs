use crate::{
    util::Completable,
    gui::DynamicText,
    tetra::Context,
};

use crate::battle::Battle;

pub mod managers;

pub mod transitions;
pub mod openers;
pub mod introductions;
pub mod closers;


pub(crate) trait BattleTransition: Completable {

    fn update(&mut self, ctx: &mut Context, delta: f32);

    fn draw(&self, ctx: &mut Context);

    // fn render_below_player(&self);

}

pub(crate) trait BattleOpener: Completable  {

    fn spawn(&mut self, battle: &Battle);
    
    fn update(&mut self, delta: f32);

    fn draw_below_panel(&self, ctx: &mut Context, battle: &Battle);

    fn draw(&self, ctx: &mut Context);

    fn offset(&self) -> f32;

}

pub(crate) trait BattleIntroduction: Completable {

    fn spawn(&mut self, battle: &Battle, text: &mut DynamicText);

    fn update(&mut self, ctx: &mut Context, delta: f32, battle: &mut Battle, text: &mut DynamicText);

    fn draw(&self, ctx: &mut Context, battle: &Battle);

}

pub(crate) trait BattleCloser: Completable {

    fn spawn(&mut self, battle: &Battle, text: &mut DynamicText);
    
    fn update(&mut self, ctx: &mut Context, delta: f32, text: &mut DynamicText);

    fn draw(&self, ctx: &mut Context);

    fn draw_battle(&self, ctx: &mut Context);

    fn world_active(&self) -> bool;

}