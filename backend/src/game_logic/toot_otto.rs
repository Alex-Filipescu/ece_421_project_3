use std::fmt;
use crate::game_logic::game_info::{GameState, Message, TwoPlayer};
use crate::game_logic::game_info::Player;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GridLocation{
    letter: Option<char>,
    row: usize,
    col: usize,
    traversed:bool,
}

#[derive (Clone)]
pub struct BoardState {
    pub cols: Vec<Vec<GridLocation>>,
    pub max_cols: usize,
    pub max_rows: usize
}

#[derive (Clone)]
pub struct TootOtto {
    pub board: BoardState,
    next_player: Player,
    winner: Player
}

impl BoardState {
    fn new(rows: usize, cols: usize) -> Self{
        let mut col_vec = Vec::with_capacity(cols-1);
        for col in 0..cols{
            let mut row_vec = Vec::with_capacity(cols-1);
            for row in 0..rows{
                row_vec.push(GridLocation {letter: None, traversed: false, row, col})
            }
            col_vec.push(row_vec);
        }
        BoardState {cols: col_vec, max_cols: cols, max_rows: rows}
    }

    fn play_move(&mut self, column: usize, letter: char,) -> Option<GridLocation>{
        for row in &mut self.cols[column] {
            if row.letter.is_none() {
                row.letter = Some(letter);
                return Some(row.clone());
            }
        }
        return None;
    }

    fn check_winner(&mut self, mut location: &mut GridLocation) -> Message{
        let mut char_vec: Vec<char>;
        let mut token_string: String;
        let mut toot_found= false;
        let mut otto_found = false;
        char_vec = self.check_left_right(location.col, location.row);
        token_string = char_vec.into_iter().collect();
        // println!("{}", token_string);

        if token_string.contains("TOOT") && token_string.contains("OTTO"){
            return Message::Tie;
        } else if token_string.contains("TOOT") {
            toot_found = true;
        } else if token_string.contains("OTTO") {
            otto_found = true;
        }

        char_vec = self.check_downward_diagonal(location.col, location.row);
        token_string = char_vec.into_iter().collect();

        if token_string.contains("TOOT") && token_string.contains("OTTO"){
            return Message::Tie;
        } else if token_string.contains("TOOT") {
            toot_found = true;
        } else if token_string.contains("OTTO") {
            otto_found = true;
        }

        char_vec = self.check_up_down(location.col, location.row);
        token_string = char_vec.into_iter().collect();

        if token_string.contains("TOOT") && token_string.contains("OTTO"){
            return Message::Tie;
        } else if token_string.contains("TOOT") {
            toot_found = true;
        } else if token_string.contains("OTTO") {
            otto_found = true;
        }

        char_vec = self.check_upward_diagonal(location.col, location.row);
        token_string = char_vec.into_iter().collect();

        if token_string.contains("TOOT") && token_string.contains("OTTO"){
            return Message::Tie;
        } else if token_string.contains("TOOT") {
            toot_found = true;
        } else if token_string.contains("OTTO") {
            otto_found = true;
        }

        if toot_found && otto_found {
            return Message::Tie
        } else if toot_found {
            Message::Winner(Player::PlayerOne)

        } else if otto_found {
            Message::Winner(Player::PlayerTwo)
        } else {
            Message::NoWinner
        }
    }

    fn check_left_right(&mut self, col: usize, row: usize) -> Vec<char>{
        let mut result = Vec::new();
        if col < self.max_cols-1 {
            if self.cols[col+1][row].letter.is_some(){
                result.push(self.cols[col+1][row].letter.unwrap());
                if col < self.max_cols-2 {
                    if self.cols[col+2][row].letter.is_some(){
                        result.push(self.cols[col+2][row].letter.unwrap());
                        if col < self.max_cols-3 {
                            if self.cols[col+3][row].letter.is_some(){
                                result.push(self.cols[col+3][row].letter.unwrap());
                            }
                        }
                    }
                }
            }
        }
        result.reverse();
        result.push(self.cols[col][row].letter.unwrap());
        if col > 0 {
            if self.cols[col-1][row].letter.is_some(){
                result.push(self.cols[col-1][row].letter.unwrap());
                if col > 1 {
                    if self.cols[col-2][row].letter.is_some(){
                        result.push(self.cols[col-2][row].letter.unwrap());                        if col > 2 {
                            if self.cols[col-3][row].letter.is_some(){
                                result.push(self.cols[col-3][row].letter.unwrap());                            }
                        }
                    }
                }
            }
        }
        return result;
    }

