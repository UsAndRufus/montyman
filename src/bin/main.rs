extern crate montyman;
extern crate nineman;
extern crate indextree;

use nineman::game::Game;
use nineman::player::Player;
use nineman::player::Human;
use nineman::player::Random;

use montyman::Monty;

use indextree::Arena;

fn main() {
    let p1 = Player::new(String::from("Ruth"), 1, Box::new(Human {player_id: 1}));

    let p2 = Player::new(String::from("Monty"), 2,
                            Box::new(Monty { tree: Arena::new(), root: None, player_id: 2}));

    let mut game = Game::new(p1, p2);

    println!("{:?}", game);
    println!();

    game.game_loop();

    //println!("player1: {:?}, player2: {:?}",
    //            game.player1.input_handler.to_string(), game.player2.input_handler.to_string());

}
