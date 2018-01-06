use nineman::game::*;

#[derive(Debug)]
pub struct Statistic {
    times_visited: i8,
    sum_of_payoffs_received: i8,
    pub game_state: GameState,

}

impl Statistic {
    pub fn new(game_state: GameState) -> Self {
        Statistic {
            times_visited: 0,
            sum_of_payoffs_received: 0,
            game_state: game_state,
        }
    }
}