    fn check_downward_diagonal(&self, col: usize, row: usize) -> Vec<char>{
        let mut result = Vec::new();

        if col < self.max_cols-1 && row > 0{
            if self.cols[col+1][row-1].letter.is_some(){
                result.push(self.cols[col+1][row-1].letter.unwrap());
                if col < self.max_cols-2 && row > 1 {
                    if self.cols[col+2][row-2].letter.is_some(){
                        result.push(self.cols[col+2][row-2].letter.unwrap());
                        if col < self.max_cols-3 && row > 2 {
                            if self.cols[col+3][row-3].letter.is_some(){
                                result.push(self.cols[col+3][row-3].letter.unwrap());
                            }
                        }
                    }
                }
            }
        }
        result.reverse();
        result.push(self.cols[col][row].letter.unwrap());

        if col > 0 && row < self.max_rows-1{
            if self.cols[col-1][row+1].letter.is_some(){
                result.push(self.cols[col-1][row+1].letter.unwrap());
                if col > 1 && row < self.max_rows-2{
                    if self.cols[col-2][row+2].letter.is_some(){
                        result.push(self.cols[col-2][row+2].letter.unwrap());
                        if col > 2 && row < self.max_rows-3{
                            if self.cols[col-3][row+3].letter.is_some(){
                                result.push(self.cols[col-3][row+3].letter.unwrap());
                            }
                        }
                    }
                }
            }
        }
        return result;
    }

    fn check_up_down(&mut self, col: usize, row: usize) -> Vec<char>{
        let mut result = Vec::new();

        if row > 0{
            if self.cols[col][row-1].letter.is_some(){
                result.push(self.cols[col][row-1].letter.unwrap());
                if row > 1 {
                    if self.cols[col][row-2].letter.is_some(){
                        result.push(self.cols[col][row-2].letter.unwrap());
                        if row > 2 {
                            if self.cols[col][row-3].letter.is_some(){
                                result.push(self.cols[col][row-3].letter.unwrap());
                            }
                        }
                    }
                }
            }
        }
        result.reverse();
        result.push(self.cols[col][row].letter.unwrap());

        if row < self.max_rows-1{
            if self.cols[col][row+1].letter.is_some(){
                result.push(self.cols[col][row+1].letter.unwrap());
                if row < self.max_rows-2{
                    if self.cols[col][row+2].letter.is_some(){
                        result.push(self.cols[col][row+2].letter.unwrap());
                        if row < self.max_rows-3{
                            if self.cols[col][row+3].letter.is_some(){
                                result.push(self.cols[col][row+3].letter.unwrap());
                            }
                        }
                    }
                }
            }
        }
        return result;
    }

    fn check_upward_diagonal(&mut self, col: usize, row: usize) -> Vec<char>{
        let mut result = Vec::new();

        if col < self.max_cols-1 && row < self.max_rows-1{
            if self.cols[col+1][row+1].letter.is_some(){
                result.push(self.cols[col+1][row+1].letter.unwrap());
                if col < self.max_cols-2 && row < self.max_rows-2 {
                    if self.cols[col+2][row+2].letter.is_some(){
                        result.push(self.cols[col+2][row+2].letter.unwrap());
                        if col < self.max_cols-3 && row < self.max_rows-3 {
                            if self.cols[col+3][row+3].letter.is_some(){
                                result.push(self.cols[col+3][row+3].letter.unwrap());
                            }
                        }
                    }
                }
            }
        }
        result.reverse();
        result.push(self.cols[col][row].letter.unwrap());

        if col > 0 && row > 0{
            if self.cols[col-1][row-1].letter.is_some(){
                result.push(self.cols[col-1][row-1].letter.unwrap());
                if col > 1 && row > 1 {
                    if self.cols[col-2][row-2].letter.is_some(){
                        result.push(self.cols[col-2][row-2].letter.unwrap());
                        if col > 2 && row > 2 {
                            if self.cols[col-3][row-3].letter.is_some(){
                                result.push(self.cols[col-3][row-3].letter.unwrap());
                            }
                        }
                    }
                }
            }
        }
        return result;
    }

    fn check_available_move(&mut self) -> bool{
        for col in &self.cols{
            if col[self.max_rows-1].letter.is_none(){
                return true;
            }
        }
        return false;
    }

}

impl GameState for TootOtto {
    fn init(rows: usize, cols: usize) -> Self {
        TootOtto {next_player: Player::PlayerOne, board: BoardState::new(rows, cols), winner: Player::None}
    }
}

