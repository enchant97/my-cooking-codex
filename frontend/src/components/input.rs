use crate::core::{api::sanitise_base_url, Fraction};
use regex::Regex;
use url::Url;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

const VALID_FRACTIONAL_INPUT_REGEX: &str = r#"^(?:(?:\d+)|(?:\d+/\d+)|(?:\d+\.\d+))$"#;
const VALID_FRACTION_REGEX: &str = r#"^(?:\d+/\d+)$"#;

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

#[derive(Properties, PartialEq)]
pub struct FractionalNumberInputProps {
    #[prop_or_default]
    pub classes: Classes,
    pub value: f32,
    pub oninput: Callback<f32>,
    #[prop_or_default]
    pub placeholder: &'static str,
    #[prop_or_default]
    pub required: bool,
}

#[function_component(FractionalNumberInput)]
pub fn fractional_input(props: &FractionalNumberInputProps) -> Html {
    let actual_state = use_state(f32::default);
    let input_state = use_state(AttrValue::default);
    let is_valid_state = use_state(bool::default);

    {
        let initial_value = props.value;
        let actual_state = actual_state.clone();
        let input_state = input_state.clone();
        let is_valid_state = is_valid_state.clone();
        use_effect_with_deps(
            move |_| {
                actual_state.set(initial_value);
                if initial_value == 0.0 {
                    input_state.set("".into());
                    is_valid_state.set(false);
                    return;
                }
                is_valid_state.set(true);
                input_state.set(initial_value.to_string().into());
            },
            props.value,
        );
    }

    let on_input = {
        let on_input_callback = props.oninput.clone();
        let actual_state = actual_state.clone();
        let input_state = input_state.clone();
        let is_valid_state = is_valid_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if Regex::new(VALID_FRACTIONAL_INPUT_REGEX)
                    .unwrap()
                    .is_match(&input.value())
                {
                    // if the input is a valid fractional number, update the state
                    let parsed: f32;
                    input_state.set(input.value().into());
                    if Regex::new(VALID_FRACTION_REGEX)
                        .unwrap()
                        .is_match(&input.value())
                    {
                        // if the input is a valid fraction, convert it to a float
                        parsed = input
                            .value()
                            .parse::<Fraction>()
                            .expect("Failed to parse fraction")
                            .to_f32();
                        actual_state.set(parsed);
                    } else {
                        // otherwise, just parse it as a float
                        parsed = input.value().parse::<f32>().expect("Failed to parse float");
                        actual_state.set(parsed);
                    }
                    is_valid_state.set(true);
                    on_input_callback.emit(parsed);
                } else {
                    is_valid_state.set(false);
                    input_state.set(input.value().into());
                }
            }
        })
    };

    html! {
        <input
            class={classes!("input", props.classes.clone(), if !*is_valid_state {"input-error"} else {""})}
            oninput={on_input}
            value={(*input_state).clone()}
            type="text"
            pattern={VALID_FRACTIONAL_INPUT_REGEX}
            placeholder={props.placeholder}
            required={props.required}
        />
    }
}
