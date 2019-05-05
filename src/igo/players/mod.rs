use std::rc::Rc;
use std::cell::RefCell;
use igo::board::Board;
use igo::game::Turn;

pub mod random_ai;

pub trait Player {
    fn put(&mut self, board: Rc<RefCell<Board>>, turn: &Turn) -> Option<(usize, usize)>;
}
