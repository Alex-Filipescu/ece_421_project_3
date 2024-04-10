use std::time::Duration;

use crate::components::column::Col;
use reqwest::Client;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use yew::{ platform::spawn_local, platform::time::sleep, prelude::*, suspense::* };

use crate::components::slider::Slider;
use wasm_bindgen::JsCast;
use web_sys::{ EventTarget, HtmlInputElement };

#[derive(Serialize, Deserialize, Debug)]
struct botResponse {
    bot_move: usize,
    token: String,
    message: String,
}

pub enum Msg {
    x_value(String),
    o_value(String),
}

pub async fn user_move(
    col_index: usize,
    cell_states: UseStateHandle<Vec<Vec<char>>>,
    result_message: UseStateHandle<String>
) {
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/getCol";
    let col_num = col_index.to_string();
    let token = 'X';
    let request_body = json!({
        "col_num": col_num,
        "token": token,
    });

    // Update cells
    let mut updated_cells = cell_states.to_vec().clone();
    let mut cells_clone = updated_cells[col_index].clone();

    let mut is_full = true;
    let mut counter = 0;
    for c in cells_clone.iter_mut().rev() {
        if *c == ' ' {
            *c = 'X'; // Change the cell to 'X' if empty
            is_full = false;
            break;
        } else {
            counter += 1;
        }
    }

    //if column is full, it's the user's turn again
    if is_full {
        return;
    }

    updated_cells[col_index] = cells_clone;
    cell_states.set(updated_cells.clone());

    let response = client.post(endpoint).json(&request_body).send().await.unwrap(); // Handle errors properly in production code

    let res: i32 = response.json().await.unwrap();

    if res == 1 {
        let res_msg = "You won!!!";
        result_message.set(res_msg.to_string());
        return;
    } else if res == 0 {
        let res_msg = "Tie!!";
        result_message.set(res_msg.to_string());
        return;
    } else if res == 2 {
        let res_msg = "Bot won!!!";
        result_message.set(res_msg.to_string());
        return;
    }

    let delay = Duration::new(1, 0);

    sleep(delay).await;
    // Call bot_move to get the next move
    bot_move(cell_states.clone(), updated_cells.clone(), result_message.clone()).await;
}

pub async fn bot_move(
    cell_states: UseStateHandle<Vec<Vec<char>>>,
    updated_cells: Vec<Vec<char>>,
    result_message: UseStateHandle<String>
) {
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/botMove";

    let response = client
        .get(endpoint)
        // .json(&request_body)
        .send().await
        .unwrap(); // Handle errors properly in production code

    let res: botResponse = response.json().await.unwrap();
    let bot_response_col_index = res.bot_move;
    // match result to determine if a game has been won or not

    // Update cells again with the new information
    let mut updated_cells = updated_cells.clone();
    let mut cells_clone = updated_cells[bot_response_col_index].clone();
    for c in cells_clone.iter_mut().rev() {
        if *c == ' ' {
            *c = 'O'; // Change the cell to 'O' for the bot's move
            break;
        }
    }
    updated_cells[bot_response_col_index] = cells_clone;
    cell_states.set(updated_cells);

    if res.message == "1" {
        let res_msg = "You won!!!";
        result_message.set(res_msg.to_string());
        return;
    } else if res.message == "0" {
        let res_msg = "Tie!!";
        result_message.set(res_msg.to_string());
        return;
    } else if res.message == "2" {
        let res_msg = "Bot won!!!";
        result_message.set(res_msg.to_string());
        return;
    }
}

pub async fn get_hint(hint: UseStateHandle<String>) {
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/getHint";

    let response = client.get(endpoint).send().await.unwrap(); // Handle errors properly in production code

    let res: botResponse = response.json().await.unwrap();
    // match result to determine if a game has been won or not
    let mut col = res.bot_move;
    col += 1; //add 1 for 1 indexing
    hint.set(col.to_string());
}

pub async fn refresh() {
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/refreshGame";

    let response = client.post(endpoint).send().await.unwrap(); // Handle errors properly in production code
}

