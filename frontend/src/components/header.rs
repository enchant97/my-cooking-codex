use yew::{
    prelude::{classes, function_component, html, Html},
    use_context,
};
use yew_router::prelude::Link;

use crate::{contexts::CurrentLoginContext, Route};

#[function_component(Header)]
pub fn header() -> Html {
    let login_ctx = use_context::<CurrentLoginContext>().unwrap();
    let current_login = login_ctx.inner.to_owned();
    html! {
        <div class="navbar bg-neutral text-neutral-content mb-4">
        <div class="flex-none">
            <button class="btn btn-square btn-ghost">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
            </button>
        </div>
        <div class="flex-1">
            <a class="btn btn-ghost normal-case text-xl">{"Recipes"}</a>
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
