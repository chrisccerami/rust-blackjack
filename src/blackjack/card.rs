pub struct Card {
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
fn test_score_for_numbers() {
    let card = Card { suit: "Hearts".into(), value: "2".into() };
    assert_eq!(card.score(), 2);
}

#[test]
fn test_score_for_face_card() {
    let card = Card { suit: "Hearts".into(), value: "K".into() };
    assert_eq!(card.score(), 13);
}

#[test]
fn test_name() {
    let card = Card { suit: "Hearts".into(), value: "K".into() };
    assert_eq!(card.name(), String::from("K of Hearts"));
}
