use crate::board::{Board, Position, PositionIter};
use crate::player::Player;

#[derive(PartialEq, Debug)]
pub enum Turn {
    InProgress { next: Player },
    Finished { winner: Player },
}

pub fn select(board: &Board, pos: &Position) -> (Turn, Board) {
    // TODO: Return an error if pos is a store.
    // TODO: Handle the special pattern of retry.
    // TODO: Handle the finished state.
    // TODO: Handle invalid succession of one player.
    match pos {
        Position::Store { player: _ } => panic!(),
        Position::Pocket { player, idx: _ } => {
            let count = board[*pos];

            let mut board = board.clone();
            let pos_iter = PositionIter { pos: *pos };
            board.update(pos, 0);

            for pos in pos_iter.take(count as usize) {
                board.update(&pos, board[pos] + 1);
            }

            (
                Turn::InProgress {
                    next: player.other(),
                },
                board,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_init_normal() {
        let board = Board::new();
        let (turn, board) = select(
            &board,
            &Position::Pocket {
                player: Player::A,
                idx: 0,
            },
        );

        assert_eq!(Turn::InProgress { next: Player::B }, turn);
        assert_eq!(
            0,
            board[Position::Pocket {
                player: Player::A,
                idx: 0
            }]
        );
    }

    #[test]
    fn select_init_successive() {
        // TODO: Add
    }

    #[test]
    fn select_init_backfire() {
        let mut board = Board::new();
        board.update(
            &Position::Pocket {
                player: Player::A,
                idx: 0,
            },
            100,
        );
        let (_, board) = select(
            &board,
            &Position::Pocket {
                player: Player::A,
                idx: 0,
            },
        );

        assert_ne!(
            0,
            board[Position::Pocket {
                player: Player::A,
                idx: 0
            }]
        );
    }

    #[test]
    fn select_win() {
        // TODO: Add
    }
}
