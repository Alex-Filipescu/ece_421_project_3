use reqwest::Client;
use serde_json::json;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SliderProps {
    pub diff_change: Callback<usize>, // Callback to pass the column index to the parent
}
pub async fn set_difficulty(value: String) {
    let client = Client::new();
    let endpoint = "http://localhost:8000/api/setDifficulty";
    let request_body = json!({
        "text": value
    });

    let response = client
        .post(endpoint)
        .json(&request_body)
        .send()
        .await
        .unwrap(); // Handle errors properly in production code
}

#[function_component]
pub fn Slider(props: &SliderProps) -> Html {
    // Define state for styling
    let translation = use_state(|| "horizTranslate1");
    let diff_change = props.diff_change.clone();

    let go_to_1 = {
      
        let translation_clone = translation.clone();
        let diff_change_clone = diff_change.clone();

        Callback::from(move |_| {
            let level = "easy".to_string();
            diff_change_clone.emit(1);
            wasm_bindgen_futures::spawn_local(set_difficulty(level.clone())); // Call set_difficulty
            // Update styling
            translation_clone.set("horizTranslate1");
        })
        
    };

    let go_to_2 = {
      
        let translation_clone = translation.clone();

        let diff_change_clone = diff_change.clone();

        Callback::from(move |_| {
            let level = "medium".to_string();
            diff_change_clone.emit(2);
            wasm_bindgen_futures::spawn_local(set_difficulty(level.clone())); // Call set_difficulty
    
            // Update styling
          
            translation_clone.set("horizTranslate2");

        })
    };
  

    let go_to_3 = {
        let translation_clone = translation.clone();

        let diff_change_clone = diff_change.clone();

        Callback::from(move |_| {
            let level = "hard".to_string();
            diff_change_clone.emit(3);
            wasm_bindgen_futures::spawn_local(set_difficulty(level.clone())); // Call set_difficulty
        // Update styling
        translation_clone.set("horizTranslate3");

        })
    };

    html! {
        <div id="outerContainer">
            <div id="buttonContainer" >
                <div class={format!("redButton {}", *translation)}></div>
            </div>
            <div id="legendTextContainer" >
                <div class="legendText" onclick={go_to_1}>{1}</div>
                <div class="legendText" onclick={go_to_2}>{2}</div>
                <div class="legendText" onclick={go_to_3}>{3}</div>
            </div>
        </div>
    }
}
