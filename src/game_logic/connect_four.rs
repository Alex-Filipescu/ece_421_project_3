use std::fmt;
use crate::game_logic::game_info::{GameState, Message, TwoPlayer};
use crate::game_logic::game_info::Player;

#[derive (Clone, Debug)]
pub struct GridLocation{
    owner: Player,
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct BoardState {
    pub cols: Vec<Vec<GridLocation>>,
    pub max_cols: usize,
    pub max_rows: usize
}

#[derive(Debug)]
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

    fn check_available_move(&mut self) -> bool{
        for col in &self.cols{
            if col[self.max_rows-1].owner == Player::None{
                return true;
            }
        }
        return false;
    }
}

impl ConnectFour {
    pub fn init(rows: usize, cols: usize) -> Self{
        ConnectFour {next_player: Player::PlayerOne, board: BoardState::new(rows, cols), winner:Player::None}
    }

    pub fn play_move(&mut self, column: usize) -> Message {
        // The GUI should not allow the user to do these, but this is defensive programming
        if column >= self.board.max_cols {
            return Message::OutOfBounds;
        }
        if self.winner.is_player(){
            return Message::Winner(self.winner);
        }
        if !&self.board.check_available_move() {
            return Message::Tie;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_right() {
        let play_vec: Vec<usize> = vec![0, 5, 1, 5, 2, 5, 3];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_upward_diagonal() {
        let play_vec: Vec<usize> = vec![0, 1, 1, 2, 2, 5, 2, 3, 3, 5, 3, 5, 3];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_downward_diagonal() {
        let play_vec: Vec<usize> = vec![0, 0, 0, 5, 0, 1, 1, 5, 1, 2, 2, 5, 3];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_up_down() {
        let play_vec: Vec<usize> = vec![0, 1, 0, 1, 0, 1, 0];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_normal_game_p2_win(){
        let play_vec: Vec<usize> = vec![0, 6, 5, 5, 4, 3, 4, 4, 3, 5, 5, 4, 2, 3, 6, 1, 2, 2];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerTwo), result);
    }

    #[test]
    fn test_normal_game_p1_win(){
        let play_vec: Vec<usize> = vec![2, 6, 3, 6, 6, 5, 2, 5, 5, 0, 1, 1, 4];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_tie_game(){
        // TODO: Input sequence
        let play_vec: Vec<usize> = vec![0, 0];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        // assert_eq!(Message::Tie, result);
    }

    #[test]
    fn test_column_full(){
        let play_vec: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::ColumnFull, result);
    }

    #[test]
    fn test_out_of_bounds(){
        let play_vec: Vec<usize> = vec![9];
        let mut game = ConnectFour::init(6, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::OutOfBounds, result);
    }

}