use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use crate::components::drawer;
use crate::contexts::toasts::{Toast, ToastChange};
use crate::contexts::{CurrentLoginContext, ToastsContext};
use crate::core::effects::{use_login_redirect_effect, LoginState};
use crate::core::types;
use crate::Route;

#[function_component(NewRecipe)]
pub fn new_recipe() -> Html {
    let login_ctx = use_context::<CurrentLoginContext>().unwrap();
    let toasts_ctx = use_context::<ToastsContext>().unwrap();
    let navigator = use_navigator().unwrap();

    let title_state = use_state(String::default);

    use_login_redirect_effect(LoginState::HasLogin, Route::Home);

    // create a new recipe from form values
    let create_new_recipe = {
        let api = (*login_ctx).http_api.clone();
        let title = (*title_state).clone();

        use_async(async move {
            api.unwrap()
                .post_new_recipe(&types::recipe::CreateRecipe {
                    title,
                    short_description: None,
                    long_description: None,
                    tags: vec![],
                    ingredients: vec![],
                    steps: vec![],
                })
                .await
        })
    };

    // new recipe value changed
    {
        let create_new_recipe = create_new_recipe.clone();
        use_effect_with_deps(
            move |response| {
                if response.loading {
                    return;
                }
                match &response.error {
                    Some(_) => {
                        // TODO handle the actual errors
                        toasts_ctx.dispatch(ToastChange::Push(Toast {
                            message: "failed recipe creation!",
                        }));
                    }
                    None => match &response.data {
                        Some(recipe) => {
                            gloo::console::debug!(format!("new recipe created: '{:?}'", recipe));
                            navigator.push(&Route::Recipes);
                        }
                        None => (),
                    },
                };
            },
            create_new_recipe,
        );
    }

    let on_submit = {
        let create_new_recipe = create_new_recipe.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            create_new_recipe.run();
        })
    };
    let on_title_input = {
        let title_state = title_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                title_state.set(input.value());
            }
        })
    };

    html! {
        <drawer::Drawer r#for="main-drawer">
            <drawer::DrawerContent header=true>
                <div class="p-4 rounded bg-base-200">
                    <h1 class={"text-3xl font-bold mb-2"}>{ "New Recipe" }</h1>
                    <form onsubmit={on_submit} class="max-w-xs">
                        <div class="form-control mb-6">
                            <label for="recipe-title" class="label"><span class="label-text">{ "Recipe Title" }</span></label>
                            <input
                                oninput={on_title_input}
                                id="recipe-title"
                                type="text"
                                class="input input-bordered"
                                required=true
                            />
                        </div>
                        <div class="form-control">
                            if create_new_recipe.loading {
                                <button type="submit" disabled=true class="btn loading">{"Loading"}</button>
                            } else {
                                <button type="submit" class="btn btn-primary">{"Create"}</button>
                            }
                        </div>
                    </form>
                </div>
            </drawer::DrawerContent>
            <drawer::DrawerDraw r#for="main-drawer">
                <drawer::DrawerLink to={Route::Home}>{"Home"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::NewRecipe} active=true>{"New Recipe"}</drawer::DrawerLink>
                <drawer::DrawerLink to={Route::Recipes}>{"Recipes"}</drawer::DrawerLink>
            </drawer::DrawerDraw>
        </drawer::Drawer>
    }
}
