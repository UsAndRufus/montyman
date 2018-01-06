use nineman::game::*;

// 1/root(2)
const UCT_CONST: f32 = 0.70710678118654752;

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

    pub fn visit(&mut self, payoff: i8) {
        self.sum_of_payoffs_received += payoff;
        self.times_visited += 1;
    }

    pub fn uct(&self, child: &Statistic) -> f32 {
        let average_payoff = child.sum_of_payoffs_received_f() /
                             child.times_visited_f();

        let ln_term = 2.0 * self.times_visited_f().ln() /
                      child.times_visited_f();

        let root_term = UCT_CONST * ln_term.sqrt();

        average_payoff + root_term

    }

    pub fn times_visited_f(&self) -> f32 {
        self.times_visited as f32
    }

    pub fn sum_of_payoffs_received_f(&self) -> f32 {
        self.sum_of_payoffs_received as f32
    }
}
