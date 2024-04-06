use std::{cell::RefCell, rc::Rc, sync::Mutex};

use yew::{prelude::*};
use crate::components::cell::Cell;



lazy_static! {
    static ref CELLS: Mutex<Vec<char>> = Mutex::new(vec![' '; 6]);
}

#[derive(Properties, Clone, PartialEq)]
pub struct ColProps {
    pub index: usize,
    pub on_click: Callback<usize>, // Callback to pass the column index to the parent
}

#[function_component]
pub fn Col(props: &ColProps)->Html{
    let mut cells = vec![' '; 6]; // Use char instead of String
    let mut cell_state = use_state(||vec![' '; 6]);    
    let onclick = {
       
        let mut cells_clone = cell_state.to_vec();
        for c in cells_clone.iter_mut().rev(){
            if *c==' '{
                *c = 'X'; // Change the cell to 'X' if it's empty
                break
            }
        }

        let cell_state = cell_state.clone();
        // let props_clone = props.clone();
        let on_click = props.on_click.clone();
        let col_index = props.index;

        Callback::from(move |_| 
            {
            on_click.emit(col_index);
            cell_state.set(cells_clone.to_vec()); 
            }
        )
    };

    html! {
        <div class="column" {onclick}>
        <Cell index=0 value={*cell_state.get(0).unwrap()}/>
        <Cell index=1 value = {*cell_state.get(1).unwrap()}/>
        <Cell index=2 value = {*cell_state.get(2).unwrap()}/>
        <Cell index=3 value = {*cell_state.get(3).unwrap()}/>
        <Cell index=4 value ={*cell_state.get(4).unwrap()}/>
        <Cell index=5 value = {*cell_state.get(5).unwrap()}/>

        </div>
    }
}

