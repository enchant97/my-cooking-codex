use crate::contexts::login::use_login;

use crate::modals::Modal;
use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SetImageProps {
    pub id: String,
    pub onclose: Callback<Option<bool>>,
}

#[function_component(SetImage)]
pub fn recipe_image(props: &SetImageProps) -> Html {
    // TODO allow image removal
    let login_ctx = use_login().unwrap();
    let is_loading_state = use_state(bool::default);
    let image_file: UseStateHandle<Option<File>> = use_state(Option::default);

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
        let api = login_ctx.http_api.clone();
        let is_loading_state = is_loading_state.clone();
        let image_file = image_file.clone();
        Callback::from(move |_| {
            let on_close_callback = on_close_callback.clone();
            let id = id.clone();
            let api = api.clone().unwrap();
            let is_loading_state = is_loading_state.clone();
            if let Some(file) = (*image_file).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    is_loading_state.set(true);
                    api.post_recipe_image(id, file).await.unwrap();
                    is_loading_state.set(false);
                    on_close_callback.emit(Some(true));
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
        <Modal title={"Upload Image"} oncancel={on_cancel} onsave={on_save} loading={*is_loading_state}>
            <label for="recipe-set-image-upload-input">{"Upload new image"}</label>
            <input onchange={on_image_change} id="recipe-set-image-upload-input" type="file" accept="image/*" multiple=false />
        </Modal>
    }
}
