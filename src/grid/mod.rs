use std::ops::Deref;

use crate::player::Player;

pub mod naive;

/// container for the game state
pub trait Grid {
    /// get the player in a given position
    fn get(&self, pos: Pos) -> Option<Player>;

    /// set a player in the given grid position.
    /// shouldn't allow placing in a position that can't be played (e.g. occupied / restricted)
    fn set(&mut self, pos: Pos, player: Player) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub outer: GridIndex,
    pub inner: GridIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridIndex(u8);

impl std::ops::DerefMut for GridIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for GridIndex {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<u8> for GridIndex {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // 9: the size of the grid
        (value < 9).then_some(Self(value)).ok_or(())
    }
}

impl GridIndex {
    pub const ALL: [Self; 9] = [
        Self(0),
        Self(1),
        Self(2),
        Self(3),
        Self(4),
        Self(5),
        Self(6),
        Self(7),
        Self(8),
    ];

    pub fn get(&self) -> u8 {
        self.0
    }
}
