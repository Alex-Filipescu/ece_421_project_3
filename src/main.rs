mod web_assembly;
mod ai;
mod game_logic;

use std::io;
use ai::connect_four_bot::ConnectFourBot;
use ai::toot_otto_bot::TootOttoBot;

use crate::game_logic::connect_four::ConnectFour;
use crate::game_logic::toot_otto::TootOtto;
use crate::game_logic::game_info::{GameState, Message};
use crate::game_logic::game_info::Player;

fn main() {
    let mut result = Message::NextPlayer(Player::PlayerOne);

    println!("1: Connect Four\n2:Toot Otto");
    let mut gametype = String::new();
    io::stdin().read_line(&mut gametype).expect("Failed to read line\n");
    let gametype: i8 = gametype.trim().parse().expect("Invalid Input\n");

    match gametype {
        1 => {
            let mut game = ConnectFour::init(6, 7);

            println!("\nYou are X, Bot is O");
            println!("{:?}", game);

            loop {
                println!("| 0 | 1 | 2 | 3 | 4 | 5 | 6 |\n");

                let mut suggest = ConnectFourBot::new(game.clone(), 1000, Player::PlayerOne);
                println!("Suggested Move: {}", suggest.select_move());

                println!("your move: ");
                let mut ip = String::new();
                io::stdin().read_line(&mut ip).expect("Failed to read line\n");
                let ip: i8 = ip.trim().parse().expect("Invalid Input\n");

                println!("");

                result = game.play_move(ip as usize);
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

                // hard: 1000, medium: 500, easy: 5
                let mut bot = ConnectFourBot::new(game.clone(), 1000, Player::PlayerOne);
                result = game.play_move(bot.select_move());
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
        2 => {
            let mut game = TootOtto::init(4, 6);

            println!("\nYou are TOOT, Bot is OTTO");
            println!("{:?}", game);

            loop {
                println!("| 0 | 1 | 2 | 3 | 4 | 5 |\n");

                let mut suggest = TootOttoBot::new(game.clone(), 2000, Player::PlayerOne);
                let suggest_move = suggest.select_move();
                println!("Suggested Move: {} {}", suggest_move.0, suggest_move.1);

                println!("your move: ");
                let mut ip = String::new();
                io::stdin().read_line(&mut ip).expect("Failed to read line\n");
                let args: Vec<&str> = ip.trim().split_whitespace().collect();
                let col: usize = args[0].trim().parse().expect("Invalid Input\n");
                let token: char = args[1].trim().parse().expect("Invalid Input\n");

                println!("");

                result = game.play_move(col as usize, token);
                println!("{:?}", game);

                if result == Message::Winner(Player::PlayerOne) || result == Message::Winner(Player::PlayerTwo) || result == Message::Tie {
                    break;
                }

                // hard: 1000, medium: 500, easy: 5
                let mut mcst = TootOttoBot::new(game.clone(), 2000, Player::PlayerTwo);
                let bot_move = mcst.select_move();
                result = game.play_move(bot_move.0, bot_move.1);
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
        _ => println!("Invalid Input")
    }
}

// fn main() {
//     let mut result = Message::NextPlayer(Player::PlayerOne);

//     println!("1: Connect Four\n2:Toot Otto");
//     let mut gametype = String::new();
//     io::stdin().read_line(&mut gametype).expect("Failed to read line\n");
//     let gametype: i8 = gametype.trim().parse().expect("Invalid Input\n");

//     match gametype {
//         1 => {
//             let mut bot_tally = 0;
//             let mut suggestion_tally = 0;
//             let mut tie_tally = 0;

//             for i in 0..50 {
//                 let mut game = ConnectFour::init(6, 7);
//                 println!("playing game {} of 50", i+1);

//                 loop {
//                     let mut suggest = ConnectFourBot::new(game.clone(), 1000, Player::PlayerOne);
//                     result = game.play_move(suggest.select_move());

//                     match result {
//                         Message::Winner(Player::PlayerOne) => {
//                             suggestion_tally += 1;
//                             break;
//                         }
//                         Message::Winner(Player::PlayerTwo) => {
//                             bot_tally += 1;
//                             break;
//                         }
//                         Message::Tie => {
//                             tie_tally += 1;
//                             break;
//                         }
//                         _ => {}
//                     }

//                     // hard: 1000, medium: 500, easy: 5
//                     let mut bot = ConnectFourBot::new(game.clone(), 1000, Player::PlayerTwo);
//                     result = game.play_move(bot.select_move());

//                     match result {
//                         Message::Winner(Player::PlayerOne) => {
//                             suggestion_tally += 1;
//                             break;
//                         }
//                         Message::Winner(Player::PlayerTwo) => {
//                             bot_tally += 1;
//                             break;
//                         }
//                         Message::Tie => {
//                             tie_tally += 1;
//                             break;
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             println!("Bot {}, Suggestions {}, Tie {}", bot_tally, suggestion_tally, tie_tally)
//         }
//         2 => {
//             let mut bot_tally = 0;
//             let mut suggestion_tally = 0;
//             let mut tie_tally = 0;

//             for i in 0..50 {
//                 let mut game = TootOtto::init(4, 6);
//                 println!("playing game {} of 50", i+1);

//                 loop {
//                     let mut suggest = TootOttoBot::new(game.clone(), 1000, Player::PlayerOne);
//                     result = game.play_move(suggest.select_move().0, suggest.select_move().1);

//                     match result {
//                         Message::Winner(Player::PlayerOne) => {
//                             suggestion_tally += 1;
//                             break;
//                         }
//                         Message::Winner(Player::PlayerTwo) => {
//                             bot_tally += 1;
//                             break;
//                         }
//                         Message::Tie => {
//                             tie_tally += 1;
//                             break;
//                         }
//                         _ => {}
//                     }

//                     // hard: 1000, medium: 500, easy: 5
//                     let mut bot = TootOttoBot::new(game.clone(), 1000, Player::PlayerTwo);
//                     result = game.play_move(bot.select_move().0, bot.select_move().1);

//                     match result {
//                         Message::Winner(Player::PlayerOne) => {
//                             suggestion_tally += 1;
//                             break;
//                         }
//                         Message::Winner(Player::PlayerTwo) => {
//                             bot_tally += 1;
//                             break;
//                         }
//                         Message::Tie => {
//                             tie_tally += 1;
//                             break;
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             println!("Bot {}, Suggestions {}, Tie {}", bot_tally, suggestion_tally, tie_tally)
//         }
//         _ => println!("Invalid Input")
//     }
// }