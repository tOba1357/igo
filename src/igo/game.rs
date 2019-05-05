use igo::board::{Board, Cell};
use std::rc::Rc;
use std::fmt;
use std::cell::RefCell;
use igo::players::Player;

pub struct Game {
    pub board: Rc<RefCell<Board>>,
    turn: Turn,
    black_player: Rc<RefCell<Player>>,
    white_player: Rc<RefCell<Player>>,
}

pub enum Turn {
    Black,
    White,
}

impl Turn {
    pub fn get_next_turn(&self) -> Turn {
        match self {
            Turn::Black => Turn::White,
            Turn::White => Turn::Black,
        }
    }

    pub fn to_cell(&self) -> Cell {
        match self {
            Turn::Black => Cell::White,
            Turn::White => Cell::Black,
        }
    }
}

pub enum Winner {
    Black,
    White,
    None,
}

impl Winner {
    pub fn to_string(&self) -> &str {
        match self {
            Winner::Black => "Black",
            Winner::White => "White",
            Winner::None => "None"
        }
    }
}

impl fmt::Debug for Winner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Game {
    pub fn new(black_player: Rc<RefCell<Player>>, white_player: Rc<RefCell<Player>>) -> Box<Game> {
        Box::from(Game {
            board: Rc::new(RefCell::new(Board::new(9))),
            turn: Turn::Black,
            black_player: black_player.clone(),
            white_player: white_player.clone(),
        })
    }

    pub fn start(&mut self) -> Winner {
        let mut passed = false;
        let mut put_failed_count = 0;
        loop {
            let player = match self.turn {
                Turn::Black => self.black_player.clone(),
                Turn::White => self.white_player.clone(),
            };
            let pos = player.borrow_mut().put(self.board.clone(), &self.turn);
            if let Some((x, y)) = pos {
                if self.board.borrow_mut().put(&x, &y, self.turn.to_cell()) {
                    put_failed_count = 0;
                } else {
                    put_failed_count += 1;
                    if put_failed_count >= 3 {
                        panic!("put failed many time");
                    }
                    continue;
                }
            } else {
                self.board.borrow_mut().pass();
                if passed {
                    break;
                } else {
                    passed = true;
                }
            }
            self.turn = self.turn.get_next_turn();
        }
        Winner::None
    }
}