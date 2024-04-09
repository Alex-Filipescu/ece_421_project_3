use yew::html;
use yew::prelude::*;
mod components;
mod pages;
mod router;
use crate::components::header::Header;
use crate::pages::connect4::Connect4;
use crate::pages::tootOtto::TootOtto;

use crate::pages::home::Home;
use crate::router::{switch, RootRoutes};
use stylist::{style, yew::styled_component, Style};

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <Header>
        </Header>
       
      
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
