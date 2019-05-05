use std::cell::RefCell;
use std::rc::Rc;

use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;

use igo::board::Board;
use igo::game::Turn;
use igo::players::Player;

pub struct RandomAI {
    rng: ThreadRng,
}

impl RandomAI {
    pub fn new() -> RandomAI {
        RandomAI {
            rng: rand::thread_rng(),
        }
    }
}


impl Player for RandomAI {
    fn put(&mut self, board: Rc<RefCell<Board>>, turn: &Turn) -> Option<(usize, usize)> {
        let size = (*board).borrow().size;
        let mut puttables = Vec::with_capacity(size);
        for x in 0..size {
            for y in 0..size {
                if board.borrow_mut().is_put(&x, &y) {
                    puttables.push((x, y));
                }
            }
        }
        puttables.choose(&mut self.rng).map(|v| *v)
    }
}
