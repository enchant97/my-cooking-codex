use crate::core::api::sanitise_base_url;
use url::Url;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BaseUrlSelectorProps {
    pub onchange: Callback<AttrValue>,
}

/// Component to provide a user adjustable api url base input (for use in login/signup)
#[function_component(BaseUrlSelector)]
pub fn base_url_selector(props: &BaseUrlSelectorProps) -> Html {
    let base_url_state = use_state(AttrValue::default);
    let api_host_state: UseStateHandle<Option<AttrValue>> = use_state(Option::default);
    let change_state = use_state(bool::default);

    // get the default api base url from current window location
    {
        let base_url_state = base_url_state.clone();
        use_effect_with_deps(
            move |_| {
                if let Ok(href) = gloo::utils::window().location().origin() {
                    let href = sanitise_base_url(href);
                    base_url_state.set(href.into());
                };
            },
            (),
        );
    }

    // change the host preview display on api url change
    {
        let api_host_state = api_host_state.clone();
        let base_url = (*base_url_state).clone();
        use_effect_with_deps(
            move |_| {
                if &base_url == "" {
                    api_host_state.set(None);
                    return;
                }
                let url = match Url::parse(&base_url) {
                    Ok(v) => v,
                    Err(_) => {
                        api_host_state.set(None);
                        return;
                    }
                };
                let new_host = url.host_str().unwrap().to_owned();
                api_host_state.set(Some(new_host.into()));
            },
            base_url_state.clone(),
        );
    }

    {
        let base_url = (*base_url_state).clone();
        let change_mode = *change_state;
        let onchange_callback = props.onchange.clone();
        use_effect_with_deps(
            move |_| {
                if !change_mode && base_url != "" {
                    onchange_callback.emit(base_url);
                }
            },
            base_url_state.clone(),
        );
    }

    let on_change_click = {
        let base_url = (*base_url_state).clone();
        let api_host = (*api_host_state).clone();
        let change_state = change_state.clone();
        let change_mode = *change_state;
        let onchange_callback = props.onchange.clone();
        Callback::from(move |_: MouseEvent| {
            if change_mode && api_host.is_some() {
                onchange_callback.emit(base_url.clone());
            }
            change_state.set(!change_mode);
        })
    };

    let on_base_url_change = {
        let base_url_state = base_url_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                base_url_state.set(sanitise_base_url(input.value()).into());
            }
        })
    };

    html! {
        <div class="form-control">
            <div class="input-group">
                if *change_state {
                    <input value={ (*base_url_state).clone() } oninput={on_base_url_change} type="url" placeholder="https://" class="input input-bordered" required=true />
                    <button onclick={on_change_click} type="button" class="btn">{"Save"}</button>
                } else {
                    <span>{"Using Server At: "}</span>
                    <span>{(*api_host_state).clone().unwrap_or_else(||"(unset)".into())}</span>
                    <button onclick={on_change_click} type="button" class="btn">{"Change"}</button>
                }
            </div>
        </div>
    }
}
