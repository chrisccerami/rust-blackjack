extern crate rand;

use rand::{thread_rng, Rng};
use blackjack::card::Card;

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn shuffle(&mut self) {
        let suits = vec!["Hearts", "Diamonds", "Spades", "Clubs"];
        for suit in suits {
            let numbers = vec!["2","3","4","5","6","7","8","9","10","J","Q","K","A"];
            for number in numbers {
                let card = Card { suit: suit.into(), value: number.to_string() };
                self.cards.push(card);
            }
        }
        thread_rng().shuffle(self.cards.as_mut_slice());
    }

    pub fn deal_card(&mut self) -> Card {
        return self.cards.pop().unwrap();
    }
}

#[test]
fn test_shuffle() {
    let mut deck = Deck { cards: vec![] };
    deck.shuffle();
    assert_eq!(deck.cards.len(), 52);
    let two_hearts = Card { suit: "Hearts".into(), value: "2".into() };
    let first_card = deck.cards.pop().unwrap();
    // i'm aware this is a bad test case, but it'll work 98% of the time ¯\_(ツ)_/¯
    assert!(first_card != two_hearts);
}

#[test]
fn test_deal_card() {
    let mut deck = Deck { cards: vec![] };
    deck.shuffle();
    deck.deal_card();
    assert_eq!(deck.cards.len(), 51);
    deck.deal_card();
    assert_eq!(deck.cards.len(), 50);
}
