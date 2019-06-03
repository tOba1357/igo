extern crate igo;

use igo::igo::board::Cell;
use igo::igo::board::Board;
use igo::igo::game::Game;
use igo::igo::players::random_ai::RandomAI;
use std::rc::Rc;
use std::cell::RefCell;
use igo::igo::players::monte_carlo_ai::MonteCarloAi;

fn main() {
    let mut black_player = Rc::new(RefCell::new(MonteCarloAi::new(1000)));
    let mut white_player = Rc::new(RefCell::new(RandomAI::new()));
    let mut game = Game::new(black_player, white_player);
    let winner = game.start();
    println!("{:?}", winner);
}
