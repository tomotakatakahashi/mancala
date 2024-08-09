use crate::player::Player;
use std::ops::Index;

const NUM_POCKETS: usize = 6;
const INITIAL_STONE_COUNT: i32 = 4;

pub struct Board {
    data: [i32; ((NUM_POCKETS + 1) * 2) as usize],
}

pub enum Position {
    Store { player: Player },
    Pocket { player: Player, idx: usize },
}

impl Board {
    pub fn new() -> Self {
        let mut data = [0; (NUM_POCKETS + 1) * 2];
        for i in 0..NUM_POCKETS {
            data[i] = INITIAL_STONE_COUNT;
            data[NUM_POCKETS + 1 + i] = INITIAL_STONE_COUNT;
        }

        Board { data }
    }

    fn position_to_index(pos: &Position) -> usize {
        match pos {
            Position::Pocket {
                player: Player::A,
                idx,
            } => *idx,
            Position::Store { player: Player::A } => NUM_POCKETS,
            Position::Pocket {
                player: Player::B,
                idx,
            } => NUM_POCKETS + 1 + idx,
            Position::Store { player: Player::B } => 2 * NUM_POCKETS + 1,
        }
    }
}

impl Index<Position> for Board {
    type Output = i32;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.data[Self::position_to_index(&pos)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_initial() {
        let board = Board::new();
        assert_eq!(
            4,
            board[Position::Pocket {
                player: Player::A,
                idx: 0
            }]
        );
        assert_eq!(
            4,
            board[Position::Pocket {
                player: Player::A,
                idx: 5
            }]
        );
        assert_eq!(0, board[Position::Store { player: Player::A }]);
        assert_eq!(
            4,
            board[Position::Pocket {
                player: Player::B,
                idx: 0
            }]
        );
        assert_eq!(
            4,
            board[Position::Pocket {
                player: Player::B,
                idx: 5
            }]
        );
        assert_eq!(0, board[Position::Store { player: Player::B }]);
    }
}
