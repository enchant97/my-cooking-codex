use crate::core::types::recipe::{UpdateIngredient, UpdateRecipe};
use crate::modals::Modal;
use crate::{contexts::login::use_login, core::types::recipe::Ingredient};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnitSelectorProps {
    pub id: &'static str,
}

#[function_component(UnitSelector)]
pub fn unit_selector(props: &UnitSelectorProps) -> Html {
    html! {
        <datalist id={props.id}>
            <option value="g" />
            <option value="kg" />
            <option value="ml" />
            <option value="l" />
            <option value="tsp" />
            <option value="tbsp" />
            <option value="cup" />
            <option value="oz" />
            <option value="lb" />
            <option value="pinch" />
            <option value="dash" />
            <option value="slice" />
            <option value="can" />
            <option value="bottle" />
            <option value="jar" />
            <option value="head" />
            <option value="stalk" />
            <option value="bunch" />
            <option value="handful" />
        </datalist>
    }
}

#[derive(Properties, PartialEq)]
pub struct EditIngredientProps {
    pub len: usize,
    pub index: usize,
    pub ingredient: Ingredient,
    pub on_input: Callback<(usize, Ingredient)>,
    pub on_delete: Callback<usize>,
}

#[function_component(EditIngredient)]
pub fn recipe_ingredient(props: &EditIngredientProps) -> Html {
    let ingredient_state = use_state(|| props.ingredient.clone());

    let on_delete = {
        let on_delete_callback = props.on_delete.clone();
        let index = props.index;
        Callback::from(move |_| {
            on_delete_callback.emit(index);
        })
    };

    let on_name_input = {
        let on_input_callback = props.on_input.clone();
        let index = props.index;
        let ingredient_state = ingredient_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut ingredient = (*ingredient_state).clone();
                ingredient.name = input.value();
                ingredient_state.set(ingredient.clone());
                on_input_callback.emit((index, ingredient));
            }
        })
    };

    let on_amount_input = {
        let on_input_callback = props.on_input.clone();
        let index = props.index;
        let ingredient_state = ingredient_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut ingredient = (*ingredient_state).clone();
                ingredient.amount = input.value().parse().unwrap_or(0);
                ingredient_state.set(ingredient.clone());
                on_input_callback.emit((index, ingredient));
            }
        })
    };

    let on_unit_input = {
        let on_input_callback = props.on_input.clone();
        let index = props.index;
        let ingredient_state = ingredient_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let mut ingredient = (*ingredient_state).clone();
                ingredient.unit_type = input.value();
                ingredient_state.set(ingredient.clone());
                on_input_callback.emit((index, ingredient));
            }
        })
    };

    let on_description_input = {
        let on_input_callback = props.on_input.clone();
        let index = props.index;
        let ingredient_state = ingredient_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                let mut ingredient = (*ingredient_state).clone();
                if input.value().is_empty() {
                    ingredient.description = None;
                } else {
                    ingredient.description = Some(input.value());
                }
                ingredient_state.set(ingredient.clone());
                on_input_callback.emit((index, ingredient));
            }
        })
    };

    html! {
        <div class="mb-4 p-4 rounded bg-base-200">
            <div class="grid grid-cols-[auto_3rem] gap-2 mb-2">
                <input
                    class="input input-bordered w-full"
                    oninput={on_name_input}
                    value={props.ingredient.name.clone()}
                    type="text"
                    placeholder="name..."
                    required=true
                />
                <button class="btn" onclick={on_delete}>{"X"}</button>
            </div>
            <div class="grid grid-cols-[8rem_auto] gap-2 mb-2">
                <input
                    class="input input-bordered w-full"
                    oninput={on_amount_input}
                    value={format!("{}", props.ingredient.amount)}
                    type="number"
                    placeholder="amount..."
                    required=true
                />
                <input
                    class="input input-bordered w-full"
                    oninput={on_unit_input}
                    value={props.ingredient.unit_type.clone()}
                    type="text"
                    placeholder="unit..."
                    list="units"
                    required=true
                />
                <UnitSelector id="units" />
            </div>
            <textarea
                class="textarea textarea-bordered w-full"
                oninput={on_description_input}
                value={props.ingredient.description.clone()}
                placeholder="notes..."
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct EditIngredientsProps {
    pub id: String,
    pub ingredients: Vec<Ingredient>,
    pub onclose: Callback<Option<Vec<Ingredient>>>,
}

#[function_component(EditIngredients)]
pub fn recipe_ingredients(props: &EditIngredientsProps) -> Html {
    let login_ctx = use_login().unwrap();
    let ingredients_state = use_state(|| props.ingredients.clone());
    let is_loading_state = use_state(bool::default);

    let on_save = {
        let id = props.id.to_string();
        let on_close_callback = props.onclose.clone();
        let api = login_ctx.http_api.clone();
        let is_loading_state = is_loading_state.clone();
        let ingredients_state = ingredients_state.clone();
        Callback::from(move |_| {
            let api = api.clone().unwrap();
            let id = id.clone();
            let on_close_callback = on_close_callback.clone();
            let is_loading_state = is_loading_state.clone();
            let ingredients = (*ingredients_state).clone();
            wasm_bindgen_futures::spawn_local(async move {
                is_loading_state.set(true);
                api.patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        title: None,
                        short_description: None,
                        long_description: None,
                        tags: None,
                        ingredients: Some(
                            ingredients
                                .iter()
                                .map(|i| UpdateIngredient {
                                    name: Some(i.name.clone()),
                                    amount: Some(i.amount),
                                    unit_type: Some(i.unit_type.clone()),
                                    description: i.description.clone(),
                                })
                                .collect(),
                        ),
                        steps: None,
                    },
                )
                .await
                .unwrap();
                is_loading_state.set(false);
                on_close_callback.emit(Some(ingredients));
            });
        })
    };

    let on_cancel = {
        let on_close_callback = props.onclose.clone();
        Callback::from(move |_| {
            on_close_callback.emit(None);
        })
    };

    let on_ingredient_input = {
        let ingredients_state = ingredients_state.clone();
        Callback::from(move |(index, ingredient): (usize, Ingredient)| {
            let mut ingredients = (*ingredients_state).clone();
            ingredients[index] = ingredient;
            ingredients_state.set(ingredients);
        })
    };

    let on_delete_ingredient = {
        let ingredients_state = ingredients_state.clone();
        Callback::from(move |index: usize| {
            let mut ingredients = (*ingredients_state).clone();
            ingredients.remove(index);
            ingredients_state.set(ingredients);
        })
    };

    html! {
        <Modal title={"Edit Ingredients"} oncancel={on_cancel} onsave={on_save}>
            <div class="max-h-[50vh] lg:max-h-[60vh] overflow-y-auto">
                <div>
                    { for (*ingredients_state).clone().iter().enumerate().map(|(i, ingredient)| {
                        html! {
                            <EditIngredient
                                len={(*ingredients_state).clone().len()}
                                index={i}
                                ingredient={ingredient.clone()}
                                on_input={on_ingredient_input.clone()}
                                on_delete={on_delete_ingredient.clone()}
                            />
                        }
                    })}
                </div>
                <button type="button" class="btn w-full">{"Add Ingredient"}</button>
            </div>
        </Modal>
    }
}
