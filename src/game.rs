use crate::card::Card;
use crate::deck::Deck;
use std::io;
use std::thread::sleep;
use std::time::Duration;
pub struct Game {
    pub deck: Deck,
    pub player_hand: Hand,
    pub dealer_hand: Hand,
    pub player_money: u32,
    pub dealer_money: u32,
}
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn total(&self) -> u32 {
        let mut total = 0;
        let mut aces = 0;
        for card in &self.cards {
            total += card.value();
            if card.value() == 11 {
                aces += 1;
            }
        }
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }
        total
    }
}
pub fn initial_draw(game: &mut Game) {
    pause();
    let card = game.deck.draw();
    print_player(&card);
    game.player_hand.cards.push(card);
    pause();
    let card = game.deck.draw();

    game.dealer_hand.cards.push(card);
    pause();
    let card = game.deck.draw();
    print_player(&card);
    game.player_hand.cards.push(card);
    pause();
    let card = game.deck.draw();
    print_dealer(&card);
    game.dealer_hand.cards.push(card);
}
pub fn print_player(card: &Card) {
    println!("You drew a {} {}", card.rank, card.suit())
}
pub fn print_dealer(card: &Card) {
    println!("The dealer drew a {} {}", card.rank, card.suit())
}
pub fn pause() {
    sleep(Duration::from_secs(1));
}

pub fn bet_stage(game: &mut Game) -> u32 {
    println!("Type 'bet' to bet");
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "bet" {
            println!("How much?");
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<u32>() {
                    Ok(bet) => {
                        if bet <= game.player_money && bet <= game.dealer_money {
                            return bet;
                        } else if game.player_money < bet {
                            println!("You don't have that much money")
                        } else {
                            println!("The dealer doesn't have that much money")
                        }
                    }
                    Err(_) => {
                        println!("Not a valid bet")
                    }
                }
            }
        } else {
            println!("You have to bet!")
        }
    }
}

pub fn deal_player(game: &mut Game) -> bool {
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "hit" => {
                let card = game.deck.draw();
                print_player(&card);
                game.player_hand.cards.push(card);
                if game.player_hand.total() > 21 {
                    return false;
                }
            }
            "stand" => {
                break;
            }
            _ => {
                println!("Try again")
            }
        }
    }
    true
}

pub fn deal_dealer(game: &mut Game) -> bool {
    loop {
        println!("Dealer is thinking...");
        pause();
        if game.dealer_hand.total() < 17 {
            let card = game.deck.draw();
            print_dealer(&card);
            game.dealer_hand.cards.push(card);
            if game.dealer_hand.total() > 21 {
                return false;
            }
        } else {
            return true;
        }
    }
}
pub fn start() -> bool {
    println!("type 'start' to start");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "start" {
        println!("Game has started");
        true
    } else {
        false
    }
}
