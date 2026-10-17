use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::grid::{Grid, Pos};
use crate::player::Player;

#[derive(Debug)]
pub struct Logger<G> {
    grid: G,
    target: Option<File>,
}

impl<G> Logger<G> {
    fn log(&mut self, pos: Pos, player: Player) {
        if let Some(ref mut target) = self.target {
            let message = format!("{:?},{},{}", player, *pos.outer, *pos.inner);
            eprintln!("step: {}", message);
            writeln!(target, "{}", message).ok();
        }
    }
}

impl<G> Default for Logger<G>
where
    G: Default,
{
    fn default() -> Self {
        let path = std::env::var("LOG_PATH").ok();
        let target = path.and_then(|path| {
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path)
                .ok()
        });

        Self {
            grid: Default::default(),
            target,
        }
    }
}

impl<G> std::ops::DerefMut for Logger<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<G> std::ops::Deref for Logger<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<G> Grid for Logger<G>
where
    G: Grid,
{
    fn get(&self, pos: Pos) -> Option<Player> {
        self.grid.get(pos)
    }

    fn set(&mut self, pos: Pos, player: Player) -> bool {
        if self.grid.set(pos, player) {
            self.log(pos, player);
            true
        } else {
            false
        }
    }
}
