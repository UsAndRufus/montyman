use rand::{thread_rng, Rng};
use indextree::*;

use nineman::game::*;
use nineman::player::InputHandler;

pub struct Monty {
    pub tree: Arena<GameState>,
    pub root: Option<NodeId>,
}

impl Monty {
}

impl InputHandler for Monty {
    fn update_game_state(&mut self, game_state: GameState) {
        let mut tree = Arena::new();
        let root = tree.new_node(game_state);
        self.tree = tree;
        self.root = Some(root);
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
        format!("Monty InputHandler: {:?}", self.tree)
    }
}
