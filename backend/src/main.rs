#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
use rocket::http::Method;
use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};
use std::io;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use reqwest;
mod ai;
mod game_logic;

use ai::connect_four_bot::ConnectFourBot;
use ai::toot_otto_bot::TootOttoBot;

use crate::game_logic::connect_four::ConnectFour;
use crate::game_logic::toot_otto::TootOtto;
use crate::game_logic::game_info::{GameState, Message};
use crate::game_logic::game_info::Player;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref RESULT: Mutex<Message> = Mutex::new(Message::NextPlayer(Player::PlayerOne));
    static ref CONNECT_FOUR: Mutex<ConnectFour> = Mutex::new(ConnectFour::init(6, 7));
    static ref TOOT_OTTO: Mutex<TootOtto> = Mutex::new(TootOtto::init(4,6));
    static ref DIFFICULTY_LEVEL: Mutex<usize> = Mutex::new(5); // Default difficulty level
    static ref GAME_CHOICE: Mutex<String> = Mutex::new("".to_string());
}

#[derive(Serialize, Deserialize)]
struct JsonMessage {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct TurnInfo{
    col_num: String,
    token: char
}
#[derive(Serialize, Deserialize)]
struct CountData {
    count: u32,
}

#[post("/api/getCol", data = "<message>")]
fn receive_col_num(message: Json<TurnInfo>)-> String{
    let mut result = RESULT.lock().unwrap();
    //let game_choice = "Connect4";
    // let mut game_choice = GAME_CHOICE.lock().unwrap();
    let mut connect4 = true;

    // match game_choice.as_str() {
    //     "Connect4" => {connect4 = true},
    //     "TootOtto" => {connect4 = false},
    //     _ => panic!("Invalid game choice"), // Handle any other unexpected value
    // };

    let column_number = message.col_num.clone();
    let ip: usize = column_number.trim().parse().expect("Invalid Input\n");
    println!("Received column number: {}", message.col_num);

    if connect4 {
        let mut game = CONNECT_FOUR.lock().unwrap();
        *result = game.play_move(ip);
        println!("{:?}", game);
    }else {
        let mut game = TOOT_OTTO.lock().unwrap();
        let token = message.token.clone();
        println!("Received token: {}", message.token);
        *result = game.play_move(ip, token);
        println!("{:?}", game);
    }

    let mut response = "".to_string();
    match *result {
        Message::Winner(Player::PlayerOne) => {
            response = "1".to_string();
        }
        Message::Winner(Player::PlayerTwo) => {
            response = "2".to_string();
        }
        Message::Tie => {
            response = "0".to_string();
        }
        _ => { response = "3".to_string();}
    }

    response
}

pub fn update_difficulty_level(level: usize) {
    let mut difficulty_level = DIFFICULTY_LEVEL.lock().unwrap();
    *difficulty_level = level;
}

pub fn update_game_choice(game: String) {
    let mut game_choice = GAME_CHOICE.lock().unwrap();
    *game_choice = game;
}

#[post("/api/setDifficulty", data = "<message>")]
fn receive_difficulty(message: Json<JsonMessage>){
    println!("received difficulty: {}", message.text);
    let difficulty_level = match message.text.as_str() {
        "easy" => 5,
        "medium" => 500,
        "hard" => 1000,
        _ => {
            println!("Invalid difficulty level");
            return; // Handle invalid difficulty level
        }
    };
    update_difficulty_level(difficulty_level);
}

#[post("/api/getGame", data = "<message>")]
fn receive_game(message: Json<JsonMessage>) {
    println!("Received game name: {}", message.text);
    update_game_choice(message.text.clone());
}

#[post("/api/refreshGame")]
fn refresh_game() {
    // Reinitialize RESULT to its initial state
    {
        let mut result = RESULT.lock().unwrap();
        *result = Message::NextPlayer(Player::PlayerOne);
    }

    // Reinitialize CONNECT_FOUR to its initial state
    {
        let mut game_choice = GAME_CHOICE.lock().unwrap();
        let mut connect4 = true;
    
        match game_choice.as_str() {
            "Connect4" => {connect4 = true},
            "TootOtto" => {connect4 = false},
             _ => {connect4 = true}, // Handle any other unexpected value
        };
        if connect4{
            let mut game = CONNECT_FOUR.lock().unwrap();
            *game = ConnectFour::init(6, 7); // Assuming init is a function that initializes the game
        }else{
            let mut game = TOOT_OTTO.lock().unwrap();
            *game = TootOtto::init(4,6);
            println!("refreshed tootOtto");
        }
        
    }

}

#[get("/api/botMove")]
fn bot_move() -> JsonValue {
    let mut result = RESULT.lock().unwrap();
    let difficulty_level = *DIFFICULTY_LEVEL.lock().unwrap() as i32; 
    println!("difficulty level: {}",difficulty_level);
    let mut game_choice = GAME_CHOICE.lock().unwrap();
        let mut connect4 = true;
    
        let mut response = json!({
            "bot_move": "",
            "token": "",
            "message": ""
        });

        match game_choice.as_str() {
            "Connect4" => {connect4 = true},
            "TootOtto" => {connect4 = false},
            _ => panic!("Invalid game choice"), // Handle any other unexpected value
        };
        if connect4{
            let mut game = CONNECT_FOUR.lock().unwrap();
            let mut mcst = ConnectFourBot::new(game.clone(), difficulty_level, Player::PlayerTwo);
            let bot_move = mcst.select_move();
            *result = game.play_move(bot_move);
            println!("bot_move: {:?}", bot_move);

            let bot_move_str = bot_move.into();
            response["bot_move"] = bot_move_str;
            response["token"] = "O".to_string().into();
            println!("{:?}", game);
        }else{
            let mut game = TOOT_OTTO.lock().unwrap();
            let mut mcst = TootOttoBot::new(game.clone(), difficulty_level, Player::PlayerTwo);
            let bot_move = mcst.select_move();
            println!("bot_move: {:?}", bot_move);
            *result = game.play_move(bot_move.0, bot_move.1);
            response["bot_move"] = bot_move.0.into();
            response["token"] = bot_move.1.to_string().into();
            println!("{:?}", game);
        }
    

    match *result {
        Message::Winner(Player::PlayerOne) => {
            response["message"] = "1".into();
        }
        Message::Winner(Player::PlayerTwo) => {
            response["message"] = "2".into();
        }
        Message::Tie => {
            response["message"] = "0".into();
        }
        _ => {response["message"] = "3".into();}
    }
    response
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
        "http://127.0.0.1:5500",
    ]);
    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get,
        Method::Post].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type" // Include Content-Type header
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let file_path = Path::new("../frontend/dist/").join(&file);
    println!("Requested file location: {:?}", file_path);
    NamedFile::open(file_path).ok()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    let index_file_path = "../frontend/dist/index.html";
    println!("Requested index file location: {:?}", index_file_path);
    NamedFile::open(index_file_path)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, files,receive_game,receive_col_num, bot_move, refresh_game, receive_difficulty])
        .attach(make_cors())
}
fn main() {
    rocket().launch();
}
