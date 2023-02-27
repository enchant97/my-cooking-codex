use yew::prelude::*;

use crate::{
    components::drawer,
    components::stats,
    contexts::prelude::{push_toast, use_login, use_toasts},
    core::effects::{use_login_redirect_effect, LoginState},
    core::{
        handlers::{api_error_to_toast, logout_on_401},
        types,
    },
    Route,
};

#[function_component(HomeAccountStats)]
fn home_account_stats() -> HtmlResult {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();

    let stats_state: UseStateHandle<Option<types::stats::AccountStats>> =
        use_state(Option::default);

    {
        let stats_state = stats_state.clone();
        use_effect_with_deps(
            move |_| {
                let api = match &login_ctx.http_api {
                    Some(v) => v.clone(),
                    None => return,
                };
                wasm_bindgen_futures::spawn_local(async move {
                    match api.get_stats().await {
                        Ok(v) => stats_state.set(Some(v)),
                        Err(err) => {
                            push_toast(&toasts_ctx, api_error_to_toast(&err, "loading stats"));
                            logout_on_401(&err, &login_ctx);
                        }
                    };
                });
            },
            (),
        );
    }

    Ok(html! {
        if let Some(stats) = (*stats_state).clone() {
            <stats::Stats>
                <stats::Stat title={"Number Of Users"} value={format!("{}", stats.user_count)}/>
                <stats::Stat title={"Number Of Recipes"} value={format!("{}", stats.recipe_count)}/>
                <stats::Stat title={"Number Of Books"} value={"0"}/>
            </stats::Stats>
        }
    })
}

#[function_component(Home)]
pub fn home() -> Html {
    use_login_redirect_effect(LoginState::HasLogin, Route::Login);

    html! {
        <drawer::Drawer r#for="main-drawer">
            <drawer::DrawerContent header=true>
                <div class="p-4 rounded bg-base-200">
                    <h1 class={classes!("text-3xl", "font-bold", "mb-2")}>{ "Home" }</h1>
                    <h2 class="text-2xl mb-2">{"Your Stats"}</h2>
                    <HomeAccountStats/>
                </div>
            </drawer::DrawerContent>
            <drawer::DrawerDraw r#for="main-drawer">
                <drawer::DrawerLink to={Route::Home} active=true>{"Home"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::NewRecipe}>{"New Recipe"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::Recipes}>{"Recipes"}</drawer::DrawerLink>
            </drawer::DrawerDraw>
        </drawer::Drawer>
    }
}
