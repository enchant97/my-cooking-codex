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
