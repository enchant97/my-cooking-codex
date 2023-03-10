use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    contexts::{
        login::use_login,
        prelude::{push_toast, use_toasts},
    },
    core::{
        handlers::{api_error_to_toast, logout_on_401},
        types::{self, Fraction},
    },
    modals::{self, ModalController},
    Route,
};

#[derive(Properties, PartialEq)]
pub struct RecipeToolbarProps {
    #[prop_or_default]
    pub classes: Classes,
    pub recipe: types::recipe::Recipe,
}

#[function_component(RecipeToolbar)]
pub fn recipe_toolbar(props: &RecipeToolbarProps) -> Html {
    let recipe_state = use_state(|| props.recipe.clone());
    let navigator = use_navigator().unwrap();
    let toasts_ctx = use_toasts().unwrap();
    let login_ctx = use_login().unwrap();

    let on_print_click = {
        let recipe_id = (*recipe_state).clone().id;
        Callback::from(move |_: MouseEvent| {
            let window = gloo::utils::window();
            let print_window = window.open_with_url_and_target_and_features(
                &format!("{}/print", recipe_id),
                "_blank",
                "Recipe Print",
            );
            if let Ok(Some(print_window)) = print_window {
                print_window.open().unwrap();
            }
        })
    };

    let on_delete_click = {
        let recipe_id = (*recipe_state).clone().id;
        Callback::from(move |_: MouseEvent| {
            let recipe_id = recipe_id.clone();
            let navigator = navigator.clone();
            let toasts_ctx = toasts_ctx.clone();
            let login_ctx = login_ctx.clone();
            let api = login_ctx.http_api.clone().unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                match api.delete_recipe(&recipe_id).await {
                    Ok(_) => {
                        navigator.push(&Route::Recipes);
                    }
                    Err(e) => {
                        push_toast(&toasts_ctx, api_error_to_toast(&e, "deleting recipe"));
                        logout_on_401(&e, &login_ctx);
                    }
                };
            });
        })
    };

    html! {
        <div class={classes!(props.classes.clone())}>
            <button class="btn" onclick={on_print_click}>{"Print"}</button>
        <div class="dropdown dropdown-bottom">
            <label tabindex="0" class="btn m-1">{"Remove"}</label>
            <div class="dropdown-content menu bg-base-200 rounded">
                <button
                    tabindex="0"
                    class="btn btn-outline btn-error"
                    onclick={on_delete_click}
                    aria-label={"Confirm Deletion"}>
                    {"Confirm"}
                </button>
            </div>
        </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct InfoProps {
    #[prop_or_default]
    pub classes: Classes,
    pub info: types::recipe::Info,
}

#[function_component(Info)]
pub fn info(props: &InfoProps) -> Html {
    html! {
        <table classes={classes!("table", "w-full", props.classes.clone())}>
            <tbody>
                if let Some(v) = &props.info.yields {
                    <tr>
                        <th>{v.unit_type.clone()}</th>
                        <td>{v.value.to_string()}</td>
                    </tr>
                } else {
                    <tr>
                        <th>{"Servings"}</th>
                        <td>{"0"}</td>
                    </tr>
                }
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct IngredientsProps {
    pub items: Vec<types::recipe::Ingredient>,
}

#[function_component(Ingredients)]
pub fn ingredients(props: &IngredientsProps) -> Html {
    html! {
        <table class="table table-compact table-zebra w-full">
            <thead>
                <tr>
                    <th>{"Amount"}</th>
                    <th>{"Name"}</th>
                    <th>{"Notes"}</th>
                </tr>
            </thead>
            <tbody>
                {
                    for props.items.iter().map(|ingredient| {
                        html!{
                            <tr>
                                <td class="whitespace-normal">{format!("{} {}", Fraction::from(ingredient.amount), {&ingredient.unit_type})}</td>
                                <td class="whitespace-normal">{&ingredient.name}</td>
                                <td class="whitespace-normal">{&ingredient.description.clone().unwrap_or_default()}</td>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct StepsProps {
    pub items: Vec<types::recipe::Step>,
}

#[function_component(Steps)]
pub fn steps(props: &StepsProps) -> Html {
    html! {
        <ul>
        {
            for props.items.iter().enumerate().map(|(i, step)| {
                html!{
                    <li class="mb-2">
                        <h2 class="text-l font-bold mb-2">{&step.title.clone().unwrap_or(format!("Step {}", i+1))}</h2>
                        <pre class="whitespace-normal text-base font-sans">{&step.description}</pre>
                    </li>
                }
            })
        }
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct RecipeContentProps {
    #[prop_or_default]
    pub classes: Classes,
    pub recipe: types::recipe::Recipe,
    pub media_url: AttrValue,
}

#[function_component(RecipeContent)]
pub fn recipe_content(props: &RecipeContentProps) -> Html {
    let modal_html_state: UseStateHandle<Option<Html>> = use_state(Option::default);
    let recipe_state = use_state(|| props.recipe.clone());

    let image_modal_closed = {
        let modal_html_state = modal_html_state.clone();
        let recipe_state = recipe_state.clone();
        Callback::from(move |image_id: Option<Option<String>>| {
            if let Some(image_id) = image_id {
                let mut recipe = (*recipe_state).clone();
                recipe.image_id = image_id;
                recipe_state.set(recipe)
            }
            modal_html_state.set(None);
        })
    };

    let title_modal_closed = {
        let modal_html_state = modal_html_state.clone();
        let recipe_state = recipe_state.clone();
        Callback::from(move |new_title: Option<String>| {
            modal_html_state.set(None);
            if let Some(title) = new_title {
                let mut recipe = (*recipe_state).clone();
                recipe.title = title;
                recipe_state.set(recipe)
            }
        })
    };

    let info_modal_closed = {
        let modal_html_state = modal_html_state.clone();
        let recipe_state = recipe_state.clone();
        Callback::from(move |new_info: Option<types::recipe::Info>| {
            modal_html_state.set(None);
            if let Some(info) = new_info {
                let mut recipe = (*recipe_state).clone();
                recipe.info = info;
                recipe_state.set(recipe)
            }
        })
    };

    let description_modal_closed = {
        let modal_html_state = modal_html_state.clone();
        let recipe_state = recipe_state.clone();
        Callback::from(move |new_description: Option<String>| {
            modal_html_state.set(None);
            if let Some(description) = new_description {
                let mut recipe = (*recipe_state).clone();
                recipe.short_description = Some(description);
                recipe_state.set(recipe)
            }
        })
    };

    let long_description_modal_closed = {
        let modal_html_state = modal_html_state.clone();
        let recipe_state = recipe_state.clone();
        Callback::from(move |new_description: Option<String>| {
            modal_html_state.set(None);
            if let Some(description) = new_description {
                let mut recipe = (*recipe_state).clone();
                recipe.long_description = Some(description);
                recipe_state.set(recipe)
            }
        })
    };

    let ingredients_modal_closed = {
        let modal_html_state = modal_html_state.clone();
        let recipe_state = recipe_state.clone();
        Callback::from(
            move |new_ingredients: Option<Vec<types::recipe::Ingredient>>| {
                modal_html_state.set(None);
                if let Some(ingredients) = new_ingredients {
                    let mut recipe = (*recipe_state).clone();
                    recipe.ingredients = ingredients;
                    recipe_state.set(recipe)
                }
            },
        )
    };

    let steps_modal_closed = {
        let modal_html_state = modal_html_state.clone();
        let recipe_state = recipe_state.clone();
        Callback::from(move |new_steps: Option<Vec<types::recipe::Step>>| {
            modal_html_state.set(None);
            if let Some(steps) = new_steps {
                let mut recipe = (*recipe_state).clone();
                recipe.steps = steps;
                recipe_state.set(recipe)
            }
        })
    };

    let on_edit_image_click = {
        let modal_html_state = modal_html_state.clone();
        let recipe = (*recipe_state).clone();
        Callback::from(move |_: MouseEvent| {
            modal_html_state.set(Some(html! {
                <modals::recipe::SetImage
                    id={recipe.id.clone()}
                    image_id={recipe.image_id.clone()}
                    onclose={image_modal_closed.clone()}
                />
            }));
        })
    };

    let on_edit_title_click = {
        let modal_html_state = modal_html_state.clone();
        let recipe = (*recipe_state).clone();
        Callback::from(move |_: MouseEvent| {
            modal_html_state.set(Some(html! {
                <modals::recipe::EditTitle
                    id={recipe.id.clone()}
                    title={recipe.title.clone()}
                    onclose={title_modal_closed.clone()}
                />
            }));
        })
    };

    let on_edit_info_click = {
        let modal_html_state = modal_html_state.clone();
        let recipe = (*recipe_state).clone();
        Callback::from(move |_: MouseEvent| {
            modal_html_state.set(Some(html! {
                <modals::recipe::EditInfo
                    id={recipe.id.clone()}
                    info={recipe.info.clone()}
                    onclose={info_modal_closed.clone()}
                />
            }));
        })
    };

    let on_edit_description_click = {
        let modal_html_state = modal_html_state.clone();
        let recipe = (*recipe_state).clone();
        Callback::from(move |_: MouseEvent| {
            modal_html_state.set(Some(html! {
                <modals::recipe::EditDescription
                    id={recipe.id.clone()}
                    description={recipe.short_description.clone()}
                    onclose={description_modal_closed.clone()}
                />
            }));
        })
    };

    let on_edit_long_description_click = {
        let modal_html_state = modal_html_state.clone();
        let recipe = (*recipe_state).clone();
        Callback::from(move |_: MouseEvent| {
            modal_html_state.set(Some(html! {
                <modals::recipe::EditLongDescription
                    id={recipe.id.clone()}
                    description={recipe.long_description.clone()}
                    onclose={long_description_modal_closed.clone()}
                />
            }));
        })
    };

    let on_edit_ingredients_click = {
        let modal_html_state = modal_html_state.clone();
        let recipe = (*recipe_state).clone();
        Callback::from(move |_: MouseEvent| {
            modal_html_state.set(Some(html! {
                <modals::recipe::EditIngredients
                    id={recipe.id.clone()}
                    ingredients={recipe.ingredients.clone()}
                    onclose={ingredients_modal_closed.clone()}
                />
            }));
        })
    };

    let on_edit_steps_click = {
        let modal_html_state = modal_html_state.clone();
        let recipe = (*recipe_state).clone();
        Callback::from(move |_: MouseEvent| {
            modal_html_state.set(Some(html! {
                <modals::recipe::EditSteps
                    id={recipe.id.clone()}
                    steps={recipe.steps.clone()}
                    onclose={steps_modal_closed.clone()}
                />
            }));
        })
    };

    html! {
        <>
        <ModalController modal={(*modal_html_state).clone()}/>
        <div class={props.classes.clone()}>
            <div class="mb-4 relative h-64">
                if recipe_state.image_id.is_some() {
                    <img
                        class="object-cover w-full h-full rounded"
                        src={format!("{}/recipe-image/{}", props.media_url, recipe_state.image_id.as_ref().unwrap())}
                    />
                } else {
                    <div class="w-full h-full bg-neutral rounded"></div>
                }
                <div class="flex items-center absolute bottom-0 left-0 p-2 w-full bg-[#000000cc] rounded-b">
                    <h1 class="mr-auto text-2xl font-bold text-slate-300 whitespace-nowrap overflow-hidden text-ellipsis">{recipe_state.title.clone()}</h1>
                    <button class="btn" onclick={on_edit_title_click}>{"Edit"}</button>
                    <button class="btn" onclick={on_edit_image_click}>{"Edit Image"}</button>
                </div>
            </div>
            <RecipeToolbar classes="mb-4 p-4 rounded bg-base-200" recipe={(*recipe_state).clone()} />
            <div class="mb-4 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">{"Info"}</h2>
                    <button class="btn" onclick={on_edit_info_click}>{"Edit"}</button>
                </div>
                <Info info={recipe_state.info.clone()} />
            </div>
            <div class="mb-4 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">{"Description"}</h2>
                    <button class="btn" onclick={on_edit_description_click}>{"Edit"}</button>
                </div>
                <p>{recipe_state.short_description.clone()}</p>
            </div>
            <div class="mb-4 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">{"Notes"}</h2>
                    <button class="btn" onclick={on_edit_long_description_click}>{"Edit"}</button>
                </div>
                <pre class="whitespace-normal text-base font-sans">{recipe_state.long_description.clone()}</pre>
            </div>
            <div class="flex flex-col md:flex-row gap-4">
                <div class="basis-full md:basis-3/4 lg:basis-11/12 p-4 rounded bg-base-200">
                    <div class="flex mb-2">
                        <h2 class="text-xl font-bold mr-auto">{"Ingredients"}</h2>
                        <button class="btn" onclick={on_edit_ingredients_click}>{"Edit"}</button>
                    </div>
                    <Ingredients items={recipe_state.ingredients.clone()}/>
                </div>
                <div class="w-full p-4 rounded bg-base-200">
                    <div class="flex mb-2">
                        <h2 class="text-xl font-bold mr-auto">{"Steps"}</h2>
                        <button class="btn" onclick={on_edit_steps_click}>{"Edit"}</button>
                    </div>
                    <Steps items={recipe_state.steps.clone()}/>
                </div>
            </div>
        </div>
        </>
    }
}
