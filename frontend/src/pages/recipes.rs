use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    components::{card_grid, drawer},
    contexts::prelude::use_login,
    core::{
        effects::{use_login_redirect_effect, LoginState},
        types::recipe,
    },
    Route,
};

#[function_component(Recipes)]
pub fn recipes() -> Html {
    let login_ctx = use_login().unwrap();

    let recipes_state: UseStateHandle<Vec<recipe::Recipe>> = use_state(Vec::default);

    let recipes = (*recipes_state).clone();

    use_login_redirect_effect(LoginState::HasLogin, crate::Route::Login);
    {
        let login_ctx = login_ctx.clone();
        use_effect_with_deps(
            move |_| {
                let api = match &login_ctx.http_api {
                    Some(v) => v.clone(),
                    None => return,
                };
                wasm_bindgen_futures::spawn_local(async move {
                    let new_recipes = api.get_recipes().await.unwrap();
                    recipes_state.set(new_recipes);
                });
            },
            (),
        );
    }

    html! {
        <drawer::Drawer r#for="main-drawer">
            <drawer::DrawerContent header=true>
                <div class="p-4 rounded bg-base-200">
                    <h1 class={classes!("text-3xl", "font-bold", "mb-2")}>{ "Recipes" }</h1>
                    <card_grid::Grid>
                        {
                            for recipes.iter().map(|recipe| {
                            let image_src = match recipe.has_image {
                                true => Some(format!("{}/recipe-image/{}", login_ctx.login.as_ref().unwrap().media_url, recipe.id)),
                                false => None,
                            };
                            html!{
                                <card_grid::GridItem title={recipe.title.clone()} image_src={image_src}>
                                    <Link<Route> to={Route::Recipe { id: recipe.id.clone() }} classes="btn">{"View"}</Link<Route>>
                                </card_grid::GridItem>
                            }
                        })}
                    </card_grid::Grid>
                </div>
            </drawer::DrawerContent>
            <drawer::DrawerDraw r#for="main-drawer">
                <drawer::DrawerLink to={Route::Home}>{"Home"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::NewRecipe}>{"New Recipe"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::Recipes} active=true>{"Recipes"}</drawer::DrawerLink>
            </drawer::DrawerDraw>
        </drawer::Drawer>
    }
}
