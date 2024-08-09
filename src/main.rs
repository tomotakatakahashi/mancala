mod cliio;
use mancala::board::Board;

fn main() {
    let board = Board::new();
    cliio::print(&board);
}
