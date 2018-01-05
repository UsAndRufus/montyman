use nineman::board::Board;
use nineman::player;

pub struct GameState {
    pub board: Board,
    pub current_player: i8,
    pub player1_score: i8,
    pub player2_score: i8,
    pub player1_pieces_to_place: i8,
    pub player2_pieces_to_place: i8,
}

impl GameState {
    pub fn at_start(board: &Board) -> Self {
        GameState {
            board: board.clone(),
            current_player: 1,
            player1_score: player::STARTING_SCORE,
            player2_score: player::STARTING_SCORE,
            player1_pieces_to_place: player::STARTING_PIECES,
            player2_pieces_to_place: player::STARTING_PIECES,
        }
    }

    pub fn winner(&self) -> Option<i8> {
        if self.player1_score >=  player::WIN_SCORE {
            return Some(1);
        }

        if self.player2_score >=  player::WIN_SCORE {
            return Some(2);
        }

        None
    }
}
