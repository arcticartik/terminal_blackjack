use rand::RngExt;
use std::io;
use std::thread::sleep;
use std::time::Duration;

struct Deck {
    cards: Vec<u32>,
}
impl Deck {
    fn draw(&mut self, rng: &mut rand::rngs::ThreadRng) -> u32 {
        let card_index = rng.random_range(0..self.cards.len());
        self.cards.swap_remove(card_index)
    }
}
struct Hand {
    total: u32,
    aces: u32,
}
fn main() {
    let mut player_dollars: u32 = 100;
    let mut dealer_dollars: u32 = 100;

    let mut rng = rand::rng();
    let started = start();
    if !started {
        println!("You should start");
        return;
    }
    loop {
        println!("You have {} dollars", player_dollars);
        println!("The dealer has {} dollars", dealer_dollars);
        if dealer_dollars == 0 {
            println!("The dealer is out");
            break;
        } else if player_dollars == 0 {
            println!("The player is out");
            break;
        }
        let mut player_hand = Hand { total: 0, aces: 0 };
        let mut dealer_hand = Hand { total: 0, aces: 0 };
        let mut cards_in_deck = Vec::with_capacity(52);
        for _suit in 0..=3 {
            cards_in_deck.push(11);
            for pip_card in 2..=10 {
                cards_in_deck.push(pip_card);
            }
            for _face_card in 0..=2 {
                cards_in_deck.push(10);
            }
        }
        let mut deck = Deck {
            cards: cards_in_deck,
        };

        let wager = bet_stage(player_dollars, dealer_dollars);
        player_dollars -= wager;
        dealer_dollars -= wager;
        println!("Type 'hit' to hit, and 'stand' to stand");
        pause();
        draw_card(&mut rng, &mut deck, "Dealer", &mut dealer_hand);
        pause();
        draw_card(&mut rng, &mut deck, "Player", &mut player_hand);
        pause();
        draw_hidden_card(&mut rng, &mut deck, &mut dealer_hand);
        pause();
        draw_card(&mut rng, &mut deck, "Player", &mut player_hand);

        if !deal_player(&mut rng, &mut deck, &mut player_hand) {
            pause();
            println!("Player busts");
            dealer_dollars += wager * 2;
            continue;
        }

        if !deal_dealer(&mut rng, &mut deck, &mut dealer_hand) {
            pause();
            println!("Dealer busts");
            player_dollars += wager * 2;
            continue;
        }
        if player_hand.total > dealer_hand.total {
            pause();
            println!("Player won by score");
            player_dollars += wager * 2;
        } else if dealer_hand.total > player_hand.total {
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
fn ace_logic(hand: &mut Hand) {
    while hand.aces >= 1 && hand.total >= 22 {
        hand.aces -= 1;
        hand.total -= 10;
    }
}
fn draw_card(rng: &mut rand::rngs::ThreadRng, deck: &mut Deck, who: &str, hand: &mut Hand) {
    let card = deck.draw(rng);
    if card == 11 {
        hand.aces += 1;
    }

    let pre = if matches!(card, 8 | 11) { "an" } else { "a" };
    hand.total += card;
    ace_logic(hand);
    println!("{}'s card is {} {}", who, pre, card);
    println!("{}'s total is now {}", who, hand.total);
}
fn draw_hidden_card(rng: &mut rand::rngs::ThreadRng, deck: &mut Deck, hand: &mut Hand) {
    let card = deck.draw(rng);
    if card == 11 {
        hand.aces += 1;
    }

    hand.total += card;
    ace_logic(hand);
}
fn bet_stage(player_dollars: u32, dealer_dollars: u32) -> u32 {
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
                        if bet <= player_dollars && bet <= dealer_dollars {
                            return bet;
                        } else if player_dollars < bet {
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

fn deal_player(rng: &mut rand::rngs::ThreadRng, deck: &mut Deck, hand: &mut Hand) -> bool {
    let mut input = String::new();

    let who = "The player";
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "hit" => {
                draw_card(rng, deck, who, hand);
                if hand.total > 21 {
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

fn deal_dealer(rng: &mut rand::rngs::ThreadRng, deck: &mut Deck, hand: &mut Hand) -> bool {
    let who = "The dealer";

    loop {
        println!("Dealer is thinking...");
        pause();
        if hand.total < 17 {
            draw_card(rng, deck, who, hand);
            if hand.total > 21 {
                return false;
            }
        } else {
            return true;
        }
    }
}

fn start() -> bool {
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
fn pause() {
    sleep(Duration::from_secs(1));
}
