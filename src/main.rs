extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    let mut deck = Deck { cards: vec![] };
    deck.shuffle();
    let first_card = deck.deal_card();
    let second_card = deck.deal_card();
    println!("First Card: {}, score: {}", first_card.name(), first_card.score());
    println!("Second Card: {}, score: {}", second_card.name(), second_card.score());
}

struct Deck {
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

struct Card {
    pub suit: String,
    pub value: String
}

impl Card {
    pub fn score(&self) -> u32 {
        let number = match self.value.parse::<u32>() {
            Ok(x) => x,
            Err(_) => self.score_for_face_card()
        };
        return number;
    }

    pub fn name(&self) -> String {
        return format!("{} of {}", &self.value, &self.suit);
    }

    fn score_for_face_card(&self) -> u32 {
        let score = match self.value.as_str() {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => 0
        };
        return score;
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        (&self.suit, &self.value) == (&other.suit, &other.value)
    }
}

#[test]
fn test_card_score_for_numbers() {
    let card = Card { suit: "Hearts".into(), value: "2".into() };
    assert_eq!(card.score(), 2);
}

#[test]
fn test_card_score_for_face_card() {
    let card = Card { suit: "Hearts".into(), value: "K".into() };
    assert_eq!(card.score(), 13);
}

#[test]
fn test_card_name() {
    let card = Card { suit: "Hearts".into(), value: "K".into() };
    assert_eq!(card.name(), String::from("K of Hearts"));
}

#[test]
fn test_deck_shuffle() {
    let mut deck = Deck { cards: vec![] };
    deck.shuffle();
    assert_eq!(deck.cards.len(), 52);
    let two_hearts = Card { suit: "Hearts".into(), value: "2".into() };
    assert!(deck.cards[0] != two_hearts);
}