impl TootOtto {
    pub fn play_move(&mut self, column: usize, letter: char) -> Message {
        // The GUI should not allow the user to do these, but this is defensive programming
        if letter != 'T' && letter != 'O' {
            return Message::InvalidCharacter;
        }
        if column >= self.board.max_cols {
            return Message::OutOfBounds;
        }
        if self.winner.is_player(){
            return Message::Winner(self.winner);
        }
        &self.cycle_next_player();
        let mut play_location = &self.board.play_move(column, letter);
        if play_location.is_none(){
            return Message::ColumnFull;
        }
        let result = self.board.check_winner(&mut play_location.clone().unwrap());
        if result == Message::Winner(Player::PlayerOne){
            self.winner = Player::PlayerOne;
        } else if result == Message::Winner(Player::PlayerTwo){
            self.winner = Player::PlayerTwo
        }
        if !&self.board.check_available_move() {
            return Message::Tie;
        }
        return if result == Message::NoWinner {
            Message::NextPlayer(self.next_player.clone())
        } else {
            result
        }

    }
}

impl TwoPlayer for TootOtto {
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

impl fmt::Debug for TootOtto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..self.board.max_rows).rev() {
            for col in 0..self.board.max_cols {
                match self.board.cols[col][row].letter {
                    Some('T') => write!(f, "| T ")?,
                    Some('O') => write!(f, "| O ")?,
                    _ => write!(f, "|   ")?,
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
    fn test_left_right_toot() {
        let play_vec: Vec<usize> = vec![0, 5, 1, 5, 2, 5, 3];
        let letter_vec: Vec<char> = vec!['T', 'O', 'O', 'T', 'O', 'T', 'T'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_left_right_otto() {
        let play_vec: Vec<usize> = vec![0, 5, 1, 5, 2, 5, 3];
        let letter_vec: Vec<char> = vec!['O', 'O', 'T', 'T', 'T', 'T', 'O'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerTwo), result);
    }

    #[test]
    fn test_upward_diagonal_toot() {
        let play_vec: Vec<usize> = vec![0, 1, 1, 2, 2, 5, 2, 3, 3, 5, 3, 5, 3];
        let letter_vec: Vec<char> = vec!['T', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'T'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_upward_diagonal_otto() {
        let play_vec: Vec<usize> = vec![0, 1, 1, 2, 2, 5, 2, 3, 3, 5, 3, 5, 3];
        let letter_vec: Vec<char> = vec!['O', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'O'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerTwo), result);
    }

    #[test]
    fn test_downward_diagonal_toot() {
        let play_vec: Vec<usize> = vec![0, 0, 0, 5, 0, 1, 1, 5, 1, 2, 2, 5, 3];
        let letter_vec: Vec<char> = vec!['O', 'O', 'O', 'O', 'T', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'T' ];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_downward_diagonal_otto() {
        let play_vec: Vec<usize> = vec![0, 0, 0, 5, 0, 1, 1, 5, 1, 2, 2, 5, 3];
        let letter_vec: Vec<char> = vec!['T', 'T', 'T', 'T', 'O', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'O' ];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerTwo), result);
    }

    #[test]
    fn test_up_down_toot() {
        let play_vec: Vec<usize> = vec![0, 0, 0, 0];
        let letter_vec: Vec<char> = vec!['T', 'O', 'O', 'T'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }

    #[test]
    fn test_up_down_otto() {
        let play_vec: Vec<usize> = vec![0, 0, 0, 0];
        let letter_vec: Vec<char> = vec!['O', 'T', 'T', 'O'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerTwo), result);
    }

    #[test]
    fn test_column_full() {
        let play_vec: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0];
        let letter_vec: Vec<char> = vec!['T', 'T', 'O', 'T', 'O', 'T', 'T'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::ColumnFull, result);
    }
    //
    #[test]
    fn test_out_of_bounds() {
        let play_vec: Vec<usize> = vec![9];
        let letter_vec: Vec<char> = vec!['T'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::OutOfBounds, result);
    }

    #[test]
    fn test_invalid_character() {
        let play_vec: Vec<usize> = vec![3];
        let letter_vec: Vec<char> = vec!['z'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::InvalidCharacter, result);
    }

    #[test]
    fn test_normal_game() {
        let play_vec: Vec<usize> = vec![3, 5, 2, 4, 0, 1];
        let letter_vec: Vec<char> = vec!['T', 'T', 'O', 'T', 'T', 'O']
            ;
        let mut game = TootOtto::init(7, 7);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Winner(Player::PlayerOne), result);
    }
    #[test]
    fn test_tie() {
        let play_vec: Vec<usize> = vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5];
        let letter_vec: Vec<char> = vec!['T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T', 'T'];
        let mut game = TootOtto::init(4, 6);
        let mut result = Message::NextPlayer(Player::PlayerOne);
        for i in 0..play_vec.len() {
            result = game.play_move(play_vec[i], letter_vec[i]);
        }
        println!("{:?}", game);
        assert_eq!(Message::Tie, result);
    }
}