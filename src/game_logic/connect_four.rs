use std::fmt;
use crate::game_logic::game_info::{GameState, Message, TwoPlayer};
use crate::game_logic::game_info::Player;

#[derive (Clone)]
pub struct GridLocation{
    owner: Player,
    row: usize,
    col: usize,
}

pub struct BoardState {
    pub cols: Vec<Vec<GridLocation>>,
    pub max_cols: usize,
    pub max_rows: usize
}

pub struct ConnectFour {
    board: BoardState,
    next_player: Player,
    winner: Player
}

impl BoardState {
    fn new(rows: usize, cols: usize) -> Self{
        let mut col_vec = Vec::with_capacity(cols-1);
        for col in 0..cols{
            let mut row_vec = Vec::with_capacity(cols-1);
            for row in 0..rows{
                row_vec.push(GridLocation{owner:Player::None, row, col})
            }
            col_vec.push(row_vec);
        }
        BoardState{cols: col_vec, max_cols: cols, max_rows: rows}
    }

    fn play_move(&mut self, column: usize, player: Player) -> Option<GridLocation>{
        for row in &mut self.cols[column] {
            if !row.owner.is_player() {
                row.owner = player;
                return Some(row.clone());
            }
        }
        return None;
    }

    fn check_winner(&mut self, player:Player, mut location: &mut GridLocation) -> bool{

        if self.check_left_right(player, location.col, location.row, 1){
            return true;
        }

        if self.check_downward_diagonal(player, location.col, location.row, 1){
            return true;
        }

        if self.check_up_down(player, location.col, location.row, 1){
            return true;
        }

        if self.check_upward_diagonal(player, location.col, location.row, 1){
            return true;
        }
        false
    }

    fn check_left_right(&mut self, player:Player, col: usize, row: usize, mut current_chain: i8) -> bool{
        if col < self.max_cols-1 {
            if self.cols[col+1][row].owner == player{
                current_chain += 1;
                if col < self.max_cols-2 {
                    if self.cols[col+2][row].owner == player{
                        current_chain += 1;
                        if col < self.max_cols-3 {
                            if self.cols[col+3][row].owner == player{
                                current_chain += 1;
                            }
                        }
                    }
                }
            }
        }
        if col > 0 {
            if self.cols[col-1][row].owner == player{
                current_chain += 1;
                if col > 1 {
                    if self.cols[col-2][row].owner == player{
                        current_chain += 1;
                        if col > 2 {
                            if self.cols[col-3][row].owner == player{
                                current_chain += 1;
                            }
                        }
                    }
                }
            }
        }
        return current_chain > 3;
    }

    fn check_downward_diagonal(&mut self, player:Player, col: usize, row: usize, mut current_chain: i8) -> bool{
        if col < self.max_cols-1 && row > 0{
            if self.cols[col+1][row-1].owner == player{
                current_chain += 1;
                if col < self.max_cols-2 && row > 1 {
                    if self.cols[col+2][row-2].owner == player{
                        current_chain += 1;
                        if col < self.max_cols-3 && row > 2 {
                            if self.cols[col+3][row-3].owner == player{
                                current_chain += 1;
                            }
                        }
                    }
                }
            }
        }
        if col > 0 && row < self.max_rows-1{
            if self.cols[col-1][row+1].owner == player{
                current_chain += 1;
                if col > 1 && row < self.max_rows-2{
                    if self.cols[col-2][row+2].owner == player{
                        current_chain += 1;
                        if col > 2 && row < self.max_rows-3{
                            if self.cols[col-3][row+3].owner == player{
                                current_chain += 1;
                            }
                        }
                    }
                }
            }
        }
        return current_chain > 3;
    }

    fn check_up_down(&mut self, player:Player, col: usize, row: usize, mut current_chain: i8) -> bool{
        if row > 0{
            if self.cols[col][row-1].owner == player{
                current_chain += 1;
                if row > 1 {
                    if self.cols[col][row-2].owner == player{
                        current_chain += 1;
                        if row > 2 {
                            if self.cols[col][row-3].owner == player{
                                current_chain += 1;
                            }
                        }
                    }
                }
            }
        }
        if row < self.max_rows-1{
            if self.cols[col][row+1].owner == player{
                current_chain += 1;
                if row < self.max_rows-2{
                    if self.cols[col][row+2].owner == player{
                        current_chain += 1;
                        if row < self.max_rows-3{
                            if self.cols[col][row+3].owner == player{
                                current_chain += 1;
                            }
                        }
                    }
                }
            }
        }
        return current_chain > 3;
    }

    fn check_upward_diagonal(&mut self, player:Player, col: usize, row: usize, mut current_chain: i8) -> bool{
        if col < self.max_cols-1 && row < self.max_rows-1{
            if self.cols[col+1][row+1].owner == player{
                current_chain += 1;
                if col < self.max_cols-2 && row < self.max_rows-2 {
                    if self.cols[col+2][row+2].owner == player{
                        current_chain += 1;
                        if col < self.max_cols-3 && row < self.max_rows-3 {
                            if self.cols[col+3][row+3].owner == player{
                                current_chain += 1;
                            }
                        }
                    }
                }
            }
        }
        if col > 0 && row > 0{
            if self.cols[col-1][row-1].owner == player{
                current_chain += 1;
                if col > 1 && row > 1 {
                    if self.cols[col-2][row-2].owner == player{
                        current_chain += 1;
                        if col > 2 && row > 2 {
                            if self.cols[col-3][row-3].owner == player{
                                current_chain += 1;
                            }
                        }
                    }
                }
            }
        }
        return current_chain > 3;
    }

}

impl ConnectFour {
    pub fn play_move(&mut self, column: usize) -> Message {
        // The GUI should not allow the user to do these, but this is defensive programming
        if column >= self.board.max_cols {
            return Message::OutOfBounds;
        }
        if self.winner.is_player(){
            return Message::Winner(self.winner);
        }
        let current_player = &self.cycle_next_player();
        let mut play_location = &self.board.play_move(column, current_player.clone());
        if play_location.is_none(){
            return Message::ColumnFull;
        }
        if self.board.check_winner(current_player.clone(), &mut play_location.clone().unwrap()){
            self.winner = current_player.clone();
            return Message::Winner(current_player.clone())
        }
        return Message::NextPlayer(self.next_player.clone());

    }
}

impl GameState for ConnectFour {
    fn init(rows: usize, cols: usize) -> Self{
        ConnectFour {next_player: Player::PlayerOne, board: BoardState::new(rows, cols), winner:Player::None}
    }

}

impl TwoPlayer for ConnectFour {
    fn cycle_next_player(&mut self) -> Player {
        match &self.next_player {
            Player::PlayerOne => {
                self.next_player = Player::PlayerTwo;
                return Player::PlayerOne},
            Player::PlayerTwo => {
                self.next_player = Player::PlayerOne;
                return Player::PlayerTwo},
            _ => {panic!("Player is none")}
        }
    }
}

impl fmt::Debug for ConnectFour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..self.board.max_rows).rev() {
            for col in 0..self.board.max_cols {
                match self.board.cols[col][row].owner {
                    Player::PlayerOne => write!(f, "| X ")?,
                    Player::PlayerTwo => write!(f, "| O ")?,
                    Player::None => write!(f, "|   ")?,
                }
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}


// TODO: Test cases