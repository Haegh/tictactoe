mod board;
mod player;
mod state;

use board::Board;
use player::Player;
use state::State;


/// Main method of the game
fn main() {
    let player_list: Vec<Player> = vec![
        Player::new(1, String::from("O")), 
        Player::new(2, String::from("X")),
    ];
    let mut game: Board = Board::new(player_list);

    game.show();
    while game.state == State::Start {
        game.play();
        game.show();
        game.ending_check();
        game.new_turn();
    };
}
