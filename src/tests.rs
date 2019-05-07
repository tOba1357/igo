use igo::board::{Board, Cell};

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
fn put() {
    let mut board = Board::new(9);
    board.put(&2, &3, Cell::White);
    board.put(&2, &2, Cell::Black);
    board.put(&2, &4, Cell::Black);
    board.put(&1, &3, Cell::Black);
    board.put(&3, &3, Cell::Black);


    println!("{:?}", board);
    assert_eq!(board.cells[2][3], Cell::Kou);
}