use igo::board::{Board, Cell};
use std::fs;

#[test]
fn is_put() {
    let mut board = Board::new(9);
    assert!(board.is_put(&2, &2, &Cell::Black));
    board.put(&2, &2, Cell::Black);
    assert!(!board.is_put(&2, &2, &Cell::Black));

    board.put(&2, &4, Cell::Black);
    board.put(&1, &3, Cell::Black);
    board.put(&3, &3, Cell::Black);
    assert!(!board.is_put(&2, &3, &Cell::White));
}

#[test]
fn is_put3() {
//B  WWBW B
//WWWBBBWWB
//WWBBWB BW
//BB BWBBBW
//WBB BBBBW
//WWWWWWBWB
//WWWBBBWB
//W WB WBBW
//W WWBB B
//
//69: Black 0, 7
    let content = fs::read_to_string("resources/board2").unwrap();
    let mut board = Board::new(9);
    board.set_from_str(content);
    println!("{:?}", board);
    let is_put = board.is_put(&0, &7, &Cell::Black);
    board.put(&0, &7, Cell::Black);
    println!("{:?}", board);
    assert!(!is_put);
}

#[test]
fn is_put4() {
// W BW
//WWWWB BWB
//WWWB WBBW
//W WWWBW W
//WWWWWW WW
//WWWBWBWWW
//WWWWWWWBW
//WBWWWWB B
//W WWWBBBB
//
//118: Black 0, 0
    let content = fs::read_to_string("resources/board3").unwrap();
    let mut board = Board::new(9);
    board.set_from_str(content);
    println!("{:?}", board);
    let is_put = board.is_put(&0, &0, &Cell::Black);
    board.put(&0, &0, Cell::Black);
    println!("{:?}", board);
    assert!(!is_put);
}