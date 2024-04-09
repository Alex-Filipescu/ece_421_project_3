use std::{cell::RefCell, rc::Rc, sync::Mutex};

use crate::components::cell::Cell;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ColProps {
    pub index: usize,
    pub on_click: Callback<usize>, // Callback to pass the column index to the parent
    pub cells: Vec<char>,          // State of cells for this column
    pub cell_num: usize,
    pub user_color: String,
    pub bot_color: String,
}

#[function_component]
pub fn Col(props: &ColProps) -> Html {
    let onclick = props.on_click.clone();
    let col_index = props.index;

    let cells_html: Html = props
        .cells
        .iter()
        .enumerate()
        .map(|(index, &value)| {
            html! {
                <Cell index={index} value={value} user_color={props.user_color.clone()} bot_color={props.bot_color.clone()}/>
            }
        })
        .collect();

    html! {
        <div class="column" onclick={move |_| onclick.emit(col_index)}>
            {cells_html}
        </div>
    }
}
