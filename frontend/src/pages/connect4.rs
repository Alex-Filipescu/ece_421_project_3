use yew::prelude::*;
use crate::components::connect4Grid::Grid;

#[function_component(Connect4)]
pub fn connect4() -> Html{
    html!{
        // <div style={{ textAlign: 'center' }}>
        <>
       <Grid>
       </Grid>
        </>
        // </div>
    }
}