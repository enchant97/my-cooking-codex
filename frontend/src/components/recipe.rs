use yew::prelude::*;

use crate::{
    core::{types, Fraction},
    modals::{self, ModalController},
};

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
                                <td class="whitespace-normal">{format!("{} {}", Fraction::from(ingredient.amount).to_string(), {&ingredient.unit_type})}</td>
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
                        <pre>{&step.description}</pre>
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
            <div class="mb-4 p-4 rounded bg-base-200">
                if (*recipe_state).main_image_id.is_some() {
                    <figure class="h-64 w-full mb-4">
                        <img
                            class="object-cover w-full h-full rounded"
                            src={format!("{}/recipe-image/{}", props.media_url, (*recipe_state).main_image_id.as_ref().unwrap())}
                        />
                    </figure>
                }
                <h1 class="flex text-2xl font-bold mb-2">
                    <span class="mr-auto">{(*recipe_state).title.clone()}</span>
                    <button class="btn" onclick={on_edit_title_click}>{"Edit"}</button>
                </h1>
            </div>
            <div class="mb-4 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">{"Description"}</h2>
                    <button class="btn" onclick={on_edit_description_click}>{"Edit"}</button>
                </div>
                <p>{(*recipe_state).short_description.clone()}</p>
            </div>
            <div class="mb-4 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">{"Notes"}</h2>
                    <button class="btn" onclick={on_edit_long_description_click}>{"Edit"}</button>
                </div>
                <pre>{(*recipe_state).long_description.clone()}</pre>
            </div>
            <div class="flex flex-col md:flex-row gap-4">
                <div class="basis-full md:basis-3/4 lg:basis-11/12 p-4 rounded bg-base-200">
                    <div class="flex mb-2">
                        <h2 class="text-xl font-bold mr-auto">{"Ingredients"}</h2>
                        <button class="btn" onclick={on_edit_ingredients_click}>{"Edit"}</button>
                    </div>
                    <Ingredients items={(*recipe_state).ingredients.clone()}/>
                </div>
                <div class="w-full p-4 rounded bg-base-200">
                    <div class="flex mb-2">
                        <h2 class="text-xl font-bold mr-auto">{"Steps"}</h2>
                        <button class="btn" onclick={on_edit_steps_click}>{"Edit"}</button>
                    </div>
                    <Steps items={(*recipe_state).steps.clone()}/>
                </div>
            </div>
        </div>
        </>
    }
}
