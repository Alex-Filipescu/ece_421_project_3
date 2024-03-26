pub trait TwoPlayer {
    fn cycle_next_player(&mut self) -> Player;
}

pub trait GameState {
    fn init (rows: usize, cols: usize) -> Self;
    }
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Player{
    None,
    PlayerOne,
    PlayerTwo
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Message {
    Winner(Player),
    ColumnFull,
    NextPlayer(Player),
    OutOfBounds,
    NoWinner,
    Tie,
    InvalidCharacter
}

impl Player {
    pub fn is_player(&self) -> bool{
        return match self {
            Player::None => {false}
            Player::PlayerOne => {true}
            Player::PlayerTwo => {true}
        }
    }
}