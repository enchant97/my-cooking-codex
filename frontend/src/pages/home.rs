use yew::prelude::*;

use crate::{
    core::effects::{use_login_redirect_effect, LoginState},
    Route,
};

#[function_component(Home)]
pub fn home() -> Html {
    use_login_redirect_effect(LoginState::HasLogin, Route::Login);

    html! {
        <h1 class={classes!("text-3xl", "font-bold")}>{ "Hello World" }</h1>
    }
}
