use macroquad::prelude::*;

mod game_draw;
mod game_update;
mod grid_layout;

use crate::grid::naive::NaiveGrid;
use crate::grid::{Grid as _, Pos};
use crate::player::Player;

pub type Grid = NaiveGrid;

#[derive(Default)]
pub struct Game {
    grid: Grid,
    turn: Player,
}

impl Game {
    pub fn set(&mut self, pos: Pos, player: Player) -> bool {
        // TODO: add more checks
        self.grid.set(pos, player)
    }
}
