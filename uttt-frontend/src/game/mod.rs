use macroquad::prelude::*;

use uttt_core::grid::naive::NaiveGrid;
use uttt_core::grid::logger::Logger;
use uttt_core::grid::section_tracker::SectionTracker;
use uttt_core::grid::winner_cache::WinnerCache;
use uttt_core::grid::{Grid as _, Pos, Winner};
use uttt_core::player::Player;

use crate::draw::Draw;
use crate::draw::consts::INNER_PAD_PERCENT;
use crate::draw::grid_layout::GridLayout;
use crate::frame::Frame;

pub type Grid = Logger<SectionTracker<WinnerCache<NaiveGrid>>>;

#[derive(Default)]
pub struct Game {
    grid: Grid,
    turn: Player,
}

impl Game {
    pub fn draw(&self, frame: Frame) {
        if let Winner::Player(winner) = self.grid.winner() {
            winner.draw(frame)
        } else {
            self.grid.draw(frame)
        }
    }

    pub fn update(&mut self, frame: Frame) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        let (x, y) = mouse_position();

        let layout = GridLayout::new(frame).with_pad(INNER_PAD_PERCENT);
        let Some(pos) = layout.pos_at(x, y) else {
            return;
        };

        let success = self.set(pos, self.turn);
        if success {
            self.turn = self.turn.flip();
        }
    }

    pub fn set(&mut self, pos: Pos, player: Player) -> bool {
        self.grid.set(pos, player)
    }
}
