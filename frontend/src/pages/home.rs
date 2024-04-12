use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class = "pageContent">
        <div class = "instructions">
            <h1 class = "home">{"Home"}</h1>
            <img src = "img/logo.svg"/>
            <h2 class = "welcome">{"Welcome!"}</h2>
            <h3>{"Connect 4 Instructions:"}</h3>
            <p>
                {"First one to get 4 tokens in a row (vertically, horizontally, diagonally) wins!"}
            </p>
            <p>
                {"You will use the X token and play first, while the bot opponent will use the O token."}
            </p>

            <br/>

            <h3>{"Toot Otto Intructions:"}</h3>
            <p>
            {"First one to create TOOT or OTTO (vertically, horizontally, diagonally) on the board wins!"}
            </p>
            <p>
                {"You will be trying to get TOOT, while the bot opponent will try to get OTTO."}
            </p>
            <br/>
            <h3>{"Have fun!"}</h3>
        </div>
        </div>
    }
}
