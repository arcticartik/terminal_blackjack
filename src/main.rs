mod card;
mod deck;
mod game;
use crate::deck::Deck;
use crate::game::Hand;
use crate::game::{bet_stage, deal_dealer, deal_player, initial_draw, pause, start};
use game::Game;

fn main() {
    let mut player_dollars: u32 = 100;
    let mut dealer_dollars: u32 = 100000;

    let started = start();
    if !started {
        println!("You should start");
        return;
    }
    loop {
        let mut game = Game {
            deck: Deck::new(),
            player_hand: Hand { cards: vec![] },
            dealer_hand: Hand { cards: vec![] },
            player_money: player_dollars,
            dealer_money: dealer_dollars,
        };

        println!("You have {} dollars", player_dollars);
        println!("The dealer has {} dollars", dealer_dollars);
        if dealer_dollars == 0 {
            println!("The house is out");
            break;
        } else if player_dollars == 0 {
            println!("You went bust!");
            break;
        }

        let wager = bet_stage(&mut game);
        player_dollars -= wager;
        dealer_dollars -= wager;
        println!("Type 'hit' to hit, and 'stand' to stand");
        initial_draw(&mut game);

        if !deal_player(&mut game) {
            pause();
            println!("Player busts");
            dealer_dollars += wager * 2;
            continue;
        }

        if !deal_dealer(&mut game) {
            pause();
            println!("Dealer busts");
            player_dollars += wager * 2;
            continue;
        }
        if game.player_hand.total() > game.dealer_hand.total() {
            pause();
            println!("Player won by score");
            player_dollars += wager * 2;
        } else if game.dealer_hand.total() > game.player_hand.total() {
            pause();
            println!("Dealer won by score");
            dealer_dollars += wager * 2;
        } else {
            pause();
            println!("game ended in tie");
            player_dollars += wager;
            dealer_dollars += wager;
        }
    }
    println!("Thanks for playing")
}
