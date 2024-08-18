use mancala_logic::board::{Board, Position, NUM_POCKETS};
use mancala_logic::game::{select, Turn};
use mancala_logic::player::Player;
use std::io;

fn get_input() -> usize {
    let mut cmd = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read line");
    let cmd_int: usize = cmd.trim().parse::<usize>().expect("Please type a number!") - 1;
    if !(0..NUM_POCKETS).contains(&cmd_int) {
        // TODO: Retry
        println!("Please type a number between 1 and {NUM_POCKETS}");
        panic!()
    }
    cmd_int
}

fn main() {
    let mut board = Board::new();
    let mut turn = Turn::InProgress { next: Player::A };

    loop {
        mancala_cli::print(&board);
        match turn {
            Turn::InProgress { next } => {
                let cmd = get_input();
                (turn, board) = select(
                    &board,
                    &Position::Pocket {
                        player: next,
                        idx: cmd,
                    },
                );
            }
            Turn::Finished { winner } => {
                let player_name = match winner {
                    Player::A => "First player",
                    Player::B => "Second player",
                };
                println!("{player_name} won!");
                break;
            }
        }
    }
}
