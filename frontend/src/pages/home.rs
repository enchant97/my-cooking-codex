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
                <h1 class={classes!("text-3xl", "font-bold")}>{ "Home" }</h1>
            </drawer::DrawerContent>
            <drawer::DrawerDraw r#for="main-drawer">
                <drawer::DrawerLink to={Route::Home} active=true>{"Home"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::Recipes}>{"Recipes"}</drawer::DrawerLink>
            </drawer::DrawerDraw>
        </drawer::Drawer>
    }
}
