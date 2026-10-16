#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    #[default]
    X,
    O,
}

impl Player {
    pub fn flip(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}
