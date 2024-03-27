mod web_assembly;
mod ai;
mod game_logic;

use std::io;
use ai::connect_four_bot::ConnectFourBot;

use crate::game_logic::connect_four::ConnectFour;
use crate::game_logic::game_info::{GameState, Message};
use crate::game_logic::game_info::Player;

fn main() {
    let mut game = ConnectFour::init(6, 7);
    let mut result = Message::NextPlayer(Player::PlayerOne);

    println!("\nYou are X, Bot is O");
    println!("{:?}", game);

    loop {
        println!("| 0 | 1 | 2 | 3 | 4 | 5 | 6 |\n");
        println!("your move: ");
        let mut ip = String::new();
        io::stdin().read_line(&mut ip).expect("Failed to read line\n");
        let ip: i8 = ip.trim().parse().expect("Invalid Input\n");

        println!("");

        result = game.play_move(ip as usize);
        println!("{:?}", game);

        if result == Message::Winner(Player::PlayerOne) || result == Message::Winner(Player::PlayerTwo) || result == Message::Tie {
            break;
        }

        // hard: 1000, medium: 500, easy: 5
        let mut mcst = ConnectFourBot::new(game.clone(), 1000);
        result = game.play_move(mcst.select_move());
        println!("{:?}", game);

        match result {
            Message::Winner(Player::PlayerOne) => {
                println!("👎 BOO: you win");
                break;
            }
            Message::Winner(Player::PlayerTwo) => {
                println!("🥳 YAY: BOT WINS"); 
                break;
            }
            Message::Tie => {
                println!("BOO TIE");
                break;
            }
            _ => {}
        }
    }
}