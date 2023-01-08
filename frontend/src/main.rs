use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

use crate::pages::{Home, Login};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home></Home> },
        Route::Login => html! {<Login></Login>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
