use crate::contexts::prelude::push_toast;
use crate::core::handlers::{api_error_to_toast, logout_on_401};
use crate::modals::Modal;
use crate::{
    contexts::{login::use_login, prelude::use_toasts},
    core::types::recipe::{Info, InfoYields, UpdateRecipe},
};
use web_sys::HtmlInputElement;
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
pub struct YieldInputProps {
    pub yields: InfoYields,
    pub onchange: Callback<InfoYields>,
}

#[function_component(YieldInput)]
pub fn yield_input(props: &YieldInputProps) -> Html {
    let yield_state = use_state(|| props.yields.clone());

    let on_value_input = {
        let onchange_callback = props.onchange.clone();
        let yield_state = yield_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let v = match input.value().parse::<usize>() {
                Ok(v) => v,
                Err(_) => return,
            };
            let mut yield_v = (*yield_state).clone();
            yield_v.value = v;
            yield_state.set(yield_v.clone());
            onchange_callback.emit(yield_v);
        })
    };

    let on_type_input = {
        let onchange_callback = props.onchange.clone();
        let yield_state = yield_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut yield_v = (*yield_state).clone();
            yield_v.unit_type = input.value();
            yield_state.set(yield_v.clone());
            onchange_callback.emit(yield_v);
        })
    };

    html! {
        <div class="flex">
            <label class="input-group w-auto">
                <span>{"Amount"}</span>
                <input
                    class="input input-bordered w-24"
                    oninput={on_value_input}
                    value={(*yield_state).clone().value.to_string()}
                    type="number" min=1 required=true
                />
            </label>
            <label class="input-group">
                <span>{"Type"}</span>
                <input
                    class="input input-bordered w-full"
                        oninput={on_type_input}
                    value={(*yield_state).clone().unit_type.clone()}
                    type="text" list="units" required=true
                />
                <UnitSelector id="units" />
            </label>
        </div>
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

    let on_save = {
        let id = props.id.to_string();
        let on_close_callback = props.onclose.clone();
        let info_state = info_state.clone();
        let is_loading_state = is_loading_state.clone();
        Callback::from(move |_| {
            let login_ctx = login_ctx.clone();
            let toasts_ctx = toasts_ctx.clone();
            let api = login_ctx.http_api.clone().unwrap();
            let id = id.clone();
            let on_close_callback = on_close_callback.clone();
            let info = (*info_state).clone();
            let is_loading_state = is_loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                is_loading_state.set(true);
                let result = api
                    .patch_update_recipe(
                        id,
                        &UpdateRecipe {
                            info: info.clone(),
                            ..Default::default()
                        },
                    )
                    .await;
                is_loading_state.set(false);
                match result {
                    Ok(_) => {
                        on_close_callback.emit(Some(info));
                    }
                    Err(e) => {
                        push_toast(&toasts_ctx, api_error_to_toast(&e, "saving recipe info"));
                        logout_on_401(&e, &login_ctx);
                    }
                };
            });
        })
    };

    let on_cancel = {
        let on_close_callback = props.onclose.clone();
        Callback::from(move |_| {
            on_close_callback.emit(None);
        })
    };

    let on_yield_change = {
        let info_state = info_state.clone();
        Callback::from(move |new_yields: InfoYields| {
            let mut info = (*info_state).clone();
            info.yields = Some(new_yields);
            info_state.set(info);
        })
    };

    html! {
        <Modal title={"Edit Info"} oncancel={on_cancel} onsave={on_save} loading={*is_loading_state}>
            <h2 class="text-lg mb-2">{"Serving Size"}</h2>
            <YieldInput
                yields={info_state.yields.clone().unwrap_or_default()}
                onchange={on_yield_change}
            />
        </Modal>
    }
}
