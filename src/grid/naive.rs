use crate::grid::GridIndex;
use crate::player::Player;

use super::{Grid, Pos, SIZE, Winner};

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
        let cell = &mut self.values[*pos.outer as usize][*pos.inner as usize];
        let assign = cell.is_none();
        if assign {
            *cell = Some(player);
        }

        assign
    }

    fn winner_of(&self, index: GridIndex) -> Winner {
        let grid = self.grid_at(index);
        winner_of(grid)
    }
}

pub fn winner_of(grid: [Option<Player>; SIZE]) -> Winner {
    for i in 0..3 {
        // rows
        if let Some(player) = grid[i * 3]
            && grid[i * 3] == grid[i * 3 + 1]
            && grid[i * 3] == grid[i * 3 + 2]
        {
            return Winner::Player(player);
        }

        // columns
        if let Some(player) = grid[i]
            && grid[i] == grid[i + 3]
            && grid[i] == grid[i + 6]
        {
            return Winner::Player(player);
        }
    }

    // grid[4] = center
    let Some(player) = grid[4] else {
        return Winner::Ongoing;
    };

    // diagonals
    #[rustfmt::skip]
    if grid[4] == grid[0] && grid[4] == grid[8]
    || grid[4] == grid[2] && grid[4] == grid[6] {
        return Winner::Player(player);
    };

    if grid.iter().all(|p| p.is_some()) {
        Winner::Tie
    } else {
        Winner::Ongoing
    }
}
