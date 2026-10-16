use macroquad::prelude::*;

mod game_draw;
mod game_update;
mod grid_layout;

use crate::grid::naive::{NaiveGrid, ShallowGrid};
use crate::grid::{Grid as _, Pos};
use crate::player::Player;

pub type Grid = NaiveGrid;

#[derive(Default)]
pub struct Game {
    grid: Grid,
    turn: Player,
    winners_grid: ShallowGrid,
    winner: Option<Player>,
}

impl Game {
    pub fn set(&mut self, pos: Pos, player: Player) -> bool {
        // TODO: add more checks
        if !self.grid.set(pos, player) {
            return false;
        }

        if self.winners_grid[*pos.outer as usize].is_none() {
            self.winners_grid[*pos.outer as usize] = winner_of(self.grid.grid_at(pos.outer));
        }

        self.winner = winner_of(self.winners_grid);

        true
    }
}

fn winner_of(grid: ShallowGrid) -> Option<Player> {
    for i in 0..3 {
        // rows
        if let Some(player) = grid[i * 3]
            && grid[i * 3] == grid[i * 3 + 1]
            && grid[i * 3] == grid[i * 3 + 2]
        {
            return Some(player);
        }

        // columns
        if let Some(player) = grid[i]
            && grid[i] == grid[i + 3]
            && grid[i] == grid[i + 6]
        {
            return Some(player);
        }
    }

    // grid[4] = center
    let player = grid[4]?;

    // diagonals
    #[rustfmt::skip]
    if grid[4] == grid[0] && grid[4] == grid[8]
    || grid[4] == grid[2] && grid[4] == grid[6] {
        return Some(player);
    };

    None
}
