use yew::prelude::*;
use yew_router::prelude::*;

mod contexts;
mod core;
mod pages;

use crate::contexts::login::{CurrentLogin, CurrentLoginContext};
use crate::pages::{Home, Login};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
    let login_context = use_reducer(|| CurrentLogin::new());
    html! {
        <ContextProvider<CurrentLoginContext> context={login_context}>
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </ContextProvider<CurrentLoginContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
