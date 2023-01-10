use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod contexts;
mod core;
mod pages;

use crate::contexts::toasts::{Toasts, ToastsContext};
use crate::contexts::login::{CurrentLogin, CurrentLoginContext};
use crate::pages::{Home, Login, Logout};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home></Home> },
        Route::Login => html! {<Login></Login>},
        Route::Logout => html! {<Logout></Logout>},
    }
}

#[function_component(App)]
fn app() -> Html {
    let login_context = use_reducer(|| CurrentLogin::new());
    let toasts_context = use_reducer(Toasts::new);
    html! {
        <ContextProvider<CurrentLoginContext> context={login_context}>
        <ContextProvider<ToastsContext> context={toasts_context}>
            <crate::components::Toasts/>
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </ContextProvider<ToastsContext>>
        </ContextProvider<CurrentLoginContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
