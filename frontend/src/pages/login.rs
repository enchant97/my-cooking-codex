use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <h1 class={classes!("text-3xl", "font-bold")}>{ "Please Login" }</h1>
    }
}
