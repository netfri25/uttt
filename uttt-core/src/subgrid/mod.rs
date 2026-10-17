use crate::grid::{GridIndex, SIZE, Winner};
use crate::player::Player;

pub trait Subgrid {
    fn get(&self, index: GridIndex) -> Option<Player>;
    fn set(&mut self, index: GridIndex, player: Player) -> bool;

    fn subgrid_winner(&self) -> Winner {
        let index = GridIndex::ALL;
        for i in 0..3 {
            // rows
            if let Some(player) = self.get(index[i * 3])
                && self.get(index[i * 3]) == self.get(index[i * 3 + 1])
                && self.get(index[i * 3]) == self.get(index[i * 3 + 2])
            {
                return Winner::Player(player);
            }

            // columns
            if let Some(player) = self.get(index[i])
                && self.get(index[i]) == self.get(index[i + 3])
                && self.get(index[i]) == self.get(index[i + 6])
            {
                return Winner::Player(player);
            }
        }

        // grid[4] = center
        let Some(center) = self.get(index[4]) else {
            return Winner::Ongoing;
        };

        // diagonals
        #[rustfmt::skip]
        if Some(center) == self.get(index[0]) && Some(center) == self.get(index[8])
        || Some(center) == self.get(index[2]) && Some(center) == self.get(index[6]) {
            return Winner::Player(center);
        };

        if GridIndex::ALL
            .into_iter()
            .map(|i| self.get(i))
            .all(|p| p.is_some())
        {
            Winner::Tie
        } else {
            Winner::Ongoing
        }
    }
}

impl Subgrid for [Option<Player>; SIZE] {
    fn get(&self, index: GridIndex) -> Option<Player> {
        self[*index as usize]
    }

    fn set(&mut self, index: GridIndex, player: Player) -> bool {
        if self[*index as usize].is_some() {
            return false;
        }

        self[*index as usize] = Some(player);
        true
    }
}
