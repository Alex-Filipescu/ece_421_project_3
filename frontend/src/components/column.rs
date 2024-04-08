use std::{cell::RefCell, rc::Rc, sync::Mutex};

use crate::components::cell::Cell;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ColProps {
    pub index: usize,
    pub on_click: Callback<usize>, // Callback to pass the column index to the parent
    pub cells: Vec<char>,          // State of cells for this column
    pub cell_num: usize,
}

#[function_component]
pub fn Col(props: &ColProps) -> Html {
    let onclick = {
        let on_click = props.on_click.clone();
        let col_index = props.index;

        Callback::from(move |_| {
            on_click.emit(col_index);
        })
    };

    let cells_html: Html = (0..props.cell_num)
        .map(|cell_index| {
            html! {
                <Cell index={cell_index} value={props.cells[cell_index]} />
            }
        })
        .collect();
    html! {
        <div class="column" {onclick}>
            {cells_html}
        </div>
    }
}
