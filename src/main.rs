pub mod game;
use crate::game::*;

fn main() {
    let glass = Glass::new(4, vec![Ball::GREEN, Ball::RED]);

    println!("{}", glass);
}

