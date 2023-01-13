use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::components::input::ApiUrlSelector;
use crate::contexts::prelude::{create_push_toast_change, use_login, use_toasts, Toast};
use crate::core::api::sanitise_base_url;
use crate::core::effects::{use_login_redirect_effect, LoginState};
use crate::core::{api::Api, types};
use crate::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let login_ctx = use_login().unwrap();
    let toasts_ctx = use_toasts().unwrap();

    let api_url_state = use_state(AttrValue::default);
    let username_state = use_state(AttrValue::default);
    let password_state = use_state(AttrValue::default);

    // redirect if user is logged in
    use_login_redirect_effect(LoginState::NoLogin, Route::Home);
    // try and get a login token, when the form is submitted
    let get_new_token = {
        let api_url = (*api_url_state).clone();
        let username = (*username_state).clone();
        let password = (*password_state).clone();

        use_async(async move {
            let api_url = sanitise_base_url(api_url.to_string());
            let login = types::Login {
                username: username.to_string(),
                password: password.to_string(),
            };
            Api::new(api_url.clone(), None).post_login(&login).await
        })
    };

    // requested login token value has changed
    {
        let get_new_token = get_new_token.clone();
        let api_url = (*api_url_state).clone();
        use_effect_with_deps(
            move |token_response| {
                if token_response.loading {
                    return;
                }
                match &token_response.error {
                    Some(_) => {
                        // TODO handle the actual errors
                        toasts_ctx.dispatch(create_push_toast_change(Toast {
                            message: "failed login!",
                        }));
                    }
                    None => match &token_response.data {
                        Some(token) => {
                            let login_details = types::StoredLogin {
                                api_url: api_url.to_string(),
                                token: token.clone(),
                            };
                            gloo::console::debug!(format!("got details: '{:?}'", login_details));
                            login_ctx.dispatch(Some(login_details.clone()));
                        }
                        None => (),
                    },
                };
            },
            get_new_token,
        );
    }

    let on_submit = {
        let api_url = (*api_url_state).clone();
        let username = (*username_state).clone();
        let get_new_token = get_new_token.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            gloo::console::debug!(format!(
                "Login submitted: '{}', '...', {}",
                username, api_url
            ));
            // get new token in background
            get_new_token.run();
        })
    };
    let on_api_url_change = {
        let api_url_state = api_url_state.clone();
        Callback::from(move |new_value: AttrValue| {
            gloo::console::debug!(format!("api url base set to: '{}'", new_value));
            api_url_state.set(new_value);
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
                            <h1 class="text-5xl font-bold mb-4">{ "Recipes" }</h1>
                            <h2 class="text-4xl font-bold">{ "Please Login" }</h2>
                        </div>
                        <form onsubmit={on_submit}>
                            <ApiUrlSelector onchange={on_api_url_change}/>
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
                                    <button type="submit" disabled=true class="btn loading">{"Loading"}</button>
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
