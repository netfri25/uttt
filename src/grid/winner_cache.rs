use std::cell::Cell;

use crate::grid::{Grid, SIZE, Winner};
use crate::player::Player;

use super::{GridIndex, Pos};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WinnerCache<G> {
    grid: G,
    winner: [Cell<Winner>; SIZE],
}

impl<G> std::ops::DerefMut for WinnerCache<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<G> std::ops::Deref for WinnerCache<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<G> Grid for WinnerCache<G>
where
    G: Grid,
{
    #[inline(always)]
    fn get(&self, pos: Pos) -> Option<Player> {
        self.grid.get(pos)
    }

    #[inline(always)]
    fn set(&mut self, pos: Pos, player: Player) -> bool {
        self.grid.set(pos, player)
    }

    #[inline(always)]
    fn winner_of(&self, index: GridIndex) -> Winner {
        let winner = &self.winner[*index as usize];
        if winner.get() == Winner::Ongoing {
            winner.set(self.grid.winner_of(index));
        }

        winner.get()
    }
}
