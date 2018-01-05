extern crate montyman;
extern crate nineman;

use nineman::game::Game;
use nineman::player::Player;
use nineman::player::HumanInput;
use nineman::player::RandomInput;
use nineman::player::InputHandler;

use montyman::monty_input::MontyInput;
use montyman::game_state::GameState;

fn main() {
    let p1 = Player::new(String::from("Ruth"), 1, Box::new(RandomInput {}));

    let p2 = Player::new(String::from("Monty"), 2, Box::new(MontyInput {}));

    let mut game = Game::new(p1, p2);

    println!("{:?}", game);
    println!();

    let game_state = GameState::at_start(&game.board);

    game.game_loop();

    println!("winner: {:?}", game_state.winner());
}
