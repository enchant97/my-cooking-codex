use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    components::{card_grid, drawer},
    contexts::CurrentLoginContext,
    core::{
        effects::{use_login_redirect_effect, LoginState},
        types::recipe,
    },
    Route,
};

#[function_component(Recipes)]
pub fn recipes() -> Html {
    let login_ctx = use_context::<CurrentLoginContext>().unwrap();

    let recipes_state: UseStateHandle<Vec<recipe::Recipe>> = use_state(Vec::default);

    let recipes = (*recipes_state).clone();

    use_login_redirect_effect(LoginState::HasLogin, crate::Route::Login);
    use_effect_with_deps(
        move |_| {
            let recipes_state = recipes_state.clone();
            let api = login_ctx.http_api.clone().unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                let new_recipes = api.get_recipes().await.unwrap();
                recipes_state.set(new_recipes);
            });
        },
        (),
    );

    html! {
        <drawer::Drawer r#for="main-drawer">
            <drawer::DrawerContent header=true>
                <h1 class={classes!("text-3xl", "font-bold")}>{ "Recipes" }</h1>
                <card_grid::Grid>
                    { for recipes.iter().map(|recipe| {
                        html!{
                            <card_grid::GridItem title={recipe.title.clone()}>
                            </card_grid::GridItem>
                        }
                    })}
                </card_grid::Grid>
            </drawer::DrawerContent>
            <drawer::DrawerDraw r#for="main-drawer">
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                <Link<Route> to={Route::Recipes}>{"Recipes"}</Link<Route>>
            </drawer::DrawerDraw>
        </drawer::Drawer>
    }
}
