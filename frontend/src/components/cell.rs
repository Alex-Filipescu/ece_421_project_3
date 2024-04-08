use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CellProps {
    pub index: usize,
    pub value: char,
}

#[function_component]
pub fn Cell(props: &CellProps) -> Html {
    html! {
        <div class="cell" ><p>{props.value}</p></div>
    }
}
