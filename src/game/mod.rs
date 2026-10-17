use macroquad::prelude::*;

mod game_update;
mod grid_draw;
mod grid_layout;

use crate::grid::naive::{NaiveGrid, winner_of};
use crate::grid::section_tracker::SectionTracker;
use crate::grid::winner_cache::WinnerCache;
use crate::grid::{Grid as _, GridIndex, Pos};
use crate::player::Player;

pub type Grid = SectionTracker<WinnerCache<NaiveGrid>>;

#[derive(Default)]
pub struct Game {
    grid: Grid,
    turn: Player,
    winner: Option<Player>,
}

impl Game {
    pub fn set(&mut self, pos: Pos, player: Player) -> bool {
        if !self.grid.set(pos, player) {
            return false;
        }

        self.set_winner();

        true
    }

    fn set_winner(&mut self) {
        let winners_grid = GridIndex::ALL.map(|i| self.grid.winner_of(i).as_player());
        self.winner = winner_of(winners_grid).as_player();
    }
}
