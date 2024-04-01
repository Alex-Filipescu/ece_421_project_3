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
    static ref GAME: Mutex<ConnectFour> = Mutex::new(ConnectFour::init(6, 7));
    static ref DIFFICULTY_LEVEL: Mutex<usize> = Mutex::new(5); // Default difficulty level
}

#[derive(Serialize, Deserialize)]
struct JsonMessage {
    text: String,
}
#[derive(Serialize, Deserialize)]
struct CountData {
    count: u32,
}

#[post("/api/getCol", data = "<message>")]
fn receive_col_num(message: Json<JsonMessage>)-> String{
    let mut result = RESULT.lock().unwrap();
    let mut game = GAME.lock().unwrap();

    println!("Received column number: {}", message.text);
    let column_number = message.text.clone();
    let ip: usize = column_number.trim().parse().expect("Invalid Input\n");
    *result = game.play_move(ip);
    println!("{:?}", game);

    let mut response = "".to_string();
    match *result {
        Message::Winner(Player::PlayerOne) => {
            response = "👎 BOO: you win".to_string()
        }
        Message::Winner(Player::PlayerTwo) => {
            response = "🥳 YAY: BOT WINS".to_string();
        }
        Message::Tie => {
            response = "BOO TIE".to_string();
        }
        _ => { response = "made a move".to_string()}
    }
    response
}
pub fn update_difficulty_level(level: usize) {
    let mut difficulty_level = DIFFICULTY_LEVEL.lock().unwrap();
    *difficulty_level = level;
}

#[post("/api/setDifficulty", data = "<message>")]
fn receive_difficulty(message: Json<JsonMessage>){
    println!("received difficulty: {}", message.text);
    let difficulty_level = match message.text.as_str() {
        "easy" => 5,
        "medium" => 100,
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
}

#[post("/api/refreshGame")]
fn refresh_game() {
    // Reinitialize RESULT to its initial state
    {
        let mut result = RESULT.lock().unwrap();
        *result = Message::NextPlayer(Player::PlayerOne);
    }

    // Reinitialize GAME to its initial state
    {
        let mut game = GAME.lock().unwrap();
        *game = ConnectFour::init(6, 7); // Assuming init is a function that initializes the game
    }
       // Reinitialize Difficulty level to default
    {
        let mut difficulty_level = DIFFICULTY_LEVEL.lock().unwrap();
        *difficulty_level = 5;
    }
}

#[get("/api/botMove")]
fn bot_move() -> JsonValue {
    let mut result = RESULT.lock().unwrap();
    let mut game = GAME.lock().unwrap();
    let difficulty_level = *DIFFICULTY_LEVEL.lock().unwrap() as i32; 

    let mut mcst = ConnectFourBot::new(game.clone(), difficulty_level);
    let bot_move = mcst.select_move();
    *result = game.play_move(bot_move);

    println!("{:?}", game);

    let bot_move_str = bot_move.to_string();
    let mut response = json!({
        "bot_move": bot_move_str,
        "message": ""
    });

    match *result {
        Message::Winner(Player::PlayerOne) => {
            response["message"] = "👎 BOO: you win".into();
        }
        Message::Winner(Player::PlayerTwo) => {
            response["message"] = "🥳 YAY: BOT WINS".into();
        }
        Message::Tie => {
            response["message"] = "BOO TIE".into();
        }
        _ => {response["message"] = "bot made a move".into();}
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
    let file_path = Path::new("../front_end/build/").join(&file);
    println!("Requested file location: {:?}", file_path);
    NamedFile::open(file_path).ok()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    let index_file_path = "../front_end/build/index.html";
    println!("Requested index file location: {:?}", index_file_path);
    NamedFile::open(index_file_path)
}


fn play_toot_otto(){
    let mut result = Message::NextPlayer(Player::PlayerOne);
    let mut game = TootOtto::init(4, 6);

    println!("\nYou are X, Bot is O");
    println!("{:?}", game);

    loop {
        println!("| 0 | 1 | 2 | 3 | 4 | 5 |\n");
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
        let mut mcst = TootOttoBot::new(game.clone(), 500);
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

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, files,receive_game,receive_col_num, bot_move, refresh_game, receive_difficulty])
        .attach(make_cors())
}
fn main() {
    rocket().launch();
}
