use crate::core::types::recipe::Ingredient;
use crate::modals::Modal;
use yew::prelude::*;

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
            <></>
        </Modal>
    }
}
