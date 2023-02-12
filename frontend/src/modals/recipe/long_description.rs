use crate::{contexts::login::use_login, core::types::recipe::UpdateRecipe};

use crate::modals::Modal;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditLongDescriptionProps {
    pub id: String,
    pub description: Option<AttrValue>,
    pub onclose: Callback<Option<String>>,
}

#[function_component(EditLongDescription)]
pub fn recipe_long_description(props: &EditLongDescriptionProps) -> Html {
    let login_ctx = use_login().unwrap();
    let description_state = use_state(Option::default);
    let is_loading_state = use_state(bool::default);

    {
        let initial_description = props.description.clone();
        let description_state = description_state.clone();
        use_effect_with_deps(
            move |_| {
                description_state.set(initial_description.clone());
            },
            (),
        );
    }

    let on_save = {
        let api = login_ctx.http_api.clone();
        let id = props.id.to_string();
        let on_close_callback = props.onclose.clone();
        let description_state = description_state.clone();
        let is_loading_state = is_loading_state.clone();
        Callback::from(move |_| {
            let api = api.clone().unwrap();
            let id = id.clone();
            let on_close_callback = on_close_callback.clone();
            let description = (*description_state).clone();
            let is_loading_state = is_loading_state.clone();
            match description {
                Some(v) => {
                    wasm_bindgen_futures::spawn_local(async move {
                        is_loading_state.set(true);
                        api.patch_update_recipe(
                            id,
                            &UpdateRecipe {
                                title: None,
                                short_description: None,
                                long_description: Some(v.to_string()),
                                tags: None,
                                ingredients: None,
                                steps: None,
                            },
                        )
                        .await
                        .unwrap();
                        is_loading_state.set(false);
                        on_close_callback.emit(Some(v.to_string()));
                    });
                }
                None => on_close_callback.emit(None),
            }
        })
    };

    let on_cancel = {
        let on_close_callback = props.onclose.clone();
        Callback::from(move |_| {
            on_close_callback.emit(None);
        })
    };

    let on_description_input = {
        let description_state = description_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                description_state.set(Some(input.value().into()));
            }
        })
    };

    html! {
        <Modal title={"Edit Notes"} oncancel={on_cancel} onsave={on_save} loading={*is_loading_state}>
            <textarea oninput={on_description_input} value={(*description_state).clone()} class="my-4 input input-bordered w-full h-56"/>
        </Modal>
    }
}
