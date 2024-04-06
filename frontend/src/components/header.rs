use yew::prelude::*;

use yew_router::prelude::*;
use crate::router::{ RootRoutes, switch };
use stylist::{ style, yew::styled_component, Style };

#[styled_component(Header)]
pub fn header() -> Html {
    html! {
        <div class = "header-container">
            <BrowserRouter>
                    <Link<RootRoutes> to={RootRoutes::Home}>{ "Home" }</Link<RootRoutes>>
                    <Link<RootRoutes> to={RootRoutes::Connect4}>{ "Connect4" }</Link<RootRoutes>>
                    <Link<RootRoutes> to={RootRoutes::TootOtto}>{ "TootOtto" }</Link<RootRoutes>>
            <Switch<RootRoutes> render={switch} /> 
            </BrowserRouter>
        </div>
    }
}
