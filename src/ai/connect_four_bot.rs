use crate::game_logic::connect_four::ConnectFour;
use crate::game_logic::game_info::Message;
use crate::game_logic::game_info::Player;
use crate::game_logic::game_info::TwoPlayer;

use rand::prelude::*;

pub struct ConnectFourBot {
    game: ConnectFour,
    num_sims: i32,
}

impl ConnectFourBot {
    pub fn new(game: ConnectFour, num_sims: i32) -> Self {
        return ConnectFourBot{ game, num_sims };
    }

    pub fn select_move(&mut self) -> usize {
        /*
        Selects the best move in a priority order:
            1) Win if possible (no point blocking if win is possible)
            2) Block opponent if needed
            3) Choose best move by simulation
        */
        if self.check_win(Player::PlayerTwo) < self.game.board.max_cols {
            return self.check_win(Player::PlayerTwo);
        } 
        else if self.check_win(Player::PlayerOne) < self.game.board.max_cols {
            return self.check_win(Player::PlayerOne);
        }
        else {
            return self.check_best_move();
        }
    }

    fn check_win(&mut self, simulate_player: Player) -> usize {
        // Iterate every position and check for win condition of player.
        for column in 0..self.game.board.max_cols {
            let mut game_clone = self.game.clone();

            // If simulating player 1, switch to player 1 
            if simulate_player == Player::PlayerOne {
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
                if result == Message::Winner(Player::PlayerOne) || result == Message::Tie || result == Message::ColumnFull {
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
            Message::Winner(Player::PlayerOne) => return -2,
            Message::Winner(Player::PlayerTwo) => return 1, 
            Message::Tie => return -1,
            _ => return 0
        }
    }
  
    fn simulate(&mut self) -> Vec<i64> {
        let mut scores: Vec<i64> = vec![0; self.game.board.max_cols];

        for _ in 0..self.num_sims {
            for column in 0..self.game.board.max_cols {
                let mut result: Message = Message::NextPlayer(Player::PlayerTwo);  // dummy message to initialize the variable
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