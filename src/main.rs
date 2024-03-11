pub mod game;
use crate::game::*;

fn main() {
    let game = Game::create(8, 8);

    println!("{}", game);
}