#[function_component(Connect4Grid)]
pub fn connect4grid() -> Html {
    let all_cells: Vec<Vec<char>> = vec![vec![' '; 6]; 7];
    let cell_states = use_state(|| all_cells.clone());
    let result_message = use_state(|| " ".to_string());
    let running = use_state(|| false);
    let hint = use_state(|| " ".to_string());
    let hint_visible = use_state(|| false); // State to control hint visibility
    let user_color = use_state(|| "#FFFFFF".to_string());
    let bot_color = use_state(|| "#FFFFFF".to_string());

    let onclick_callback = {
        let cell_states = cell_states.clone();
        let result_message = result_message.clone();
        let running = running.clone();

        Callback::from(move |col_index: usize| {
            let cell_states = cell_states.clone();
            let result_message = result_message.clone();
            let running = running.clone();

            // Check if user_move is currently running
            if *running {
                return; // Return early if user_move is still running
            }

            running.set(true);
            if *result_message == " ".to_string() {
                //game did not end
                let task = user_move(col_index, cell_states.clone(), result_message.clone());
                spawn_local(async move {
                    task.await;
                    running.set(false); // Set is_user_move_running to false after user_move finishes
                })
            }
        })
    };

    let onclick = {
        //for button refresh
        let cell_states = cell_states.clone();
        let result_message = result_message.clone();
        let user_color = user_color.clone();
        let bot_color = bot_color.clone();
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(refresh()); // Call set_difficulty
            cell_states.set(vec![vec![' '; 6]; 7]);
            result_message.set(" ".to_string());
            user_color.set("#FFFFFF".to_string());
            bot_color.set("#FFFFFF".to_string());
        })
    };
    let diff_change = {
        //when user changes difficulty
        let cell_states = cell_states.clone();
        let result_message = result_message.clone();
        let user_color = user_color.clone();
        let bot_color = bot_color.clone();
        Callback::from(move |diff_change: usize| {
            wasm_bindgen_futures::spawn_local(refresh());
            cell_states.set(vec![vec![' '; 6]; 7]);
            result_message.set(" ".to_string());
            user_color.set("#FFFFFF".to_string());
            bot_color.set("#FFFFFF".to_string());
        })
    };

    let onmouseover = {
        let hint_clone = hint.clone();
        let hint_visible = hint_visible.clone();

        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(get_hint(hint_clone.clone()));
            hint_visible.set(true); // Set hint visibility to true
        })
    };

    let onmouseleave = {
        let hint = hint.clone();
        let hint_visible = hint_visible.clone();

        Callback::from(move |_| {
            hint.set(" ".to_string()); // Reset hint when mouse leaves
            hint_visible.set(false); // Reset hint visibility when mouse leaves
        })
    };

    let on_user_color_change = {
        let user_color = user_color.clone();
        Callback::from(move |e: Event| {
            // Clone user_color inside the closure to avoid borrowing issues
            let user_color = user_color.clone();

            // When events are created, the target is initially undefined.
            // It's only when dispatched does the target get added.
            if let Some(target) = e.target() {
                // Convert the event target to an HTML input element
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    // Set the color value from the input element
                    user_color.set(input.value());
                }
            }
        })
    };

    let on_bot_color_change = {
        let bot_color = bot_color.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                bot_color.set(input.value());
            }
        })
    };

    html! { 
        <div class = "connect4Grid">
            <div class="sliderContainer">
                <div class="leftSection">
                </div>
                <div class="middleSection">
                    <div class = "sliderLeft">
                        <span>
                        {"Select Difficulty:"}
                        </span>
                    </div>
                    <div class = "sliderMiddle">
                        <Slider diff_change={diff_change.clone()}></Slider>
                    </div>
                    <div class = "sliderRight">
                    </div>
                </div>
                <div class="rightSection">
                </div>
            </div>
            <div class = "gridControls">
                <div class = "gridLeft"></div>
                <div class = "gridMiddle">
                    <div class="grid-container">
                        <div class="grid">
                        <Col index = 0 on_click={onclick_callback.clone() } cells={cell_states.clone()[0].clone()} cell_num=6 user_color = {(*user_color).to_string()} bot_color = {(*bot_color).to_string()} />
                        <Col index = 1 on_click={onclick_callback.clone() } cells={cell_states.clone()[1].clone()} cell_num=6 user_color = {(*user_color).to_string()} bot_color = {(*bot_color).to_string()} />
                        <Col index = 2 on_click={onclick_callback.clone() } cells={cell_states.clone()[2].clone()} cell_num=6 user_color = {(*user_color).to_string()} bot_color = {(*bot_color).to_string()} />
                        <Col index = 3 on_click={onclick_callback.clone() } cells={cell_states.clone()[3].clone()} cell_num=6 user_color = {(*user_color).to_string()} bot_color = {(*bot_color).to_string()} />
                        <Col index = 4 on_click={onclick_callback.clone() } cells={cell_states.clone()[4].clone()} cell_num=6 user_color = {(*user_color).to_string()} bot_color = {(*bot_color).to_string()} />
                        <Col index = 5 on_click={onclick_callback.clone() } cells={cell_states.clone()[5].clone()} cell_num=6 user_color = {(*user_color).to_string()} bot_color = {(*bot_color).to_string()} />
                        <Col index = 6 on_click={onclick_callback.clone() } cells={cell_states.clone()[6].clone()} cell_num=6 user_color = {(*user_color).to_string()} bot_color = {(*bot_color).to_string()} />
                        </div>
                    </div>
                    <p>{format!("Result: {}", *result_message)}</p>
                    <button {onmouseover} {onmouseleave}>{"Hint"}</button>
                    <p style={format!("display: {}", if *hint_visible { "block" } else { "none" })}>{format!("Column: {} ",*hint)}</p> // Show hint when hint_visible is true
                </div>
                <div class = "gridRight">
                    <button class = "refreshButton" {onclick}>{"Refresh"}</button>
                    <div class = "color-choosers">
                        <div>
                        <label>{"X Color "}
                        <input type="color" value={(*user_color).to_string()} onchange={on_user_color_change} />
                        </label>
                        </div>
                        
                        <div>
                        <label>{"O Color "}
                        <input type="color" value={(*bot_color).to_string()} onchange={on_bot_color_change} />
                        </label>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
