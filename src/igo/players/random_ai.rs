use std::cell::RefCell;
use std::rc::Rc;

use rand::prelude::ThreadRng;
use rand::seq::{SliceRandom, IteratorRandom};

use igo::board::{Board, Cell, BoardDirectionIter};
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

pub fn calc_puttables(board: &Board, turn: &Turn) -> Vec<(usize, usize)> {
    let size = board.size;
    let mut puttables = Vec::with_capacity(size);
    for x in 0..size {
        for y in 0..size {
            if board.is_put(&x, &y, &turn.to_cell()) {
                puttables.push((x, y));
            }
        }
    }
    puttables
}

pub fn is_live(board: &Board, x: &usize, y: &usize, turn: &Turn) -> bool {
    let mut board = board.clone();
    board.put(x, y, turn.to_cell());
    let mut rem = vec![vec![false; board.size]; board.size];
    let mut queue = Vec::with_capacity(board.size * board.size);
    let mut i = 0;
    let mut count = 0;
    rem[*x][*y] = true;
    let cell = turn.to_cell();
    let mut ok = false;
    BoardDirectionIter::new(*x, *y, &board).for_each(|(x, y)| {
        if board.cells[x][y] == cell {
            queue.push((x, y));
        } else if board.cells[x][y] == Cell::None {
            count += 1;
        }
        rem[x][y] = true;
    });
    loop {
        if count >= 2 { return true }
        if queue.len() <= i { break; }
        let x = queue[i].0;
        let y = queue[i].1;
        BoardDirectionIter::new(x, y, &board).for_each(|(x, y)| {
            if rem[x][y] { return; }
            if board.cells[x][y] == cell {
                queue.push((x, y));
            } else if board.cells[x][y] == Cell::None {
                count+= 1;
            }
            rem[x][y] = true;
        });
        i += 1;
    }
    false
}

impl Player for RandomAI {
    fn put(&mut self, board: Rc<RefCell<Board>>, turn: &Turn) -> Option<(usize, usize)> {
        let board = board.borrow().clone();
        let size = board.size;
        calc_puttables(&board, turn).iter()
            .filter(|&(put_x, put_y)| is_live(&board, put_x, put_y, turn))
            .choose(&mut self.rng).map(|v| *v)
    }
}
