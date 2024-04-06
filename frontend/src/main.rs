use yew::prelude::*;
use yew::{html};
mod pages;
mod components;
mod router;
use yew_router::prelude::*;
use crate::router::{RootRoutes, switch};
use stylist::{style, yew::styled_component,Style};
use yew_router::prelude::*;
use crate::pages::home::Home;
use crate::pages::connect4::Connect4;
use crate::components::header::Header;
#[macro_use]
extern crate lazy_static;

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
