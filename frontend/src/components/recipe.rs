use yew::prelude::*;

use crate::{
    core::types,
    modals::{self, ModalController},
};

#[derive(Properties, PartialEq)]
pub struct IngredientsProps {
    #[prop_or_default]
    pub classes: Classes,
    pub items: Vec<types::recipe::Ingredient>,
}

#[function_component(Ingredients)]
pub fn ingredients(props: &IngredientsProps) -> Html {
    html! {
        <div class={props.classes.clone()}>
            <h2 class="text-xl font-bold mb-2">{"Ingredients"}</h2>
            <table class="table table-compact table-zebra w-full">
                <thead>
                    <tr>
                        <th>{"Amount"}</th>
                        <th>{"Unit Type"}</th>
                        <th>{"Name"}</th>
                        <th>{"Notes"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        for props.items.iter().map(|ingredient| {
                            html!{
                                <tr>
                                    <td class="whitespace-normal">{&ingredient.amount}</td>
                                    <td class="whitespace-normal">{&ingredient.unit_type}</td>
                                    <td class="whitespace-normal">{&ingredient.name}</td>
                                    <td class="whitespace-normal">{&ingredient.description.clone().unwrap_or_default()}</td>
                                </tr>
                            }
                        })
                    }
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct StepsProps {
    #[prop_or_default]
    pub classes: Classes,
    pub items: Vec<types::recipe::Step>,
}

#[function_component(Steps)]
pub fn steps(props: &StepsProps) -> Html {
    html! {
        <div class={props.classes.clone()}>
            <h2 class="text-xl font-bold mb-2">{"Steps"}</h2>
            <ul>
            {
                for props.items.iter().enumerate().map(|(i, step)| {
                    html!{
                        <li class="mb-2">
                            <h2 class="text-l font-bold mb-2">{&step.title.clone().unwrap_or(format!("Step {}", i+1))}</h2>
                            <p>{&step.description}</p>
                        </li>
                    }
                })
            }
            </ul>
        </div>
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

    let on_edit_title_click = {
        let modal_html_state = modal_html_state.clone();
        let recipe = (*recipe_state).clone();
        Callback::from(move |_: MouseEvent| {
            modal_html_state.set(Some(html! {
                <modals::recipe::EditTitle title={recipe.title.clone()} onclose={title_modal_closed.clone()}/>
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
                <h1 class="text-2xl font-bold mb-2">{(*recipe_state).title.clone()}<button class="btn" onclick={on_edit_title_click}>{"Edit"}</button></h1>
            </div>
            <p class="mb-4 p-4 rounded bg-base-200">{(*recipe_state).short_description.clone()}</p>
            <p class="mb-4 p-4 rounded bg-base-200">{(*recipe_state).long_description.clone()}</p>
            <div class="flex flex-col md:flex-row gap-4">
                <Ingredients classes="basis-full md:basis-3/4 lg:basis-11/12 p-4 rounded bg-base-200" items={(*recipe_state).ingredients.clone()}/>
                <Steps classes="w-full p-4 rounded bg-base-200" items={(*recipe_state).steps.clone()}/>
            </div>
        </div>
        </>
    }
}
