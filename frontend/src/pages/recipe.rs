use yew::prelude::*;
use yew_hooks::prelude::{use_async_with_options, UseAsyncOptions};

use crate::components::recipe::Steps;
use crate::contexts::prelude::use_login;
use crate::{
    components::{drawer, recipe::Ingredients},
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

    let get_recipe = {
        let id = props.id.to_string();
        let api = login_ctx.http_api.clone();
        use_async_with_options(
            async move {
                let api = api.expect("expected api to exist");
                api.get_recipe_by_id(id).await
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    use_login_redirect_effect(LoginState::HasLogin, Route::Login);

    let recipes = get_recipe.data.clone();

    html! {
        <drawer::Drawer r#for="main-drawer">
            <drawer::DrawerContent header=true>
                <div>
                    if !get_recipe.loading && get_recipe.error.is_none() && get_recipe.data.is_some() {
                        <div class="mb-4 p-4 rounded bg-base-200">
                            if get_recipe.data.as_ref().unwrap().main_image_id.is_some() {
                                <figure class="h-64 w-full mb-4">
                                    <img
                                        class="object-cover w-full h-full rounded"
                                        src={format!("{}/recipe-image/{}", login_ctx.login.as_ref().unwrap().media_url, get_recipe.data.as_ref().unwrap().main_image_id.as_ref().unwrap())}
                                    />
                                </figure>
                            }
                            <h1 class="text-2xl font-bold mb-2">{recipes.as_ref().unwrap().title.clone()}</h1>
                        </div>
                        <p class="mb-4 p-4 rounded bg-base-200">{recipes.as_ref().unwrap().short_description.clone()}</p>
                        <p class="mb-4 p-4 rounded bg-base-200">{recipes.as_ref().unwrap().long_description.clone()}</p>
                        <div class="flex flex-col md:flex-row gap-4">
                            <Ingredients classes="basis-full md:basis-3/4 lg:basis-11/12 p-4 rounded bg-base-200" items={recipes.as_ref().unwrap().ingredients.clone()}/>
                            <Steps classes="w-full p-4 rounded bg-base-200" items={recipes.unwrap().steps}/>
                        </div>
                    }
                </div>
            </drawer::DrawerContent>
            <drawer::DrawerDraw r#for="main-drawer">
                <drawer::DrawerLink to={Route::Home}>{"Home"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::NewRecipe}>{"New Recipe"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::Recipes}>{"Recipes"}</drawer::DrawerLink>
            </drawer::DrawerDraw>
        </drawer::Drawer>
    }
}
