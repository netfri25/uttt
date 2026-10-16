use crate::grid::GridIndex;
use crate::player::Player;

use super::{Grid, Pos};

// TODO: make this a newtype
pub const SIZE: usize = 9;
pub type ShallowGrid = [Option<Player>; SIZE];

#[derive(Default)]
pub struct NaiveGrid {
    values: [ShallowGrid; SIZE],
}

impl NaiveGrid {
    pub fn grid_at(&self, index: GridIndex) -> ShallowGrid {
        self.values[*index as usize]
    }
}

impl Grid for NaiveGrid {
    fn get(&self, pos: Pos) -> Option<Player> {
        self.values[*pos.outer as usize][*pos.inner as usize]
    }

    fn set(&mut self, pos: Pos, player: Player) -> bool {
        let cell = &mut self.values[*pos.outer as usize][*pos.inner as usize];
        let assign = cell.is_none();
        if assign {
            *cell = Some(player);
        }

        assign
    }
}
