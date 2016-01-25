use blackjack::card::Card;

#[derive(Clone)]
pub struct Player {
    pub hand: Vec<Card>,
    pub name: String
}

impl Player {
    pub fn score(&self) -> u32 {
        let mut score = 0;
        for card in &self.hand {
            score += card.score();
        }
        return score;
    }
}

#[test]
fn test_score() {
    let two = Card { suit: "Hearts".into(), value: "2".into() };
    let ace = Card { suit: "Spades".into(), value: "A".into() };
    let player = Player { hand: vec![two, ace], name: "Chris".into() };
    assert_eq!(player.score(), 13);
}
