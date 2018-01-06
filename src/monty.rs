use rand::{thread_rng, Rng};
use indextree::*;

use nineman::game::*;
use nineman::game::PlyType::*;
use nineman::player::InputHandler;

use statistic::Statistic;

const NUMBER_OF_SIMULATIONS: i8 = 10;

pub struct Monty {
    pub tree: Arena<Statistic>,
    pub root: Option<NodeId>,
}

impl Monty {

    fn mcts(&mut self) -> PlyType {
        for _ in 1..NUMBER_OF_SIMULATIONS {
            let node_to_expand = self.select(self.root.unwrap());
            let new_node = self.expand(node_to_expand);
            let payoff = self.simulate(new_node);
            self.update(new_node, payoff);
        }

        self.best_move()
    }

    // Recursively traverse from node, looking for most important expandable node
    // Return when you reach non-expandable node
    fn select(&self, node_id: NodeId) -> NodeId {
        node_id
    }

    // Node is expandable if it is non-terminal and has univisited children
    fn is_expandable(&self, node_id: NodeId) -> bool {
        self.tree[node_id].first_child().is_none()
    }

    // Called when seleciton finishes
    // Choose random unvisited child to add to the tree
    fn expand(&mut self, node_id: NodeId) -> NodeId {
        let child_gs;
        {
            let node = &self.tree[node_id];

            child_gs = thread_rng().choose(&node.data.game_state.children()).unwrap().clone();
        }

        let child_statistic = Statistic::new(child_gs);
        self.tree.new_node(child_statistic)

    }


    // Random playout on new node
    // Play until the end
    // Return 1 for win, 0 for lose
    fn simulate(&self, node_id: NodeId) -> i8 {
        0
    }

    // Back-propogate value from simulation to new node and ancestors
    fn update(&mut self, node_id: NodeId, payoff: i8) {
        let ancestor_ids: Vec<_> = node_id.ancestors(&self.tree).collect();

        for ancestor_id in ancestor_ids {
            let ancestor = &mut self.tree[ancestor_id];
            ancestor.data.visit(payoff);
        }
    }

    // End the search and make a move
    // Currently using most robust child (most visited)
    fn best_move(&self) -> PlyType {
        let best_statistic =
            self.root.unwrap().children(&self.tree)
                .map(|c| &self.tree[c])
                .map(|c| &c.data)
                .max()
                .unwrap();
        best_statistic.game_state.ply_to_get_here.clone()
    }

    fn create_children(&mut self, node: NodeId) {
        let children = self.tree[node].data.game_state.children();

        for child in children {
            let new_node = self.tree.new_node(Statistic::new(child));
            node.append(new_node, &mut self.tree);
        }
    }

    fn random_placement(&self) -> PlyType {
        let children: Vec<&GameState>
            = self.root.unwrap().children(&self.tree)
                        .map(|c| &self.tree[c])
                        .map(|n| &n.data)
                        .map(|s| &s.game_state)
                        .collect();

        thread_rng().choose(&children).unwrap().ply_to_get_here.clone()
    }
}

impl InputHandler for Monty {
    fn update_game_state(&mut self, game_state: GameState) {
        let mut tree = Arena::new();
        let statistic = Statistic::new(game_state);
        let root = tree.new_node(statistic);
        self.tree = tree;
        self.root = Some(root);
        self.create_children(root);
    }

    fn get_placement(&mut self, available_places: Vec<String>) -> String {
        //let chosen = self.random_placement(available_places);
        let chosen = self.mcts();

        match chosen {
            Placement {piece_id, ..} => {
                assert!(available_places.contains(&piece_id),
                    format!("available_places: {:?}, piece_id: {}", available_places, piece_id));
                piece_id
            },
            _ => panic!("Moved from a placement node using {:?}", chosen),
        }
    }

    fn get_move(&mut self, available_moves: Vec<(String, String)>) -> (String, String) {
        thread_rng().choose(&available_moves).unwrap().to_owned()
    }

    fn get_mill(&mut self, available_mills: Vec<String>) -> String {
        thread_rng().choose(&available_mills).unwrap().to_string()
    }

    fn to_string(&self) -> String {
        format!("Monty InputHandler: {:?}", self.tree)
    }
}
