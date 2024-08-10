mod cliio;
use mancala::board::{Board, Position, NUM_POCKETS};
use mancala::game::{select, Turn};
use mancala::player::Player;
use std::io;

fn main() {
    let board = Board::new();
    let turn = Turn::InProgress { next: Player::A };
    cliio::print(&board);

    let mut cmd = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read line");
    let cmd_int: usize = cmd.trim().parse::<usize>().expect("Please type a number!") - 1;
    if !(0..NUM_POCKETS).contains(&cmd_int) {
        println!("Please type a number between 1 and {NUM_POCKETS}");
        panic!()
    }

    let (turn, board) = select(
        &board,
        &Position::Pocket {
            // TODO: Use correct player.
            player: Player::A,
            idx: cmd_int,
        },
    );

    cliio::print(&board);
}
