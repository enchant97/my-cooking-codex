use yew::prelude::*;

use crate::{
    components::drawer,
    core::effects::{use_login_redirect_effect, LoginState},
    Route,
};

#[function_component(Home)]
pub fn home() -> Html {
    use_login_redirect_effect(LoginState::HasLogin, Route::Login);

    html! {
        <drawer::Drawer r#for="main-drawer">
            <drawer::DrawerContent header=true>
                <h1 class={classes!("text-3xl", "font-bold")}>{ "Hello World" }</h1>
            </drawer::DrawerContent>
            <drawer::DrawerDraw r#for="main-drawer">
                <a>{"Sidebar Item 1"}</a>
                <a>{"Sidebar Item 2"}</a>
            </drawer::DrawerDraw>
        </drawer::Drawer>
    }
}
