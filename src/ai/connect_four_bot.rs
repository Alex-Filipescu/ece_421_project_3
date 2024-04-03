use crate::game_logic::connect_four::ConnectFour;
use crate::game_logic::game_info::Message;
use crate::game_logic::game_info::Player;
use crate::game_logic::game_info::TwoPlayer;

use rand::prelude::*;

pub struct ConnectFourBot {
    game: ConnectFour,
    num_sims: i32,
    player: Player,
}

impl ConnectFourBot {
    pub fn new(game: ConnectFour, num_sims: i32, player: Player) -> Self {
        return ConnectFourBot{ game, num_sims, player };
    }

    fn get_opponent(&self) -> Player {
        match self.player {
            Player::PlayerOne => return Player::PlayerTwo,
            Player::PlayerTwo => return Player::PlayerOne,
            _ => panic!("PLAYER UNDEFINED SHOULD NEVER HAPPEN!!!")
        }
    }

    pub fn select_move(&mut self) -> usize {
        /*
        Selects the best move in a priority order:
            1) Win if possible (no point blocking if win is possible)
            2) Block opponent if needed
            3) Choose best move by simulation
        */
        if self.check_win(self.player) < self.game.board.max_cols {
            return self.check_win(self.player);
        } 
        else if self.check_win(self.get_opponent()) < self.game.board.max_cols {
            return self.check_win(self.get_opponent());
        }
        else {
            return self.check_best_move();
        }
    }

    fn check_win(&mut self, simulate_player: Player) -> usize {
        // Iterate every position and check for win condition of player.
        for column in 0..self.game.board.max_cols {
            let mut game_clone = self.game.clone();

            // If simulating opponent, switch to them
            if simulate_player == self.get_opponent() {
                game_clone.cycle_next_player();
            }

            let result = game_clone.play_move(column);
            if result == Message::Winner(simulate_player) {
                return column;
            }
        }
        // no one will win next turn, return inpossible column
        return self.game.board.max_cols + 1;
    }

    fn check_best_move(&mut self) -> usize {
        let scores = self.simulate();
        let mut columns_sorted: Vec<usize> = (0..scores.len()).collect();
        columns_sorted.sort_by_key(|&i| std::cmp::Reverse(scores[i]));
        let mut ind = 0;
    
        while ind < columns_sorted.len() {
            let best_column = columns_sorted[ind];

            /*
            captures case where placing the "best move" would give the opponent a win
            ex. Placing an O at the question mark would help the opponent (X) win
                |   |   |   |   |   |   |   |
                |   |   |   |   |   |   |   |
                |   |   | O |   |   |   |   |
                |   |   | X | ? |   |   |   |
                |   | X | X | O |   |   | O |
                | X | X | X | O |   |   | O |
            */

            let mut is_bad_move = false;
    
            for column in 0..self.game.board.max_cols {
                let mut game_clone = self.game.clone();
    
                game_clone.play_move(best_column);
    
                let result = game_clone.play_move(column);
                if result == Message::Winner(self.get_opponent()) || result == Message::Tie || result == Message::ColumnFull {
                    is_bad_move = true;
                    break;
                }
            }
    
            if is_bad_move == false {
                return best_column;
            } else {
                ind += 1;
            }
        }
        return columns_sorted[0];
    }

    fn score_game(&self, result: Message) -> i64 {
        match result {
            Message::Winner(player) if player == self.get_opponent() => -2,
            Message::Winner(player) if player == self.player => 1, 
            Message::Tie => -1,
            _ => 0
        }
    }
  
    fn simulate(&mut self) -> Vec<i64> {
        let mut scores: Vec<i64> = vec![0; self.game.board.max_cols];

        for _ in 0..self.num_sims {
            for column in 0..self.game.board.max_cols {
                let mut result: Message = Message::NextPlayer(self.player);  // dummy message to initialize the variable
                let mut game_clone = self.game.clone();

                // based on Monte Carlo tree search: try a move and then play randomly to completion and repeat for every possible next move
                result = game_clone.play_move(column);

                // Give the highest penalty to a filled column so it's never chosen.
                if result == Message::ColumnFull {
                    scores[column] = -9223372036854775808;  
                    continue;
                }

                loop {
                    let mut rng = thread_rng();
                    if rng.gen() {
                        let rand_col: usize = rng.gen_range(0..(self.game.board.max_cols));
                        result = game_clone.play_move(rand_col);
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