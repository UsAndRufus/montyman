use rand::{thread_rng, Rng};
use indextree::*;
use std::collections::HashMap;

use nineman::game::*;
use nineman::game::Ply::*;
use nineman::player::InputHandler;

use statistic::Statistic;

const NUMBER_OF_SIMULATIONS: i16 = 100;

pub struct Monty {
    pub tree: Arena<Statistic>,
    pub root: Option<NodeId>,
    pub player_id: i8,
}

impl Monty {

    fn mcts(&mut self) -> Ply {
        println!("Monty is thinking...");
        for i in 1..NUMBER_OF_SIMULATIONS {
            let root = self.root.unwrap();
            let node_to_expand = self.select(root);
            let new_node = self.expand(node_to_expand);
            let payoff = self.simulate(new_node);
            self.update(new_node, payoff);

            if i % 10 == 0 {
                println!("{}",i);
            }
        }

        self.best_move()
    }

    // Recursively traverse from node, looking for most important expandable node
    // Return when you reach non-expandable node
    // NB: not convinced I have this right, it seems counter-intuitive.
    // Seems like you will never visit unvisited children of expanded nodes, i.e. you only ever look at leaf nodes?
    fn select(&mut self, node_id: NodeId) -> NodeId {
        if self.is_expandable(node_id) {
            self.expand(node_id)
        } else {
            let parent = &self.tree[node_id].data;
            let children: HashMap<_,_>
                = node_id.children(&self.tree)
                    .map(|c| (parent.uct(&self.tree[c].data), c))
                    .collect();

            let max = children.keys().max().unwrap();

            children.get(max).unwrap().clone()
        }
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
        let mut game_state = self.tree[node_id].data.game_state.clone();

        loop {
            assert!(!(game_state.ply_to_get_here.is_mill() && game_state.next_ply.is_mill()),
                        "Got here from a mill and next move is a mill: {:?}", game_state);

            //println!("{:?}", game_state);

            //let winner = game_state.winner();
            let winner = Some(1);

            match winner {
                Some(who) => {
                    if who == self.player_id {
                        break 1
                    } else {
                        break 0
                    }
                },
                None => {
                    // TODO: currently breaks here because we never have children beyond placement

                    let new_game_state = thread_rng().choose(&game_state.children()).cloned();


                    match new_game_state {
                        Some(new) => game_state = new,
                        None => break 0, // Cannot move so have lost the game
                    }
                }
            }
        }
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
    fn best_move(&self) -> Ply {
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

    fn random_placement(&self) -> Ply {
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
    fn give_new_game_state(&mut self, game_state: GameState) {
        let mut tree = Arena::new();
        let statistic = Statistic::new(game_state);
        let root = tree.new_node(statistic);
        self.tree = tree;
        self.root = Some(root);
        self.create_children(root);
    }

    fn get_placement(&mut self, available_places: Vec<Ply>) -> Ply {
        //let chosen = self.random_placement(available_places);
        let chosen = self.mcts();

        match chosen {
            Placement {..} => {
                assert!(available_places.contains(&chosen),
                    format!("Placement impossible: available_places: {:?}, chosen: {:?}", available_places, chosen));
                chosen
            },
            _ => panic!("Moved from a placement node using {:?}", chosen),
        }
    }

    fn get_move(&mut self, available_moves: Vec<Ply>) -> Ply {
        //thread_rng().choose(&available_moves).unwrap().to_owned()

        let chosen = self.mcts();

        match chosen {
            Move {..} => {
                assert!(available_moves.contains(&chosen),
                    format!("Move impossible: available_moves: {:?}, mv: {:?}", available_moves, chosen));
                chosen
            },
            _ => panic!("Moved from a move node using {:?}", chosen),
        }
    }

    fn get_mill(&mut self, available_mills: Vec<Ply>) -> Ply {
        //thread_rng().choose(&available_mills).unwrap().to_string()

        let chosen = self.mcts();

        match chosen {
            Mill {..} => {
                assert!(available_mills.contains(&chosen),
                    format!("Mill impossible: available_mills: {:?}, chosen: {:?}", available_mills, chosen));
                chosen
            },
            _ => panic!("Moved from a mill node using {:?}", chosen),
        }
    }

    fn to_string(&self) -> String {
        format!("Monty InputHandler: {:?}", self.tree)
    }

    fn set_player_id(&mut self, player_id: i8) {
        self.player_id = player_id;
    }
}
