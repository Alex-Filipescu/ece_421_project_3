use std::fmt;
use crate::game_logic::connect_four::Player::PlayerOne;

#[derive (Clone)]
pub struct GridLocation{
    owner: Player,
    row: usize,
    col: usize,
    traversed:bool,
}

#[derive(Debug)]
pub enum Message {
    Winner(Player),
    ColumnFull,
    NextPlayer(Player),
    OutOfBounds
}

pub struct BoardState {
    cols: Vec<Vec<GridLocation>>,
    max_cols: usize,
    max_rows: usize
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Player{
    None,
    PlayerOne,
    PlayerTwo
}

pub struct GameState {
    board: BoardState,
    next_player: Player,
    winner: Player
}

impl Player {
    fn is_player(&self) -> bool{
        return match &*self {
            Player::None => {false}
            PlayerOne => {true}
            Player::PlayerTwo => {true}
        }
    }
}

impl BoardState {
    fn new(rows: usize, cols: usize) -> Self{
        let mut col_vec = Vec::with_capacity(cols-1);
        for col in 0..cols{
            let mut row_vec = Vec::with_capacity(cols-1);
            for row in 0..rows{
                row_vec.push(GridLocation{owner:Player::None, traversed: false, row, col})
            }
            col_vec.push(row_vec);
        }
        BoardState{cols: col_vec, max_cols: cols, max_rows: rows}
    }

    fn play_move(&mut self, player: Player, column: usize) -> Option<GridLocation>{
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
            }
        }
        if col > 0 {
            if self.cols[col-1][row].owner == player{
                current_chain += 1;
            }
        }
        if col < self.max_cols-2 {
            if self.cols[col+2][row].owner == player{
                current_chain += 1;
            }
        }
        if col > 1 {
            if self.cols[col-2][row].owner == player{
                current_chain += 1;
            }
        }
        if col < self.max_cols-3 {
            if self.cols[col+3][row].owner == player{
                current_chain += 1;
            }
        }
        if col > 2 {
            if self.cols[col-3][row].owner == player{
                current_chain += 1;
            }
        }
        return current_chain > 3;
    }

    fn check_downward_diagonal(&mut self, player:Player, col: usize, row: usize, mut current_chain: i8) -> bool{
        if col < self.max_cols-1 && row > 0{
            if self.cols[col+1][row-1].owner == player{
                current_chain += 1;
            }
        }
        if col > 0 && row < self.max_rows-1{
            if self.cols[col-1][row+1].owner == player{
                current_chain += 1;
            }
        }
        if col < self.max_cols-2 && row > 1 {
            if self.cols[col+2][row-2].owner == player{
                current_chain += 1;
            }
        }
        if col > 1 && row < self.max_rows-2{
            if self.cols[col-2][row+2].owner == player{
                current_chain += 1;
            }
        }
        if col < self.max_cols-3 && row > 2 {
            if self.cols[col+3][row-3].owner == player{
                current_chain += 1;
            }
        }
        if col > 2 && row < self.max_rows-3{
            if self.cols[col-3][row+3].owner == player{
                current_chain += 1;
            }
        }
        return current_chain > 3;
    }

    fn check_up_down(&mut self, player:Player, col: usize, row: usize, mut current_chain: i8) -> bool{
        if row > 0{
            if self.cols[col][row-1].owner == player{
                current_chain += 1;
            }
        }
        if row < self.max_rows-1{
            if self.cols[col][row+1].owner == player{
                current_chain += 1;
            }
        }
        if row > 1 {
            if self.cols[col][row-2].owner == player{
                current_chain += 1;
            }
        }
        if row < self.max_rows-2{
            if self.cols[col][row+2].owner == player{
                current_chain += 1;
            }
        }
        if row > 2 {
            if self.cols[col][row-3].owner == player{
                current_chain += 1;
            }
        }
        if row < self.max_rows-3{
            if self.cols[col][row+3].owner == player{
                current_chain += 1;
            }
        }
        return current_chain > 3;
    }

    fn check_upward_diagonal(&mut self, player:Player, col: usize, row: usize, mut current_chain: i8) -> bool{
        if col < self.max_cols-1 && row < self.max_rows-1{
            if self.cols[col+1][row+1].owner == player{
                current_chain += 1;
            }
        }
        if col > 0 && row > 0{
            if self.cols[col-1][row-1].owner == player{
                current_chain += 1;
            }
        }
        if col < self.max_cols-2 && row < self.max_rows-2 {
            if self.cols[col+2][row+2].owner == player{
                current_chain += 1;
            }
        }
        if col > 1 && row > 1 {
            if self.cols[col-2][row-2].owner == player{
                current_chain += 1;
            }
        }
        if col < self.max_cols-3 && row < self.max_rows-3 {
            if self.cols[col+3][row+3].owner == player{
                current_chain += 1;
            }
        }
        if col > 2 && row > 2 {
            if self.cols[col-3][row-3].owner == player{
                current_chain += 1;
            }
        }
        return current_chain > 3;
    }

}

impl GameState {
    pub fn init(rows: usize, cols: usize) -> Self{
        GameState{next_player: PlayerOne, board: BoardState::new(rows, cols), winner:Player::None}
    }

    pub fn play_move(&mut self, column: usize) -> Message {
        // The GUI should not allow the user to do this, but this is defensive programming
        if column >= self.board.max_cols {
            return Message::OutOfBounds;
        }
        let current_player = &self.get_next_player();
        let mut play_location = &self.board.play_move(current_player.clone(), column);
        if play_location.is_none(){
            return Message::ColumnFull;
        }
        if self.board.check_winner(current_player.clone(), &mut play_location.clone().unwrap()){
            return Message::Winner(current_player.clone())
        }
        return Message::NextPlayer(self.next_player.clone());

    }

    fn get_next_player(&mut self) -> Player {
        match &self.next_player {
            Player::PlayerOne => {
                self.next_player = Player::PlayerTwo;
                return Player::PlayerOne},
            Player::PlayerTwo => {
                self.next_player = PlayerOne;
                return Player::PlayerTwo},
            _ => {panic!("Next player is none")}
        }
    }

}

impl fmt::Debug for GameState {
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