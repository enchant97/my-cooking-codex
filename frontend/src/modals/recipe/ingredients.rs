use crate::components::input::FractionalNumberInput;
use crate::contexts::prelude::{push_toast, use_toasts};
use crate::core::handlers::{api_error_to_toast, logout_on_401};
use crate::core::types::recipe::{UpdateIngredient, UpdateRecipe};
use crate::modals::Modal;
use crate::{contexts::login::use_login, core::types::recipe::Ingredient};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
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
    let ingredient_state = use_state(|| Ingredient {
        name: String::from(""),
        amount: 0.0,
        unit_type: String::from(""),
        description: None,
    });

    {
        let initial_ingredient = props.ingredient.clone();
        let ingredient_state = ingredient_state.clone();
        use_effect_with_deps(
            move |_| {
                ingredient_state.set(initial_ingredient);
            },
            props.ingredient.clone(),
        );
    }

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
        Callback::from(move |new_value| {
            let mut ingredient = (*ingredient_state).clone();
            ingredient.amount = new_value;
            ingredient_state.set(ingredient.clone());
            on_input_callback.emit((index, ingredient));
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
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
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
                <button class="btn" type="button" onclick={on_delete}>{"X"}</button>
            </div>
            <div class="grid grid-cols-[8rem_auto] gap-2 mb-2">
                <FractionalNumberInput
                    classes="input-bordered w-full"
                    oninput={on_amount_input}
                    value={props.ingredient.amount}
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
            <input
                class="textarea textarea-bordered w-full"
                oninput={on_description_input}
                value={props.ingredient.description.clone().unwrap_or_default()}
                type="text"
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
    let toasts_ctx = use_toasts().unwrap();
    let ingredients_state = use_state(Vec::default);
    let is_loading_state = use_state(bool::default);

    {
        let initial_ingredients = props.ingredients.clone();
        let ingredients_state = ingredients_state.clone();
        use_effect_with_deps(
            move |_| {
                ingredients_state.set(initial_ingredients);
            },
            (),
        );
    }

    let on_save = {
        let id = props.id.to_string();
        let on_close_callback = props.onclose.clone();
        let ingredients_state = ingredients_state.clone();
        Callback::from(move |_| {
            let login_ctx = login_ctx.clone();
            let toasts_ctx = toasts_ctx.clone();
            let api = login_ctx.http_api.clone().unwrap();
            let id = id.clone();
            let on_close_callback = on_close_callback.clone();
            let is_loading_state = is_loading_state.clone();
            let ingredients = (*ingredients_state).clone();
            wasm_bindgen_futures::spawn_local(async move {
                is_loading_state.set(true);
                let result = api
                    .patch_update_recipe(
                        id,
                        &UpdateRecipe {
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
                            ..Default::default()
                        },
                    )
                    .await;
                is_loading_state.set(false);
                match result {
                    Ok(_) => {
                        on_close_callback.emit(Some(ingredients));
                    }
                    Err(e) => {
                        push_toast(
                            &toasts_ctx,
                            api_error_to_toast(&e, "saving recipe ingredients"),
                        );
                        logout_on_401(&e, &login_ctx);
                    }
                }
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

    let on_add_ingredient = {
        let ingredients_state = ingredients_state.clone();
        Callback::from(move |_| {
            let mut ingredients = (*ingredients_state).clone();
            ingredients.push(Ingredient {
                name: String::from(""),
                amount: 0.0,
                unit_type: String::from(""),
                description: None,
            });
            ingredients_state.set(ingredients);
        })
    };

    html! {
        <Modal title={"Edit Ingredients"} oncancel={on_cancel} onsave={on_save}>
            <div class="max-h-[50vh] lg:max-h-[60vh] overflow-y-auto">
                <div>
                    { for (*ingredients_state).clone().iter().enumerate().map(|(i, ingredient)| {
                        html! {<EditIngredient
                            len={(*ingredients_state).clone().len()}
                            index={i}
                            ingredient={ingredient.clone()}
                            on_input={on_ingredient_input.clone()}
                            on_delete={on_delete_ingredient.clone()}
                        />}
                    })}
                </div>
                <button class="btn w-full" onclick={on_add_ingredient} type="button">{"Add Ingredient"}</button>
            </div>
        </Modal>
    }
}
