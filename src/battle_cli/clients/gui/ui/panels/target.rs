use crate::{
    util::Reset,
    pokedex::battle::{
        view::PokemonView,
        party::knowable::BattlePartyUnknown,
    },
    input::{pressed, Control},
    gui::Panel,
    text::TextColor,
    graphics::{draw_text_left, draw_cursor}, 
    tetra::Context, 
};

pub struct TargetPanel {

    // pub cols: usize,

    panel: Panel,

    pub names: Vec<Option<String>>,
    pub cursor: usize,

}

impl TargetPanel {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            panel: Panel::new(ctx),
            names: Vec::with_capacity(4),
            cursor: 0,
        }
    }

    pub fn update_names<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord>(&mut self, targets: &BattlePartyUnknown<ID>) {
        self.names.clear();
        for index in targets.active.iter() {
            self.names.push(index.as_ref().map(|index| targets.pokemon[*index].as_ref().map(|pokemon| pokemon.name()).unwrap_or("Unknown").to_owned()));
        }
    }

    pub fn input(&mut self, ctx: &Context) {
        if pressed(ctx, Control::Up) && self.cursor >= 2 {
            self.cursor -= 2;
        } else if pressed(ctx, Control::Down) && self.cursor <= 2 {
            self.cursor += 2;
        } else if pressed(ctx, Control::Left) && self.cursor > 0 {
            self.cursor -= 1;
        } else if pressed(ctx, Control::Right) && self.cursor < 3 {
            self.cursor += 1;
        }
        if self.cursor >= self.names.len() {
            self.cursor = self.names.len() - 1;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.panel.draw(ctx, 0.0, 113.0, 160.0, 47.0);
        for (index, name) in self.names.iter().enumerate() {
            let x_offset = if index % 2 == 1 {
                72.0
            } else {
                0.0
            };
            let y_offset = if index >> 1 == 1 {
                17.0
            } else {
                0.0
            };
            draw_text_left(ctx, &0, name.as_ref().map(|name| name.as_str()).unwrap_or("None"), &TextColor::Black, 16.0 + x_offset, 121.0 + y_offset);
            if index == self.cursor {
                draw_cursor(ctx, 10.0 + x_offset, 123.0 + y_offset);
            }
        }
    }

}

impl Reset for TargetPanel {
    fn reset(&mut self) {
        let len = self.names.len();
        if self.cursor >= len {
            self.cursor = 0;
        }
    }
}