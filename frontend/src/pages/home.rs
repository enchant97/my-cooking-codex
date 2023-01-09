use yew::prelude::*;

use crate::{
    components::Header,
    core::effects::{use_login_redirect_effect, LoginState},
    Route,
};

#[function_component(Home)]
pub fn home() -> Html {
    use_login_redirect_effect(LoginState::HasLogin, Route::Login);

    html! {
        <div class="drawer drawer-mobile">
        <input id="main-drawer" type="checkbox" class="drawer-toggle" />
        <div class="drawer-content">
            <Header/>
            <div class="pt-3 px-3">
                <h1 class={classes!("text-3xl", "font-bold")}>{ "Hello World" }</h1>
            </div>
        </div>
        <div class="drawer-side">
          <label for="main-drawer" class="drawer-overlay"></label>
          <ul class="menu p-4 w-80 bg-base-200 text-base-content">
            <li><a>{"Sidebar Item 1"}</a></li>
            <li><a>{"Sidebar Item 2"}</a></li>
          </ul>
        </div>
      </div>
    }
}
