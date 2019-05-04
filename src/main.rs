extern crate igo;
use igo::igo::board::Cell;
use igo::igo::board::Board;

fn main() {
    let mut board = Board::new(9);
    println!("{:?}", board);
    board.put(&1, &1, Cell::White);
    println!("{:?}", board);
}
