use crate::components::connect4Grid::Connect4Grid;
use reqwest::Client;
use serde_json::json;
use crate::components::header::Header;
use yew::prelude::*;

pub async fn set_game(){
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/getGame";
    
    let request_body = json!({
        "text": "Connect4"
    });
    let request = client.post(endpoint)
            .json(&request_body)
            .send()
            .await;
   
}
pub async fn refresh() {
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/refreshGame";

    let response = client.post(endpoint).send().await.unwrap(); // Handle errors properly in production code
}

#[function_component(Connect4)]
pub fn connect4() -> Html {
    use_effect(move || {
        wasm_bindgen_futures::spawn_local(set_game());
        wasm_bindgen_futures::spawn_local(refresh());
        || {
            // Cleanup code goes here
        }
    });

    html! {
        <div>
       <Connect4Grid>
       </Connect4Grid>
        </div>
    }
}
