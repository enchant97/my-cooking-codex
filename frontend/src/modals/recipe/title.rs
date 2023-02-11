use crate::{contexts::login::use_login, core::types::recipe::UpdateRecipe};

use crate::modals::Modal;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditTitleProps {
    pub id: String,
    pub title: AttrValue,
    pub onclose: Callback<Option<String>>,
}

#[function_component(EditTitle)]
pub fn recipe_title(props: &EditTitleProps) -> Html {
    let login_ctx = use_login().unwrap();
    let title_state = use_state(AttrValue::default);
    let is_loading_state = use_state(bool::default);

    {
        let initial_title = props.title.clone();
        let title_state = title_state.clone();
        use_effect_with_deps(
            move |_| {
                title_state.set(initial_title.clone());
            },
            (),
        );
    }

    let on_save = {
        let api = login_ctx.http_api.clone();
        let id = props.id.to_string();
        let on_close_callback = props.onclose.clone();
        let title_state = title_state.clone();
        let is_loading_state = is_loading_state.clone();
        Callback::from(move |_| {
            let api = api.clone().unwrap();
            let id = id.clone();
            let on_close_callback = on_close_callback.clone();
            let title = (*title_state).clone();
            let is_loading_state = is_loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                is_loading_state.set(true);
                api.patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        title: Some(title.to_string()),
                        short_description: None,
                        long_description: None,
                        tags: None,
                        ingredients: None,
                        steps: None,
                    },
                )
                .await
                .unwrap();
                is_loading_state.set(false);
                on_close_callback.emit(Some(title.to_string()));
            });
        })
    };

    let on_cancel = {
        let on_close_callback = props.onclose.clone();
        Callback::from(move |_| {
            on_close_callback.emit(None);
        })
    };

    let on_title_input = {
        let title_state = title_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                title_state.set(input.value().into());
            }
        })
    };

    html! {
        <Modal title={"Edit Title"} oncancel={on_cancel} onsave={on_save} loading={(*is_loading_state).clone()}>
            <input oninput={on_title_input} value={(*title_state).clone()} class="my-4 input input-bordered w-full" required=true/>
        </Modal>
    }
}