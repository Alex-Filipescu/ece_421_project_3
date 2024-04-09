use yew::prelude::*;

use crate::router::{switch, RootRoutes};
use yew_router::prelude::*;
use web_sys::console;

#[function_component(Header)]
pub fn header() -> Html {

    html! {
        <div class = "header-container">
        <BrowserRouter>
        <ul class = "header-list">
                <li class = "header-item"><Link<RootRoutes> to={RootRoutes::Home}>{ "Home" }</Link<RootRoutes>></li>
                <li class = "header-item"><Link<RootRoutes> to={RootRoutes::Connect4}>{ "Connect4" }</Link<RootRoutes>></li>
                <li class = "header-item"><Link<RootRoutes> to={RootRoutes::TootOtto}>{ "TootOtto" }</Link<RootRoutes>></li>
        </ul>

        <Switch<RootRoutes> render={switch} />
        </BrowserRouter>
    </div>
}
}
