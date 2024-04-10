mod web_assembly;
mod ai;
mod game_logic;
use game_logic::connect_four::ConnectFour;
use crate::game_logic::game_info::{GameState, Message, Player};
use crate::game_logic::toot_otto::TootOtto;

fn main() {
    let play_vec: Vec<usize> = vec![0, 0, 0, 0];
    let letter_vec: Vec<char> = vec!['o', 't', 't', 'o'];
    let mut game = TootOtto::init(4, 6);
    let mut result = Message::NextPlayer(Player::PlayerOne);
    for i in 0..play_vec.len() {
        result = game.play_move(play_vec[i], letter_vec[i]);
    }
    println!("{:?}", game);

}
