mod card;
mod deck;
mod game;
use crate::game::{RoundResult, play_round, start};
use game::Game;

fn main() {
    if !start() {
        println!("You should start");
        return;
    }
    let mut game = Game::new();
    while play_round(&mut game) == RoundResult::Continue {}
    println!("Thanks for playing")
}
