use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{
    components::{drawer, loading::LoadingButton, thumbnail_link_grid},
    contexts::prelude::{push_toast, use_login, use_toasts},
    core::{
        effects::{use_login_redirect_effect, LoginState},
        handlers::{api_error_to_toast, logout_on_401},
        types::{query::RecipesFilter, recipe},
    },
    Route,
};

#[function_component(Recipes)]
pub fn recipes() -> Html {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();

    let filters_state = use_state(RecipesFilter::default);
    let recipes_state: UseStateHandle<Vec<recipe::Recipe>> = use_state(Vec::default);

    use_login_redirect_effect(LoginState::HasLogin, crate::Route::Login);

    let current_page = {
        let login_ctx = login_ctx.clone();
        let api = login_ctx.http_api.clone();
        let filters = (*filters_state).clone();
        use_async_with_options(
            async move {
                let api = api.expect("expected api to exist");
                match api.get_recipes(&filters).await {
                    Ok(v) => Ok(v),
                    Err(err) => {
                        push_toast(
                            &toasts_ctx,
                            api_error_to_toast(
                                &err,
                                &format!("loading recipes page {}", &filters.page).to_owned(),
                            ),
                        );
                        logout_on_401(&err, &login_ctx);
                        Err(err)
                    }
                }
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    {
        let filters_state = filters_state.clone();
        let filters_state_ref = filters_state.clone();
        let current_page = current_page.clone();
        use_effect_with_deps(
            move |_| {
                current_page.run();
            },
            filters_state_ref,
        );
    }

    {
        let recipes_state = recipes_state.clone();
        let current_page = current_page.clone();
        let current_page_ref = current_page.clone();
        use_effect_with_deps(
            move |_| {
                if !&current_page.loading
                    && current_page.error.as_ref().is_none()
                    && current_page.data.as_ref().is_some()
                {
                    let mut recipes = (*recipes_state).clone();
                    let current_page_data = current_page.data.as_ref().unwrap().clone();
                    recipes.extend(current_page_data);
                    recipes_state.set(recipes);
                }
            },
            current_page_ref,
        );
    }

    let on_load_more_click = {
        let filters_state = filters_state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut filters = (*filters_state).clone();
            filters.page += 1;
            filters_state.set(filters);
        })
    };

    html! {
        <drawer::Drawer r#for="main-drawer">
            <drawer::DrawerContent header=true>
                <div class="p-4 rounded bg-base-200">
                    <h1 class={classes!("text-3xl", "font-bold", "mb-2")}>{ "Recipes" }</h1>
                    <thumbnail_link_grid::Grid>
                        {
                            for (*recipes_state).clone().iter().map(|recipe| {
                            let image_src = match &recipe.image_id {
                                Some(v) => Some(format!("{}/recipe-image/{}", login_ctx.login.as_ref().unwrap().media_url, v)),
                                None => None,
                            };
                            html!{
                                <thumbnail_link_grid::GridItem
                                    key={recipe.id.clone()}
                                    navigate_to={Route::Recipe { id: recipe.id.clone() }}
                                    title={recipe.title.clone()}
                                    image_src={image_src}
                                />
                            }
                        })}
                    </thumbnail_link_grid::Grid>
                    if current_page.loading {
                       <LoadingButton classes="btn-block" r#type="button" />
                    } else if !current_page.loading && current_page.error.is_none() && current_page.data.is_some() {
                        if current_page.data.as_ref().unwrap().len() == (*filters_state).per_page {
                            <button class="btn btn-block" onclick={on_load_more_click}>{"More"}</button>
                        } else {
                            <div class="text-center">{"Reached Bottom"}</div>
                        }
                    }
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
