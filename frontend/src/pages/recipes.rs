use yew::prelude::*;

use crate::{
    components::{drawer, thumbnail_link_grid},
    contexts::prelude::{push_toast, use_login, use_toasts},
    core::{
        effects::{use_login_redirect_effect, LoginState},
        handlers::{api_error_to_toast, logout_on_401},
        types::recipe,
    },
    Route,
};

#[function_component(Recipes)]
pub fn recipes() -> Html {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();

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
                    match api.get_recipes().await {
                        Ok(v) => recipes_state.set(v),
                        Err(err) => {
                            push_toast(&toasts_ctx, api_error_to_toast(&err, "loading recipes"));
                            logout_on_401(&err, &login_ctx);
                        }
                    };
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
                    <thumbnail_link_grid::Grid>
                        {
                            for recipes.iter().map(|recipe| {
                            let image_src = match &recipe.image_id {
                                Some(v) => Some(format!("{}/recipe-image/{}", login_ctx.login.as_ref().unwrap().media_url, v)),
                                None => None,
                            };
                            html!{
                                <thumbnail_link_grid::GridItem
                                    navigate_to={Route::Recipe { id: recipe.id.clone() }}
                                    title={recipe.title.clone()}
                                    image_src={image_src}
                                />
                            }
                        })}
                    </thumbnail_link_grid::Grid>
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
