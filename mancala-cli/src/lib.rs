use mancala::board::{Board, Position, NUM_POCKETS};
use mancala::player::Player;

pub fn print(board: &Board) {
    let pos = Position::Store { player: Player::B };
    let count = board[pos];
    print!("{count}");
    for i in (0..NUM_POCKETS).rev() {
        let pos = Position::Pocket {
            player: Player::B,
            idx: i,
        };
        let count = board[pos];
        print!("\t{count}");
    }
    println!("");

    print!("\t");
    for i in 0..NUM_POCKETS {
        let pos = Position::Pocket {
            player: Player::A,
            idx: i,
        };
        let count = board[pos];
        print!("{count}\t");
    }
    let pos = Position::Store { player: Player::A };
    let count = board[pos];
    println!("\t{count}");
}
