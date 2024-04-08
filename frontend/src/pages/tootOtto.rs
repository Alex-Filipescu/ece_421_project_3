use yew::prelude::*;
use crate::components::tootOttoGrid::TootOttoGrid;
use reqwest::Client;
use serde_json::json;

pub async fn set_game(){
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/getGame";
    
    let request_body = json!({
        "text": "TootOtto"
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

#[function_component(TootOtto)]
pub fn tootOtto() -> Html {
    use_effect(move || {
        wasm_bindgen_futures::spawn_local(set_game());
        wasm_bindgen_futures::spawn_local(refresh());
        || {
            // Cleanup code goes here
        }
    });
    html! {
        <div>
       <TootOttoGrid>
       </TootOttoGrid>
       </div>
    }
}
