use rand::{thread_rng, Rng};
use indextree::Arena;

use std::cell::RefCell;

use nineman::game::Game;
use nineman::player::InputHandler;

use game_state::GameState;

pub struct Monty {
    pub tree: RefCell<Arena<GameState>>,
}

impl Monty {
}

impl InputHandler for Monty {
    fn update_game(&self, game: &Game) {
        let mut tree = Arena::new();
        let game_state = GameState::from_game(game);
        tree.new_node(game_state);
        *self.tree.borrow_mut() = tree;
    }

    fn get_placement(&self, available_places: Vec<String>) -> String {
        thread_rng().choose(&available_places).unwrap().to_string()
    }

    fn get_move(&self, available_moves: Vec<(String, String)>) -> (String, String) {
        thread_rng().choose(&available_moves).unwrap().to_owned()
    }

    fn get_mill(&self, available_mills: Vec<String>) -> String {
        thread_rng().choose(&available_mills).unwrap().to_string()
    }

    fn to_string(&self) -> String {
        format!("Monty InputHandler: {:?}", self.tree.borrow())
    }
}
