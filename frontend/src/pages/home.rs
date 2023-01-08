use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <h1 class={classes!("text-3xl", "font-bold")}>{ "Hello World" }</h1>
    }
}
