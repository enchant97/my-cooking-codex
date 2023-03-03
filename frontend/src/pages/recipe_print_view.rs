use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{contexts::login::use_login, core::types::Fraction};

#[derive(Properties, PartialEq)]
pub struct RecipePrintViewProps {
    pub id: AttrValue,
}

#[function_component(RecipePrintView)]
pub fn recipe_print_view(props: &RecipePrintViewProps) -> Html {
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

    html! {
        <div class="p-2" data-theme="light">
            if !get_recipe.loading && get_recipe.error.is_none() && get_recipe.data.is_some() {
                if get_recipe.data.as_ref().unwrap().image_id.is_some() {
                    <figure class="h-64 w-full mb-4">
                        <img
                            class="object-cover w-full h-full rounded"
                            src={format!("{}/recipe-image/{}", {login_ctx.login.as_ref().unwrap().media_url.clone()}, get_recipe.data.as_ref().unwrap().image_id.as_ref().unwrap())}
                        />
                    </figure>
                }
                <h1 class="text-3xl font-bold mb-4">{get_recipe.data.as_ref().unwrap().title.clone()}</h1>
                <div class="mb-4">
                    <h2 class="text-xl font-bold mb-1">{"Description"}</h2>
                    <p>{get_recipe.data.as_ref().unwrap().short_description.clone()}</p>
                </div>
                <div class="mb-4">
                    <h2 class="text-xl font-bold mb-1">{"Notes"}</h2>
                    <pre class="whitespace-normal">{get_recipe.data.as_ref().unwrap().long_description.clone()}</pre>
                </div>
                <div class="mb-4">
                    <h2 class="text-xl font-bold mb-1">{"Ingredients"}</h2>
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
                                for get_recipe.data.as_ref().unwrap().ingredients.iter().map(|ingredient| {
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
                </div>
                <div class="mb-4">
                    <h2 class="text-xl font-bold mb-1">{"Steps"}</h2>
                    <ul>
                    {
                        for get_recipe.data.as_ref().unwrap().steps.iter().enumerate().map(|(i, step)| {
                            html!{
                                <li class="mb-2">
                                    <h2 class="text-l font-bold mb-2">{&step.title.clone().unwrap_or(format!("Step {}", i+1))}</h2>
                                    <pre class="whitespace-normal">{&step.description}</pre>
                                </li>
                            }
                        })
                    }
                    </ul>
                </div>
            } else {
                <div>{"Loading..."}</div>
            }
        </div>
    }
}
