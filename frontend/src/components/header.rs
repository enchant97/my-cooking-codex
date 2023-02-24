use yew::prelude::{classes, function_component, html, Html};
use yew_router::prelude::Link;

use crate::{contexts::prelude::use_login, Route, core::APP_TITLE};

#[function_component(Header)]
pub fn header() -> Html {
    let login_ctx = use_login().unwrap();
    let current_login = login_ctx.login.to_owned();
    html! {
        <div class="navbar bg-neutral text-neutral-content">
        <div class="flex-none">
            <label class="btn btn-square btn-ghost drawer-button lg:hidden" for="main-drawer">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
            </label>
        </div>
        <div class="flex-1">
            <a class="btn btn-ghost normal-case text-xl">{APP_TITLE}</a>
        </div>
        <div class="flex-none">
            if current_login.is_none() {
                <Link<Route> to={Route::Login} classes={classes!("btn", "btn-ghost")}>{"Login"}</Link<Route>>
            } else{
                <Link<Route> to={Route::Logout} classes={classes!("btn", "btn-ghost")}>{"Logout"}</Link<Route>>
            }
        </div>
      </div>
    }
}
