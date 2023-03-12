use crate::modals::Modal;
use crate::{
    contexts::{login::use_login, prelude::use_toasts},
    core::types::recipe::{Info, UpdateRecipe},
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct UnitSelectorProps {
    pub id: &'static str,
}

#[function_component(UnitSelector)]
fn unit_selector(props: &UnitSelectorProps) -> Html {
    html! {
        <datalist id={props.id}>
            <option value="servings" />
            <option value="g" />
            <option value="kg" />
            <option value="ml" />
            <option value="l" />
            <option value="tsp" />
            <option value="tbsp" />
            <option value="cup" />
            <option value="oz" />
            <option value="lb" />
            <option value="can" />
            <option value="bottle" />
            <option value="jar" />
        </datalist>
    }
}

#[derive(Properties, PartialEq)]
pub struct EditInfoProps {
    pub id: AttrValue,
    pub info: Info,
    pub onclose: Callback<Option<Info>>,
}

#[function_component(EditInfo)]
pub fn recipe_info(props: &EditInfoProps) -> Html {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();
    let info_state = use_state(|| props.info.clone());
    let is_loading_state = use_state(bool::default);

    let on_save = { Callback::from(move |_| todo!()) };

    let on_cancel = {
        let on_close_callback = props.onclose.clone();
        Callback::from(move |_| {
            on_close_callback.emit(None);
        })
    };

    html! {
        <Modal title={"Edit Info"} oncancel={on_cancel} onsave={on_save} loading={*is_loading_state}>
            <div class="form-control">
              <label class="label">
                <span class="label-text">{"Yields"}</span>
              </label>
              <label class="input-group">
                <span>{"Amount"}</span>
                <input type="number" min=1 class="input input-bordered w-24" required=true />
                <span>{"Type"}</span>
                <input type="text" class="input input-bordered w-full" list="units" required=true />
                <UnitSelector id="units" />
              </label>
            </div>
        </Modal>
    }
}
