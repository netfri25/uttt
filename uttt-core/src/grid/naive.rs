use crate::grid::GridIndex;
use crate::player::Player;
use crate::subgrid::Subgrid as _;

use super::{Grid, Pos, SIZE};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NaiveGrid {
    values: [[Option<Player>; 9]; SIZE],
}

impl NaiveGrid {
    pub fn grid_at(&self, index: GridIndex) -> [Option<Player>; SIZE] {
        self.values[*index as usize]
    }
}

impl Grid for NaiveGrid {
    fn get(&self, pos: Pos) -> Option<Player> {
        self.values[*pos.outer as usize][*pos.inner as usize]
    }

    fn set(&mut self, pos: Pos, player: Player) -> bool {
        let subgrid = &mut self.values[*pos.outer as usize];
        subgrid.set(pos.inner, player)
    }
}
