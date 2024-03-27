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
        let scores = self.simulate();
        let best_score_column = scores
            .iter()
            .enumerate()
            .max_by_key(|(_, &value)| value)
            .map(|(index, _)| index)
            .unwrap();
        
        if self.check_win(Player::PlayerTwo, best_score_column) < self.game.board.max_cols {
            return self.check_win(Player::PlayerTwo, best_score_column);
        } 
        else if self.check_win(Player::PlayerOne, best_score_column) < self.game.board.max_cols {
            return self.check_win(Player::PlayerOne, best_score_column);
        }
        else {
            return best_score_column;
        }
    }

    fn check_win(&mut self, player: Player, bot_move: usize) -> usize {
        // Iterate every position and check for win condition of player.
        for column in 0..self.game.board.max_cols {
            let mut game_clone = self.game.clone();

            // player 2 makes best move and checks if it leads to player 1 wins
            if player == Player::PlayerOne {
                game_clone.play_move(bot_move);
            }

            let result = game_clone.play_move(column);
            if result == Message::Winner(player) {
                return column;
            }
        }
        // no one will win next turn, return inpossible column
        return self.game.board.max_cols + 1;
    }
  
    fn simulate(&mut self) -> Vec<i64> {
        // let mut scores: Vec<i64> = vec![0; self.game.board.max_cols];
        let mut scores: Vec<i64> = vec![-4, -2, -2, -1, -2, -2, -4];

        for _ in 0..self.num_sims {
            for column in 0..self.game.board.max_cols {
                // dummy message to initialize the variable
                let mut result: Message = Message::NextPlayer(Player::PlayerTwo);
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
                        let bot_move: usize = rng.gen_range(0..(self.game.board.max_cols));
                        result = game_clone.play_move(bot_move);
                    }

                    match result {
                        Message::Winner(Player::PlayerOne) => {
                            scores[column] -= 2;
                            break;
                        }
                        Message::Winner(Player::PlayerTwo) => {
                            scores[column] += 1;
                            break;
                        }
                        Message::Tie => {
                            scores[column] -= 1;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
        return scores;
    }
}