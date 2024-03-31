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
        let scores = self.simulate();
        let best_score_column = scores
            .iter()
            .enumerate()
            .max_by_key(|(_, &value)| value)
            .map(|(index, _)| index)
            .unwrap();

        let mut best_move = (self.game.board.max_cols, 'x');
        let mut token = 'x';
        if best_score_column < self.game.board.max_cols {
            token = 't';
            best_move = (best_score_column, token);
        } else {
            token = 'o';
            best_move = (best_score_column-self.game.board.max_cols, token);
        }

        if self.check_win(Player::PlayerTwo, best_move).0 < self.game.board.max_cols {
            return self.check_win(Player::PlayerTwo, best_move);
        } 
        else if self.check_win(Player::PlayerOne, best_move).0 < self.game.board.max_cols {
            return self.check_win(Player::PlayerOne, best_move);
        }
        else {
            return best_move;
        }
    }

    fn check_win(&mut self, player: Player, bot_move: (usize, char)) -> (usize, char) {
        // Iterate every position and check for win condition of player.
        for token in ['t', 'o'] {
            for column in 0..self.game.board.max_cols {
                let mut game_clone = self.game.clone();

                // player 2 makes best move and checks if it leads to player 1 wins
                if player == Player::PlayerOne {
                    game_clone.play_move(bot_move.0, bot_move.1);
                }

                let result = game_clone.play_move(column, token);
                if result == Message::Winner(player) {
                    if player == Player::PlayerOne {
                        match token {
                            't' => return (column, 'o'),
                            'o' => return (column, 't'),
                            _ => {}
                        }
                    }
                    return (column, token);
                }
            }
        }
        // no one will win next turn, return inpossible column and token
        return (self.game.board.max_cols + 1, 'x');
    }
  
    fn simulate(&mut self) -> Vec<i64> {
        // let mut scores: Vec<i64> = vec![0; self.game.board.max_cols];
        let mut scores: Vec<i64> = vec![0; self.game.board.max_cols*2];

        for token in ['t', 'o'] {
            for _ in 0..self.num_sims {
                for column in 0..self.game.board.max_cols {
                    // dummy message to initialize the variable
                    let mut result: Message = Message::NextPlayer(Player::PlayerTwo);
                    let mut game_clone = self.game.clone();

                    // based on Monte Carlo tree search: try a move and then play randomly to completion and repeat for every possible next move
                    result = game_clone.play_move(column, token);

                    // Give the highest penalty to a filled column so it's never chosen.
                    if result == Message::ColumnFull {
                        scores[column] = -9223372036854775808;  
                        continue;
                    }

                    loop {
                        let mut rng = thread_rng();
                        if rng.gen() {
                            let bot_move: usize = rng.gen_range(0..(self.game.board.max_cols));
                            let bot_token: usize = rng.gen_range(0..2);
                            if bot_token == 0 {
                                result = game_clone.play_move(bot_move, 't');
                            } else {
                                result = game_clone.play_move(bot_move, 'o');
                            }
                        }

                        let mut ind = column;
                        if token == 'o' {
                            ind += self.game.board.max_cols;
                        }
                        match result {
                            Message::Winner(Player::PlayerOne) => {
                                scores[ind] -= 2;
                                break;
                            }
                            Message::Winner(Player::PlayerTwo) => {
                                scores[ind] += 1;
                                break;
                            }
                            Message::Tie => {
                                scores[ind] -= 1;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        return scores;
    }
}