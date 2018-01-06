use rand::{thread_rng, Rng};
use indextree::*;

use nineman::game::*;
use nineman::game::PlyType::*;
use nineman::player::InputHandler;

pub struct Monty {
    pub tree: Arena<GameState>,
    pub root: Option<NodeId>,
}

impl Monty {

    fn mcts(&self) -> String {
        "".to_string()
    }

    fn select(&self) {

    }

    fn expand(&self) {

    }

    fn simulate(&self) {

    }

    fn update(&self) {

    }

    fn create_children(&mut self, node: NodeId) {
        let children = self.tree[node].data.children();

        for child in children {
            let new_node = self.tree.new_node(child);
            node.append(new_node, &mut self.tree);
        }
    }

    fn random_placement(&self, available_places: Vec<String>) -> String {
        let children: Vec<&GameState>
            = self.root.unwrap().children(&self.tree)
                        .map(|c| &self.tree[c])
                        .map(|n| &n.data)
                        .collect();

        // Completely random choice for now!
        let chosen: PlyType = thread_rng().choose(&children).unwrap().move_to_get_here.clone();

        match chosen {
            Placement {player_id, piece_id} => {
                assert!(available_places.contains(&piece_id));
                piece_id
            },
            _ => panic!("Moved from a placement node using {:?}", chosen),
        }
    }
}

impl InputHandler for Monty {
    fn update_game_state(&mut self, game_state: GameState) {
        let mut tree = Arena::new();
        let root = tree.new_node(game_state);
        self.tree = tree;
        self.root = Some(root);
        self.create_children(root);
    }

    fn get_placement(&self, available_places: Vec<String>) -> String {
        self.random_placement(available_places)
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
