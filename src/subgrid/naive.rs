use crate::grid::{GridIndex, SIZE, Winner};
use crate::player::Player;
use crate::subgrid::Subgrid;

pub struct NaiveSubgrid([Option<Player>; SIZE]);

impl Subgrid for NaiveSubgrid {
    fn get(&self, index: GridIndex) -> Option<Player> {
        todo!()
    }

    fn set(&mut self, index: GridIndex, player: Player) -> bool {
        todo!()
    }

}

pub fn winner_of(grid: [Option<Player>; SIZE]) -> Winner {
}
