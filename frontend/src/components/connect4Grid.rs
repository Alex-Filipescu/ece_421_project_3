use std::{borrow::BorrowMut, future::IntoFuture};

use yew::{platform::spawn_local, prelude::*, suspense::*};
use crate::components::column::Col;
use wasm_bindgen::JsValue;
use serde_json::json;
use reqwest::Client;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct botResponse{
    col_num:i32,
    token:char,
    result:i32
}
pub async fn user_move(col_index:usize){
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/getCol";
    let col_num = col_index.to_string();
    let token = 'X';
    let request_body = json!({
        "col_num": col_num,
        "token": token,
    });

    let response = client.post(endpoint)
        .json(&request_body)
        .send()
        .await
        .unwrap(); // Handle errors properly in production code

    let res :i32 = response.json().await.unwrap(); 

}
pub async fn bot_move()-> i32{
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/botMove";

    let response = client.post(endpoint)
        // .json(&request_body)
        .send()
        .await
        .unwrap(); // Handle errors properly in production code

    let res :botResponse = response.json().await.unwrap(); 
    res.col_num
    // match result to determine if a game has been won or not
}


#[function_component(Grid)]
pub fn connect4grid()-> Html{

    let onclick_callback = Callback::from(move |col_index: usize| {
        // Handle the click event here, you can use col_index
        // This function will be called whenever a column is clicked
        let greeting = format!("Column {} clicked", col_index);
        web_sys::console::log_1(&JsValue::from_str(&greeting));
        let task = user_move(col_index.clone());
        wasm_bindgen_futures::spawn_local(task);

        // Update the board state when a column is clicked
        //let user_task = user_move(col_index.clone());
        //wasm_bindgen_futures::spawn_local(user_task);
    
    });
    html! {
        <div class="grid">
            <Col index = 0 on_click={onclick_callback.clone()} />
            <Col index = 1 on_click={onclick_callback.clone()}/>
            <Col index = 2 on_click={onclick_callback.clone()}/>
            <Col index = 3 on_click={onclick_callback.clone()}/>
            <Col index = 4 on_click={onclick_callback.clone()}/>
            <Col index = 5 on_click={onclick_callback.clone()}/>
            <Col index = 6 on_click={onclick_callback.clone()}/>
        </div>
    }

}