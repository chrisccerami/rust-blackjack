extern crate rand;

mod blackjack;

use blackjack::deck::Deck;

fn main() {
    let mut deck = Deck { cards: vec![] };
    deck.shuffle();
    let first_card = deck.deal_card();
    let second_card = deck.deal_card();
    println!("First Card: {}, score: {}", first_card.name(), first_card.score());
    println!("Second Card: {}, score: {}", second_card.name(), second_card.score());
}
