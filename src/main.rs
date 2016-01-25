extern crate rand;

mod blackjack;

use blackjack::deck::Deck;
use blackjack::player::Player;
use blackjack::game::Game;

fn main() {
    let deck = Deck { cards: vec![] };
    let player_one = Player { hand: vec![], name: "Player One".into() };
    let player_two = Player { hand: vec![], name: "Player Two".into() };
    let mut game = Game { deck: deck, players: vec![player_one, player_two] };
    game.deal();
    game.pick_winner();
}
