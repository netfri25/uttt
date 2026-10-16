use crate::player::Player;

use super::{Grid, Pos};

const SIZE: usize = 9;

#[derive(Default)]
pub struct NaiveGrid {
    values: [[Option<Player>; SIZE]; SIZE],
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
