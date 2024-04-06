use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html{
    html!{
        // <div style={{ textAlign: 'center' }}>
        <>
        <h2>{"Welcome!"}</h2>
        <h3>{"Here are some instructions:"}</h3>  
        <h3>{"Have fun!"}</h3>  
        </>
        // </div>
    }
}