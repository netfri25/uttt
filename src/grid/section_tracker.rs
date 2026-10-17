use crate::grid::{Grid, GridIndex};
use crate::player::Player;

use super::{Pos, Winner};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SectionTracker<G> {
    grid: G,
    section: Option<GridIndex>,
}

impl<G> SectionTracker<G> {
    pub fn section(&self) -> Option<GridIndex> {
        self.section
    }
}

impl<G> std::ops::DerefMut for SectionTracker<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<G> std::ops::Deref for SectionTracker<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<G> Grid for SectionTracker<G>
where
    G: Grid,
{
    #[inline(always)]
    fn get(&self, pos: Pos) -> Option<Player> {
        self.grid.get(pos)
    }

    #[inline]
    fn set(&mut self, pos: Pos, player: Player) -> bool {
        // if can't play in that section
        if self.section.is_some_and(|section| section != pos.outer) {
            return false;
        }

        // if unable to assign
        if !self.grid.set(pos, player) {
            return false;
        }

        // section is the inner position if that section is playable
        let playable = self.winner_of(pos.inner) == Winner::Ongoing;
        self.section = playable.then_some(pos.inner);
        true
    }

    #[inline(always)]
    fn winner_of(&self, index: GridIndex) -> Winner {
        self.grid.winner_of(index)
    }
}
