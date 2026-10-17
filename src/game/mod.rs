use macroquad::prelude::*;

use crate::draw::Draw;
use crate::draw::consts::INNER_PAD_PERCENT;
use crate::draw::grid_layout::GridLayout;
use crate::frame::Frame;
use crate::grid::naive::NaiveGrid;
use crate::grid::section_tracker::SectionTracker;
use crate::grid::winner_cache::WinnerCache;
use crate::grid::{Grid as _, GridIndex, Pos};
use crate::player::Player;
use crate::subgrid::Subgrid;

pub type Grid = SectionTracker<WinnerCache<NaiveGrid>>;

#[derive(Default)]
pub struct Game {
    grid: Grid,
    turn: Player,
    winner: Option<Player>,
}

impl Game {
    pub fn draw(&self, frame: Frame) {
        if let Some(winner) = self.winner {
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
        if !self.grid.set(pos, player) {
            return false;
        }

        self.set_winner();

        true
    }

    fn set_winner(&mut self) {
        let winners_grid = GridIndex::ALL.map(|i| self.grid.winner_of(i).as_player());
        self.winner = winners_grid.winner().as_player();
    }
}
