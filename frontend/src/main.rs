use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod contexts;
mod core;
mod modals;
mod pages;

use crate::contexts::login::{CurrentLogin, CurrentLoginContext};
use crate::contexts::toasts::{Toasts, ToastsContext};
use crate::pages::{Home, Login, Logout, NewRecipe, Recipe, Recipes, Signup};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/recipes")]
    Recipes,
    #[at("/recipe/:id")]
    Recipe { id: String },
    #[at("/new")]
    NewRecipe,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/>},
        Route::Signup => html! {<Signup/>},
        Route::Login => html! {<Login/>},
        Route::Logout => html! {<Logout/>},
        Route::Recipes => html! {<Recipes/>},
        Route::Recipe { id } => html! {<Recipe id={id}/>},
        Route::NewRecipe => html! {<NewRecipe/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    let login_context = use_reducer(CurrentLogin::new);
    let toasts_context = use_reducer(Toasts::new);
    html! {
        <ContextProvider<CurrentLoginContext> context={login_context}>
        <ContextProvider<ToastsContext> context={toasts_context}>
            <div id="modal_host"></div>
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
