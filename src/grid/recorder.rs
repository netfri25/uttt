use crate::grid::{Grid, Pos};
use crate::player::Player;

use super::{GridIndex, Winner};

#[derive(Debug, Default, Clone)]
pub struct Recorder<G> {
    grid: G,
    steps: Vec<Step>,
}

impl<G> Recorder<G> {
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }
}

impl<G> std::ops::DerefMut for Recorder<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<G> std::ops::Deref for Recorder<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<G> Grid for Recorder<G>
where
    G: Grid,
{
    fn get(&self, pos: Pos) -> Option<Player> {
        self.grid.get(pos)
    }

    fn set(&mut self, pos: Pos, player: Player) -> bool {
        if self.grid.set(pos, player) {
            self.steps.push(Step { pos, player });
            true
        } else {
            false
        }
    }

    fn winner_of(&self, index: GridIndex) -> Winner {
        self.grid.winner_of(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Step {
    pub pos: Pos,
    pub player: Player,
}
