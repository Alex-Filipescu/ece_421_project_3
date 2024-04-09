use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CellProps {
    pub index: usize,
    pub value: char,
    pub user_color: String,
    pub bot_color: String,
}

#[function_component]
pub fn Cell(props: &CellProps) -> Html {

    let color = match props.value {
        'X' => &props.user_color,
        'O' => &props.bot_color,
        _ => "#FFFFFF", 
    };

    html! {
        <div class="cell" style={format!("background-color: {}", color)}><p>{props.value}</p></div>
    }

}
