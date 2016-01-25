use blackjack::deck::Deck;
use blackjack::player::Player;

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck
}

impl Game {
    pub fn deal(&mut self) {
        println!("Game is starting. Good luck!");
        &self.deck.shuffle();
        self.deal_card_to_each_player();
        self.deal_card_to_each_player();
    }

    pub fn pick_winner(& mut self) {
        let player_one = &self.players.pop().unwrap();
        let player_two = &self.players.pop().unwrap();
        if player_one.score() > player_two.score() {
            println!("{} won with a score of {}!", player_one.name, player_one.score());
        } else if player_two.score() > player_one.score() {
            println!("{} won with a score of {}!", player_two.name, player_two.score());
        } else {
            println!("Players tied with a score of {}! Push!", player_one.score());
        }
    }

    fn deal_card_to_each_player(&mut self) {
        for player in &mut self.players {
            let dealt_card = &self.deck.draw_card();
            player.hand.push(dealt_card.clone());
            println!("{} is dealt {}", player.name, dealt_card.name());
        }
    }
}

#[test]
fn test_deal() {
    // let deck = Deck { cards: vec![] };
    // let player_one = Player { hand: vec![] };
    // let player_two = Player { hand: vec![] };
    // let mut game = Game { deck: deck, players: vec![player_one, player_two] };
    // game.start();
    // assert_eq!(player_one.hand.len(), 2);
    // assert_eq!(player_two.hand.len(), 2);
    // assert_eq!(deck.cards.len(), 48);
}
