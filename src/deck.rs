use crate::card::{Card, Rank, Suit};
use rand::RngExt;

pub struct Deck {
    pub cards: Vec<Card>,
    pub rng: rand::rngs::ThreadRng,
}

impl Deck {
    pub fn draw(&mut self) -> Card {
        let card_index = self.rng.random_range(0..self.cards.len());
        self.cards.swap_remove(card_index)
    }
    pub fn new() -> Self {
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Ace,
        ];
        let suit = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let cards_in_deck: Vec<Card> = suit
            .iter()
            .flat_map(|&suit| ranks.iter().map(move |&rank| Card { rank, suit }))
            .collect();
        Deck {
            cards: cards_in_deck,
            rng: rand::rng(),
        }
    }
}
