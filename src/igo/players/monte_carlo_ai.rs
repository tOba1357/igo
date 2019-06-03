use std::cell::RefCell;
use std::rc::Rc;

use rand::prelude::ThreadRng;
use rand::seq::{SliceRandom, IteratorRandom};

use igo::board::{Board, Cell, BoardDirectionIter};
use igo::game::{Turn, Winner};
use igo::players::Player;
use std::cmp::Ordering;
use rand::distributions::{Beta, Distribution};

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
            nodes.iter()
                .map(|node| (node, Beta::new((self.win_count + 1) as f64, (self.simulate_num - self.win_count + 1) as f64).sample(&mut rand::thread_rng())))
                .max_by(|(_, v1), (_, v2)| if v1 > v2 { Ordering::Greater } else { Ordering::Less })
                .map(|(node, _)| node)
                .clone()
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
            if board.is_put(&x, &y, &turn.to_cell()).0 {
                puttables.push((x, y));
            }
        }
    }
    puttables
}

pub fn is_live(board: &Board, x: &usize, y: &usize, turn: &Turn) -> bool {
    let cell = turn.to_cell();
    let is_surround = BoardDirectionIter::new(*x, *y, &board).all(|(x, y)| { board.cells[x][y] == cell });
    if is_surround {
        let enemy_cell = turn.get_next_turn().to_cell();
        if (*x == 0 || *y == 0) || board.cells[x - 1][y - 1] == enemy_cell {
            if *y == 0 || *x >= board.size - 1 || board.cells[x + 1][y - 1] == enemy_cell {
                if *y <= 1 || board.cells[*x][y - 2] == enemy_cell {
                    return true
                }
            }
            if *x == 0 || *y >= board.size - 1 || board.cells[x - 1][y + 1] == enemy_cell {
                if *x <= 1 || board.cells[x - 2][*y] == enemy_cell {
                    return true
                }
            }
        }
        if (*x >= board.size - 1 || *y >= board.size - 1) || board.cells[x + 1][y + 1] == enemy_cell {
            if *x == 0 || *y >= board.size - 1 || board.cells[x - 1][y + 1] == enemy_cell {
                if *y >= board.size - 2 || board.cells[*x][y + 2] == enemy_cell {
                    return true
                }
            }
            if *y == 0 || *x >= board.size - 1 || board.cells[x + 1][y - 1] == enemy_cell {
                if *x >= board.size - 2 || board.cells[x + 2][*y] == enemy_cell {
                    return true
                }
            }
        }
        false
    } else {
        true
    }

}

impl MonteCarloAi {
    // return Winner and count
    fn playout(&mut self, node: Rc<RefCell<Node>>, turn: &Turn, is_root: bool) -> (Winner, i32) {
        println!("{:?}", node.borrow().board);
        let board = node.borrow().board.clone();
        let nodes_present = node.borrow().nodes.is_some();
        if !nodes_present {
            node.borrow_mut().set_next_nodes(turn);
        }
        let mut n = 0;
        let winner = match node.borrow().max_node() {
            Some(node) => {
                let t = self.playout(node.clone(), &turn.get_next_turn(), false);
                n = t.1;
                t.0
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
        (winner, n + 1)
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
        let mut c = 0;
        loop {
            self.playout_num += 1;
            let (_, count) = self.playout(next_node.clone(), turn, true);
            c += count;
            if c > self.playout_count { break }
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
