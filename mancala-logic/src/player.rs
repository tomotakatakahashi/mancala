#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}
