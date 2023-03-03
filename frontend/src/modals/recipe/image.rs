use crate::contexts::prelude::push_toast;
use crate::contexts::{login::use_login, prelude::use_toasts};

use crate::core::handlers::{api_error_to_toast, logout_on_401};
use crate::modals::Modal;
use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SetImageProps {
    pub id: String,
    pub image_id: Option<String>,
    pub onclose: Callback<Option<Option<String>>>,
}

#[function_component(SetImage)]
pub fn recipe_image(props: &SetImageProps) -> Html {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();
    let is_loading_state = use_state(bool::default);
    let image_file: UseStateHandle<Option<File>> = use_state(Option::default);

    let on_image_delete_click = {
        let on_close_callback = props.onclose.clone();
        let id = props.id.to_string();
        let is_loading_state = is_loading_state.clone();
        let login_ctx = login_ctx.clone();
        let toasts_ctx = toasts_ctx.clone();
        Callback::from(move |_: MouseEvent| {
            let login_ctx = login_ctx.clone();
            let toasts_ctx = toasts_ctx.clone();
            let api = login_ctx.http_api.clone().unwrap();
            let on_close_callback = on_close_callback.clone();
            let id = id.clone();
            let is_loading_state = is_loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                is_loading_state.set(true);
                let result = api.delete_recipe_image(id.clone()).await;
                is_loading_state.set(false);
                match result {
                    Ok(_) => {
                        on_close_callback.emit(Some(None));
                    }
                    Err(e) => {
                        push_toast(&toasts_ctx, api_error_to_toast(&e, "deleting recipe image"));
                        logout_on_401(&e, &login_ctx);
                    }
                };
            });
        })
    };

    let on_image_change = {
        let image_file = image_file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files().unwrap().get(0).unwrap();
            image_file.set(Some(file));
        })
    };

    let on_save = {
        let on_close_callback = props.onclose.clone();
        let id = props.id.to_string();
        let is_loading_state = is_loading_state.clone();
        Callback::from(move |_| {
            let login_ctx = login_ctx.clone();
            let toasts_ctx = toasts_ctx.clone();
            let api = login_ctx.http_api.clone().unwrap();
            let on_close_callback = on_close_callback.clone();
            let id = id.clone();
            let is_loading_state = is_loading_state.clone();
            if let Some(file) = (*image_file).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    is_loading_state.set(true);
                    let result = api.post_recipe_image(id, file).await;
                    is_loading_state.set(false);
                    match result {
                        Ok(image_id) => {
                            on_close_callback.emit(Some(Some(image_id)));
                        }
                        Err(e) => {
                            push_toast(
                                &toasts_ctx,
                                api_error_to_toast(&e, "uploading recipe image"),
                            );
                            logout_on_401(&e, &login_ctx);
                        }
                    };
                });
            }
        })
    };

    let on_cancel = {
        let on_close_callback = props.onclose.clone();
        Callback::from(move |_| {
            on_close_callback.emit(None);
        })
    };

    html! {
        <Modal title={"Edit Image"} oncancel={on_cancel} onsave={on_save} loading={*is_loading_state}>
            if props.image_id.is_some() {
                <button class="btn btn-outline btn-error mb-2" onclick={on_image_delete_click} type="button">{"Delete Existing"}</button>
            }
            <div>
                <label for="recipe-set-image-upload-input">{"Upload new image"}</label>
                <input onchange={on_image_change} id="recipe-set-image-upload-input" type="file" accept="image/*" multiple=false />
            </div>
        </Modal>
    }
}
