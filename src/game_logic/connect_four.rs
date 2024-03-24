use crate::game_logic::connect_four::Player::PlayerOne;

pub struct GridLocation{
    owner: Player
}

pub struct BoardState {
    cols: Vec<Vec<GridLocation>>
}

#[derive (PartialEq, Eq)]
enum Player{
    None,
    PlayerOne,
    PlayerTwo
}

pub struct GameState {
    board: BoardState,
    next_player: Player,
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
        for row in 0..cols{
            let mut row_vec = Vec::with_capacity(cols-1);
            for col in 0..rows{
                row_vec.push(GridLocation{owner:Player::None})
            }
            col_vec.push(row_vec);
        }
        BoardState{cols: col_vec}
    }

    fn play_move(&self, player: Player, column: usize) -> bool{
        let mut prev_row: Player = *self.cols[column][0].owner;
        if prev_row.is_player(){
            return false
        }
        for row in self.cols[column] {
            if row.owner.is_player() {
                return if row.owner != prev_row {
                    prev_row = player;
                    true
                } else {
                    false
                }
            }
        }
        return false
    }

    fn check_winner(&self) -> Player{
        PlayerOne
    }
}

impl GameState {
    pub fn init(rows: usize, cols: usize) -> Self{
        GameState{ next_player: PlayerOne, board: BoardState::new(rows, cols)}
    }

    pub fn play_move(&mut self, column: usize) -> Player {
        self.board.play_move(self.get_next_player(), column);
        let i = self.board.check_winner();
        return PlayerOne

    }

    fn get_next_player(&mut self) -> Player {
        match &self.next_player {
            PlayerOne => {
                self.next_player = Player::PlayerTwo;
                return PlayerOne},
            PlayerTwo => {
                self.next_player = PlayerOne;
                return Player::PlayerTwo},
            _ => {panic!("Next player is none")}
        }
    }
}