use yew::prelude::*;

use crate::core::types;

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
    html! {
        <div class={props.classes.clone()}>
            <div class="mb-4 p-4 rounded bg-base-200">
                if props.recipe.main_image_id.is_some() {
                    <figure class="h-64 w-full mb-4">
                        <img
                            class="object-cover w-full h-full rounded"
                            src={format!("{}/recipe-image/{}", props.media_url, props.recipe.main_image_id.as_ref().unwrap())}
                        />
                    </figure>
                }
                <h1 class="text-2xl font-bold mb-2">{props.recipe.title.clone()}</h1>
            </div>
            <p class="mb-4 p-4 rounded bg-base-200">{props.recipe.short_description.clone()}</p>
            <p class="mb-4 p-4 rounded bg-base-200">{props.recipe.long_description.clone()}</p>
            <div class="flex flex-col md:flex-row gap-4">
                <Ingredients classes="basis-full md:basis-3/4 lg:basis-11/12 p-4 rounded bg-base-200" items={props.recipe.ingredients.clone()}/>
                <Steps classes="w-full p-4 rounded bg-base-200" items={props.recipe.steps.clone()}/>
            </div>
        </div>
    }
}
