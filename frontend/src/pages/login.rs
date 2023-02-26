use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::components::input::BaseUrlSelector;
use crate::components::loading::LoadingButton;
use crate::contexts::prelude::{push_toast, use_login, use_toasts};
use crate::core::api::sanitise_base_url;
use crate::core::effects::{use_login_redirect_effect, LoginState};
use crate::core::handlers::api_error_to_toast;
use crate::core::APP_TITLE;
use crate::core::{api::Api, types};
use crate::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();

    let base_url_state = use_state(AttrValue::default);
    let username_state = use_state(AttrValue::default);
    let password_state = use_state(AttrValue::default);

    // redirect if user is logged in
    use_login_redirect_effect(LoginState::NoLogin, Route::Home);
    // try and get a login token, when the form is submitted
    let get_new_token = {
        let base_url = (*base_url_state).clone();
        let username = (*username_state).clone();
        let password = (*password_state).clone();

        use_async(async move {
            let api_url = sanitise_base_url(base_url.to_string()) + "/api";
            let login = types::Login {
                username: username.to_string(),
                password: password.to_string(),
            };
            Api::new(api_url, None).post_login(&login).await
        })
    };

    // requested login token value has changed
    {
        let get_new_token = get_new_token.clone();
        let base_url = (*base_url_state).clone();
        use_effect_with_deps(
            move |token_response| {
                if token_response.loading {
                    return;
                }
                match &token_response.error {
                    Some(error) => {
                        push_toast(&toasts_ctx, api_error_to_toast(error, "requesting login"));
                    }
                    None => match &token_response.data {
                        Some(token) => {
                            let base_url = base_url.to_string();
                            let login_details = types::StoredLogin {
                                api_url: base_url.clone() + "/api",
                                media_url: base_url + "/media",
                                token: token.clone(),
                            };
                            gloo::console::debug!(format!(
                                "got authentication token, will expire at: '{:?}'",
                                token.expiry,
                            ));
                            login_ctx.dispatch(Some(login_details));
                        }
                        None => (),
                    },
                };
            },
            get_new_token,
        );
    }

    let on_submit = {
        let base_url = (*base_url_state).clone();
        let username = (*username_state).clone();
        let get_new_token = get_new_token.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            gloo::console::debug!(format!(
                "Login submitted: '{}', '...', {}",
                username, base_url
            ));
            // get new token in background
            get_new_token.run();
        })
    };
    let on_base_url_change = {
        Callback::from(move |new_value: AttrValue| {
            gloo::console::debug!(format!("base url base set to: '{}'", new_value));
            base_url_state.set(new_value);
        })
    };
    let on_username_change = {
        let username_state = username_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                username_state.set(input.value().into());
            }
        })
    };
    let on_password_change = {
        let password_state = password_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                password_state.set(input.value().into());
            }
        })
    };

    html! {
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                    <div class="card-body">
                        <div class="mb-4">
                            <h1 class="text-5xl font-bold mb-4">{APP_TITLE}</h1>
                            <h2 class="text-4xl font-bold">{ "Please Login" }</h2>
                        </div>
                        <form onsubmit={on_submit}>
                            <BaseUrlSelector onchange={on_base_url_change}/>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">{ "Username" }</span></label>
                                <input value={ (*username_state).clone() } oninput={on_username_change} type="text" placeholder="e.g. leo" autocomplete="username" class="input input-bordered" required=true />
                            </div>
                            <div class="form-control mb-6">
                                <label class="label"><span class="label-text">{ "Password" }</span></label>
                                <input value={ (*password_state).clone() } oninput={on_password_change} type="password" placeholder="e.g. ••••••••" autocomplete="current-password" class="input input-bordered" required=true />
                            </div>
                            <div class="form-control btn-group btn-group-vertical">
                                if get_new_token.loading {
                                    <LoadingButton r#type="submit"/>
                                } else {
                                    <button type="submit" class="btn btn-primary">{"Login"}</button>
                                }
                                <Link<Route> to={Route::Signup} classes={classes!("btn")}>{"Signup Instead?"}</Link<Route>>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
