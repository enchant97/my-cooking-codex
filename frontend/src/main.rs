use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class={classes!("text-3xl", "font-bold")}>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
