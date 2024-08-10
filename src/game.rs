use crate::board::{Board, Position, PositionIter};
use crate::player::Player;

#[derive(PartialEq, Debug)]
pub enum Turn {
    InProgress { next: Player },
    Finished { winner: Player },
}

pub fn select(board: &Board, pos: &Position) -> (Turn, Board) {
    // TODO: Return an error if pos is a store.
    // TODO: Handle the finished state.
    // TODO: Handle invalid succession of one player.
    match pos {
        Position::Store { player: _ } => panic!(),
        Position::Pocket { player, idx: _ } => {
            let count = board[*pos];

            let mut board = board.clone();
            let pos_iter = PositionIter { pos: *pos };
            board.update(pos, 0);

            let mut is_last_my_store = false;
            for pos in pos_iter.take(count as usize) {
                board.update(&pos, board[pos] + 1);
                is_last_my_store = pos == Position::Store { player: *player };
            }

            (
                Turn::InProgress {
                    next: if is_last_my_store {
                        *player
                    } else {
                        player.other()
                    },
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
        let orig_count = board[Position::Pocket {
            player: Player::A,
            idx: 0,
        }];
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
        assert_eq!(
            5,
            board[Position::Pocket {
                player: Player::A,
                idx: orig_count as usize,
            }]
        );
        assert_eq!(
            4,
            board[Position::Pocket {
                player: Player::A,
                idx: (orig_count + 1) as usize,
            }]
        );
    }

    #[test]
    fn select_init_successive() {
        let board = Board::new();
        let (turn, _) = select(
            &board,
            &Position::Pocket {
                player: Player::B,
                idx: 2,
            },
        );
        assert_eq!(Turn::InProgress { next: Player::B }, turn);
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
