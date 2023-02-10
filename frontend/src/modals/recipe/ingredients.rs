use crate::core::types::recipe::Ingredient;
use crate::modals::Modal;
use yew::prelude::*;

#[function_component(UnitSelector)]
pub fn unit_selector() -> Html {
    html! {
        <datalist id="units">
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
    // pub on_input: Callback<(usize, Ingredient)>,
    // pub on_delete: Callback<usize>,
}

#[function_component(EditIngredient)]
pub fn recipe_ingredient(props: &EditIngredientProps) -> Html {
    html! {
        <div class="mb-4 p-4 rounded bg-base-200">
            <input
                class="input input-bordered w-full mb-2"
                value={props.ingredient.name.clone()}
                type="text"
                placeholder="name..."
                required=true
            />
            <div class="grid grid-cols-[8rem_auto] gap-2 mb-2">
                <input
                    class="input input-bordered w-full"
                    value={format!("{}", props.ingredient.amount)}
                    type="number"
                    placeholder="amount..."
                    required=true
                />
                <input
                    class="input input-bordered w-full"
                    value={props.ingredient.unit_type.clone()}
                    type="text"
                    placeholder="unit..."
                    list="units"
                    required=true
                />
                <UnitSelector />
            </div>
            <textarea
                class="textarea textarea-bordered w-full"
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
    let on_cancel = {
        let on_close_callback = props.onclose.clone();
        Callback::from(move |_| {
            on_close_callback.emit(None);
        })
    };
    html! {
        <Modal title={"Edit Ingredients"} oncancel={on_cancel}>
            <div class="max-h-[50vh] lg:max-h-[60vh] overflow-y-auto">
                <div>
                    { for props.ingredients.iter().enumerate().map(|(index, ingredient)| {
                        html! {
                            <EditIngredient
                                len={props.ingredients.len()}
                                index={index}
                                ingredient={ingredient.clone()}
                            />
                        }
                    })}
                </div>
                <button type="button" class="btn w-full">{"Add Ingredient"}</button>
            </div>
        </Modal>
    }
}
