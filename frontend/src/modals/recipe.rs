use super::Modal;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditTitleProps {
    pub title: AttrValue,
    pub onclose: Callback<Option<String>>,
}

#[function_component(EditTitle)]
pub fn recipe_content(props: &EditTitleProps) -> Html {
    let title_state = use_state(AttrValue::default);

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
        let on_close_callback = props.onclose.clone();
        let title_state = title_state.clone();
        Callback::from(move |_| {
            let title = (*title_state).clone();
            on_close_callback.emit(Some(title.to_string()));
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
        <Modal title={"Edit Title"} oncancel={on_cancel} onsave={on_save}>
            <input oninput={on_title_input} value={(*title_state).clone()} class="my-4 input input-bordered w-full"/>
        </Modal>
    }
}
