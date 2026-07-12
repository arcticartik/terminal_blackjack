use crate::card::Card;
use crate::deck::Deck;
use std::io;
use std::thread::sleep;
use std::time::Duration;
#[derive(PartialEq)]
pub enum RoundResult {
    Continue,
    Quit,
}
#[derive(PartialEq)]
enum Outcome {
    PlayerBust,
    DealerBust,
    PlayerHandWin,
    DealerHandWin,
    Tie,
}
pub struct Game {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
    player_money: u32,
    dealer_money: u32,
}
impl Game {
    pub fn new() -> Self {
        Game {
            deck: Deck::new(),
            player_hand: Hand { cards: vec![] },
            dealer_hand: Hand { cards: vec![] },
            player_money: 100,
            dealer_money: 100000,
        }
    }
}
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn total(&self) -> u32 {
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

fn play_turns(game: &mut Game) -> Outcome {
    pause();
    match player_turn(game) {
        Some(Outcome::PlayerBust) => return Outcome::PlayerBust,
        Some(_) => panic!("unreachable"),
        None => {}
    }
    match dealer_turn(game) {
        Some(Outcome::DealerBust) => return Outcome::DealerBust,
        Some(_) => panic!("unreachable"),
        None => {}
    }
    if game.player_hand.total() > game.dealer_hand.total() {
        return Outcome::PlayerHandWin;
    } else if game.dealer_hand.total() > game.player_hand.total() {
        return Outcome::DealerHandWin;
    } else {
        return Outcome::Tie;
    }
}
fn handle_outcome(game: &mut Game, wager: u32, outcome: Outcome) {
    game.player_money -= wager;
    game.dealer_money -= wager;
    match outcome {
        Outcome::PlayerBust => {
            println!("Player busts");
            game.dealer_money += wager * 2
        }
        Outcome::DealerBust => {
            println!("Dealer busts");
            game.player_money += wager * 2;
        }
        Outcome::PlayerHandWin => {
            println!("Player won by score");
            game.player_money += wager * 2;
        }
        Outcome::DealerHandWin => {
            println!("Dealer won by score");
            game.dealer_money += wager * 2;
        }
        Outcome::Tie => {
            println!("game ended in tie");
            game.player_money += wager;
            game.dealer_money += wager;
        }
    }
}

fn player_draw(game: &mut Game) {
    let card = game.deck.draw();
    print_player_card(&card);
    game.player_hand.cards.push(card);
    print_player_hand(&game.player_hand);
}

fn dealer_draw(game: &mut Game, reveal: bool) {
    let card = game.deck.draw();
    if reveal {
        print_dealer_card(&card);
    } else {
        println!("The dealer draws a hidden card")
    }
    game.dealer_hand.cards.push(card);
}

fn initial_draw(game: &mut Game) {
    player_draw(game);
    pause();
    dealer_draw(game, false);
    pause();
    player_draw(game);
    pause();
    dealer_draw(game, true);
    pause();
}

fn pause() {
    sleep(Duration::from_secs(1));
}

fn bet_stage(game: &mut Game) -> Option<u32> {
    println!("Type 'bet' to bet, and 'leave' to leave the casino");
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "bet" => {
                println!("How much?");
                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    match input.trim().parse::<u32>() {
                        Ok(bet) => {
                            if bet <= game.player_money && bet <= game.dealer_money {
                                return Some(bet);
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
            }
            "leave" => {
                println!("You left the casino with {} dollars", game.player_money);
                return None;
            }
            _ => {
                println!("Try again")
            }
        }
    }
}

fn player_turn(game: &mut Game) -> Option<Outcome> {
    println!("Type 'hit' to hit, and 'stand' to stand");
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "hit" => {
                player_draw(game);
                if is_bust(&game.player_hand) {
                    return Some(Outcome::PlayerBust);
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
    None
}

fn dealer_turn(game: &mut Game) -> Option<Outcome> {
    if let Some(card) = game.dealer_hand.cards.get(0) {
        println!("The dealer's hidden card was a {} {}", card.rank, card.suit)
    } else {
        panic!("Unreachable, dealer_turn");
    }
    pause();

    loop {
        println!("Dealer is thinking...");
        pause();
        if game.dealer_hand.total() < 17 {
            dealer_draw(game, true);
            if is_bust(&game.dealer_hand) {
                return Some(Outcome::DealerBust);
            }
        } else {
            print_dealer_hand(&game.dealer_hand);
            return None;
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
fn print_player_hand(player_hand: &Hand) {
    println!("Your total is now {} ", player_hand.total())
}
fn print_player_card(card: &Card) {
    println!("You drew a {} {}", card.rank, card.suit);
}
fn print_dealer_hand(dealer_hand: &Hand) {
    println!("The dealer's total is now {} ", dealer_hand.total())
}
fn print_dealer_card(card: &Card) {
    println!("The dealer drew a {} {}", card.rank, card.suit);
}
fn reset_game(game: &mut Game) {
    game.deck = Deck::new();
    game.player_hand = Hand { cards: vec![] };
    game.dealer_hand = Hand { cards: vec![] };
}
fn print_money(game: &Game) {
    println!("You have {} dollars", game.player_money);
    println!("The dealer has {} dollars", game.dealer_money);
}

fn is_game_over(game: &Game) -> Option<&str> {
    if game.dealer_money == 0 {
        return Some("The house is out");
    }
    if game.player_money == 0 {
        return Some("You went bust!");
    }
    None
}
fn is_bust(hand: &Hand) -> bool {
    hand.total() > 21
}
pub fn play_round(game: &mut Game) -> RoundResult {
    reset_game(game);
    print_money(&game);
    if let Some(message) = is_game_over(game) {
        println!("{}", message);
        return RoundResult::Quit;
    }
    match bet_stage(game) {
        Some(wager) => {
            initial_draw(game);
            let outcome = play_turns(game);
            handle_outcome(game, wager, outcome);
            RoundResult::Continue
        }
        None => RoundResult::Quit,
    }
}
