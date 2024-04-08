use crate::pages::connect4::Connect4;
use crate::pages::home::Home;
use crate::pages::tootOtto::TootOtto;
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum RootRoutes {
    #[at("/")]
    Home,
    #[at("/connect4")]
    Connect4,
    #[at("/tootOtto")]
    TootOtto,
}

pub fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::Home => html! { <Home/> },
        RootRoutes::Connect4 => html! {<Connect4/>},
        RootRoutes::TootOtto => html! {<TootOtto/>},
    }
}
