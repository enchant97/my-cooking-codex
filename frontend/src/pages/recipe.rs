use yew::prelude::*;
use yew_hooks::prelude::{use_async_with_options, UseAsyncOptions};

use crate::contexts::prelude::{push_toast, use_login, use_toasts};
use crate::core::handlers::{api_error_to_toast, logout_on_401};
use crate::{
    components::{drawer, recipe::RecipeContent},
    core::effects::{use_login_redirect_effect, LoginState},
    Route,
};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub id: AttrValue,
}

#[function_component(Recipe)]
pub fn recipe(props: &RecipeProps) -> Html {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();

    let get_recipe = {
        let id = props.id.to_string();
        let login_ctx = login_ctx.clone();
        let api = login_ctx.http_api.clone();
        use_async_with_options(
            async move {
                let api = api.expect("expected api to exist");
                match api.get_recipe_by_id(id).await {
                    Ok(v) => Ok(v),
                    Err(err) => {
                        push_toast(&toasts_ctx, api_error_to_toast(&err, "loading recipe"));
                        logout_on_401(&err, &login_ctx);
                        Err(err)
                    }
                }
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    use_login_redirect_effect(LoginState::HasLogin, Route::Login);

    html! {
        <drawer::Drawer r#for="main-drawer">
            <drawer::DrawerContent header=true>
                if !get_recipe.loading && get_recipe.error.is_none() && get_recipe.data.is_some() {
                    <RecipeContent
                        recipe={get_recipe.data.as_ref().unwrap().clone()}
                        media_url={login_ctx.login.as_ref().unwrap().media_url.clone()}
                    />
                }
            </drawer::DrawerContent>
            <drawer::DrawerDraw r#for="main-drawer">
                <drawer::DrawerLink to={Route::Home}>{"Home"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::NewRecipe}>{"New Recipe"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::Recipes}>{"Recipes"}</drawer::DrawerLink>
            </drawer::DrawerDraw>
        </drawer::Drawer>
    }
}
