extern crate montyman;
extern crate nineman;
extern crate indextree;

use nineman::game::Game;
use nineman::player::Player;
use nineman::player::HumanInput;
use nineman::player::RandomInput;

use montyman::Monty;
use montyman::game_state::GameState;

use indextree::Arena;

fn main() {
    let p1 = Player::new(String::from("Ruth"), 1, Box::new(RandomInput {}));

    let p2 = Player::new(String::from("Monty"), 2, Box::new(Monty {}));

    let mut game = Game::new(p1, p2);

    println!("{:?}", game);
    println!();

    let game_state = GameState::from_game(&game);

    //game.game_loop();

    let arena = &mut Arena::new();
    let a = arena.new_node(game_state);

    println!("{:?}", arena);
}
