use crate::game_logic::game_info::TwoPlayer;
use crate::game_logic::toot_otto::TootOtto;
use crate::game_logic::game_info::Message;
use crate::game_logic::game_info::Player;

use rand::prelude::*;

pub struct TootOttoBot {
    game: TootOtto,
    num_sims: i32,
}

impl TootOttoBot {
    pub fn new(game: TootOtto, num_sims: i32) -> Self {
        return TootOttoBot{ game, num_sims };
    }

    pub fn select_move(&mut self) -> (usize, char) {
        /*
        Selects the best move in a priority order:
            1) Win if possible (no point blocking if win is possible)
            2) Block opponent if needed
            3) Choose best move by simulation
        */

        for letter in ['t', 'o'] {
            let p2 = self.check_win(Player::PlayerTwo, letter);
            let p1 = self.check_win(Player::PlayerOne, letter);

            if p2.0 < self.game.board.max_cols {
                return p2;
            } 
            else if p1.0 < self.game.board.max_cols {
                let l = if letter=='o' { 't' } else { 'o' };
                return (p1.0, l);
            }
        }

        let t_best = self.check_best_move('t');
        let o_best = self.check_best_move('o');
        if t_best.0 > o_best.0 {
            return (t_best.1, 't');
        } else {
            return (o_best.1, 'o');
        }
    }

    fn check_win(&mut self, simulate_player: Player, letter: char) -> (usize, char) {
        // Iterate every position and check for win condition of player.
        for column in 0..self.game.board.max_cols {
            let mut game_clone = self.game.clone();

            // If simulating player 1, switch to player 1 
            if simulate_player == Player::PlayerOne {
                game_clone.cycle_next_player();
            }

            let result = game_clone.play_move(column, letter);
            if result == Message::Winner(simulate_player) {
                return (column, letter);
            }
        }
        // no one will win next turn, return inpossible column
        return (self.game.board.max_cols + 1, 'x');
    }

    fn check_best_move(&mut self, letter: char) -> (i64, usize) {
        let scores = self.simulate(letter);
        let mut columns_sorted: Vec<usize> = (0..scores.len()).collect();
        columns_sorted.sort_by_key(|&i| std::cmp::Reverse(scores[i]));
        let mut ind = 0;
    
        while ind < columns_sorted.len() {
            let best_column = columns_sorted[ind];

            // captures case where placing the "best move" would give the opponent a win
            let mut is_bad_move = false;
    
            for column in 0..self.game.board.max_cols {
                let mut game_clone = self.game.clone();
    
                game_clone.play_move(best_column, letter);
    
                let result = game_clone.play_move(column, letter);
                if result == Message::Winner(Player::PlayerOne) {
                    is_bad_move = true;
                    break;
                }
            }
    
            if is_bad_move == false {
                return (scores[ind], best_column);
            } else {
                ind += 1;
            }
        }
        return (scores[0], columns_sorted[0]);
    }

    fn score_game(&self, result: Message) -> i64 {
        match result {
            Message::Winner(Player::PlayerOne) => return -2,
            Message::Winner(Player::PlayerTwo) => return 1, 
            Message::Tie => return -1,
            _ => return 0
        }
    }
  
    fn simulate(&mut self, letter: char) -> Vec<i64> {
        let mut scores: Vec<i64> = vec![0; self.game.board.max_cols];

        for _ in 0..self.num_sims {
            for column in 0..self.game.board.max_cols {
                // dummy message to initialize the variable
                let mut result: Message = Message::NextPlayer(Player::PlayerTwo);
                let mut game_clone = self.game.clone();

                // based on Monte Carlo tree search: try a move and then play randomly to completion and repeat for every possible next move
                result = game_clone.play_move(column, letter);

                // Give the highest penalty to a filled column so it's never chosen.
                if result == Message::ColumnFull {
                    scores[column] = -9223372036854775808;  
                    continue;
                }

                loop {
                    let mut rng = thread_rng();
                    if rng.gen() {
                        let rand_column: usize = rng.gen_range(0..(self.game.board.max_cols));
                        let rand_letter: char = if rng.gen_range(0..2)==0 {'t'} else { 'o' };
                        result = game_clone.play_move(rand_column, rand_letter);
                    }

                    if self.score_game(result) != 0 {
                        scores[column] += self.score_game(result);
                        break;
                    }
                }
            }
        }
        return scores;
    }
}