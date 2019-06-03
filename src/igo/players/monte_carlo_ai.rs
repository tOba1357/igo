use std::cell::RefCell;
use std::rc::Rc;

use rand::prelude::ThreadRng;
use rand::seq::{SliceRandom, IteratorRandom};

use igo::board::{Board, Cell, BoardDirectionIter};
use igo::game::{Turn, Winner};
use igo::players::Player;
use std::cmp::Ordering;

pub struct MonteCarloAi {
    rng: ThreadRng,
    playout_count: i32,
    playout_num: i32,
    node: Rc<RefCell<Node>>,
}

struct Node {
    board: Board,
    win_count: i32,
    simulate_num: i32,
    utc: f64,
    point: Option<(usize, usize)>,
    nodes: Option<Vec<Rc<RefCell<Node>>>>,
}

impl Node {
    fn set_nodes(&mut self, nodes: Vec<Rc<RefCell<Node>>>) {
        self.nodes = Some(nodes);
    }

    fn win(&mut self, n: i32) {
        self.win_count += 1;
        self.simulate_num += 1;
        self.utc = self.utc(n);
    }

    fn lose(&mut self, n: i32) {
        self.simulate_num += 1;
        self.utc = self.utc(n);
    }

    fn utc(&self, n: i32) -> f64 {
        self.win_count as f64 / (self.simulate_num + 1) as f64 + (2.0 as f64).sqrt() * ((n as f64).ln() / (self.simulate_num + 1) as f64)
    }

    fn max_node(&self) -> Option<&Rc<RefCell<Node>>> {
        if let Some(ref nodes) = self.nodes {
            nodes.iter().max_by(|node1, node2| if node1.borrow().utc > node2.borrow().utc { Ordering::Greater } else { Ordering::Less }).clone()
        } else {
            None
        }
    }

    fn set_next_nodes(&mut self, turn: &Turn) {
        self.nodes = Some(calc_puttables(&self.board, turn).iter()
            .filter(|&(put_x, put_y)| is_live(&self.board, put_x, put_y, turn))
            .map(|(x, y)| {
                let mut board = self.board.clone();
                board.put(x, y, turn.to_cell());
                Rc::new(RefCell::new(Node {
                    board,
                    win_count: 0,
                    simulate_num: 0,
                    utc: 0.0,
                    point: Some((*x, *y)),
                    nodes: None,
                }))
            }).collect());
    }
}

impl MonteCarloAi {
    pub fn new(playout_count: i32) -> MonteCarloAi {
        MonteCarloAi {
            rng: rand::thread_rng(),
            playout_count,
            playout_num: 0,
            node: Rc::new(RefCell::new(Node {
                board: Board::new(9),
                win_count: 0,
                simulate_num: 0,
                utc: 0.0,
                point: None,
                nodes: None,
            })),
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
    let mut all = true;
    BoardDirectionIter::new(*x, *y, &board).for_each(|(x, y)| {
        if board.cells[x][y] == cell {
            queue.push((x, y));
        } else if board.cells[x][y] == Cell::None {
            count += 1;
            all = false;
        } else {
            all = false;
        }
        rem[x][y] = true;
    });
    if all { return false; }
    loop {
        if count >= 2 { return true; }
        if queue.len() <= i { break; }
        let x = queue[i].0;
        let y = queue[i].1;
        BoardDirectionIter::new(x, y, &board).for_each(|(x, y)| {
            if rem[x][y] { return; }
            if board.cells[x][y] == cell {
                queue.push((x, y));
            } else if board.cells[x][y] == Cell::None {
                count += 1;
            }
            rem[x][y] = true;
        });
        i += 1;
    }
    false
}

impl MonteCarloAi {
    // return Winner and count
    fn playout(&mut self, node: Rc<RefCell<Node>>, turn: &Turn, is_root: bool) -> Winner {
        let board = node.borrow().board.clone();
        let nodes_present = node.borrow().nodes.is_some();
        if !nodes_present {
            node.borrow_mut().set_next_nodes(turn);
        }
        let winner = match node.borrow().max_node() {
            Some(node) => {
                self.playout(node.clone(), &turn.get_next_turn(), false)
            }
            None => {
                if node.borrow().simulate_num > 0 {
                    if node.borrow().win_count > 0 {
                        turn.to_winner()
                    } else {
                        turn.get_next_turn().to_winner()
                    }
                } else {
                    board.calc_winner()
                }
            }
        };
        if winner == turn.to_winner() {
            node.borrow_mut().win(self.playout_num);
        } else {
            node.borrow_mut().lose(self.playout_num);
        }
        winner
    }
}

impl Player for MonteCarloAi {
    fn put(&mut self, board: Rc<RefCell<Board>>, turn: &Turn) -> Option<(usize, usize)> {
        let is_present = self.node.borrow().nodes.is_some().clone();
        let next_node = if is_present {
            if let Some((x, y, cell)) = board.borrow().points.last().unwrap() {
                let mut append_node = None;
                let next_node = match self.node.borrow().nodes {
                    None => { panic!() }
                    Some(ref nodes) => {
                        let next_node = nodes.iter()
                            .find(|node| node.borrow().board.cells[*x][*y] == *cell);
                        match next_node {
                            None => {
                                append_node = Some(Rc::new(RefCell::new(Node {
                                    board: board.borrow().clone(),
                                    win_count: 0,
                                    simulate_num: 0,
                                    utc: 0.0,
                                    point: Some((*x, *y)),
                                    nodes: None,
                                })));
                                append_node.clone().unwrap()
                            }
                            Some(next_node) => { next_node.clone() }
                        }
                    }
                };
                append_node.iter().for_each(|node| self.node = node.clone());
                next_node
            } else {
                return None;
            }
        } else {
            let board = board.borrow().clone();
            self.node.borrow_mut().set_next_nodes(turn);
            self.node.clone()
        };
        for _ in 0..self.playout_count {
            self.playout_num += 1;
            self.playout(next_node.clone(), turn, true);
        }
        let point = if let Some(max_node) = next_node.borrow().max_node() {
            self.node = max_node.clone();
            max_node.borrow().point
        } else {
            None
        };
        point
    }
}
