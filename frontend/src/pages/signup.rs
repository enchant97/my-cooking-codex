use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_navigator, Link};

use crate::{
    contexts::{
        toasts::{Toast, ToastChange},
        ToastsContext,
    },
    core::{
        api::{sanitise_base_url, Api},
        effects::{use_login_redirect_effect, LoginState},
        types::user,
    },
    Route,
};

#[function_component(Signup)]
pub fn signup() -> Html {
    let toasts_ctx = use_context::<ToastsContext>().unwrap();
    let navigator = use_navigator().unwrap();

    let api_url_state = use_state(String::default);
    let username_state = use_state(String::default);
    let password_state = use_state(String::default);
    let password_confirm_state = use_state(String::default);
    let error_tooltip_state: UseStateHandle<Option<String>> = use_state(Option::default);

    let error_tooltip = (*error_tooltip_state).clone();

    // redirect if user is logged in
    use_login_redirect_effect(LoginState::NoLogin, Route::Home);

    // try and create new account
    let get_new_user = {
        let api_url = (*api_url_state).clone();
        let username = (*username_state).clone();
        let password = (*password_state).clone();

        use_async(async move {
            let api_url = sanitise_base_url(api_url.clone());
            let details = user::CreateUser {
                username: username.clone(),
                password: password.clone(),
            };
            Api::new(api_url.clone(), None)
                .post_create_account(&details)
                .await
        })
    };

    // new user value has changed
    {
        let get_new_user = get_new_user.clone();
        let toasts_ctx = toasts_ctx.clone();
        use_effect_with_deps(
            move |response| {
                if response.loading {
                    return;
                }
                match &response.error {
                    Some(_) => {
                        // TODO handle the actual errors
                        toasts_ctx.dispatch(ToastChange::Push(Toast {
                            message: "failed account creation!",
                        }));
                    }
                    None => match &response.data {
                        Some(_) => {
                            toasts_ctx.dispatch(ToastChange::Push(Toast {
                                message: "Account Created",
                            }));
                            navigator.push(&Route::Login);
                        }
                        None => (),
                    },
                };
            },
            get_new_user,
        );
    }

    // get the default api base url from current window location
    {
        let api_url_state = api_url_state.clone();
        use_effect_with_deps(
            move |_| {
                match gloo::utils::window().location().origin() {
                    Ok(href) => {
                        let href = sanitise_base_url(href.to_owned());
                        let href = href + "/api";
                        api_url_state.set(href);
                    }
                    Err(_) => (),
                };
            },
            (),
        );
    }

    let on_submit = {
        let password = (*password_state).clone();
        let password_confirm = (*password_confirm_state).clone();
        let get_new_user = get_new_user.clone();
        let toasts_ctx = toasts_ctx.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if password != password_confirm {
                toasts_ctx.dispatch(ToastChange::Push(Toast {
                    message: "Passwords do not match",
                }));
                return;
            }
            // create new user in background
            get_new_user.run();
        })
    };
    let on_api_url_change = {
        let api_url_state = api_url_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                api_url_state.set(input.value());
            }
        })
    };
    let on_username_change = {
        let username_state = username_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                username_state.set(input.value());
            }
        })
    };
    let on_password_change = {
        let error_tooltip_state = error_tooltip_state.clone();
        let password_confirm = (*password_confirm_state).clone();
        let password_state = password_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let value = input.value();
                password_state.set(value.clone());
                if value != password_confirm {
                    error_tooltip_state.set(Some("passwords do not match!".to_owned()));
                } else {
                    error_tooltip_state.set(None);
                }
            }
        })
    };
    let on_password_confirm_change = {
        let error_tooltip_state = error_tooltip_state.clone();
        let password = (*password_state).clone();
        let password_confirm_state = password_confirm_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let value = input.value();
                password_confirm_state.set(value.clone());
                if value != password {
                    error_tooltip_state.set(Some("passwords do not match!".to_owned()));
                } else {
                    error_tooltip_state.set(None);
                }
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
                            <h2 class="text-4xl font-bold">{ "Create Account" }</h2>
                        </div>
                        <form onsubmit={on_submit}>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">{ "Api Url" }</span></label>
                                <input
                                    value={ (*api_url_state).clone() }
                                    oninput={on_api_url_change}
                                    type="url"
                                    autocomplete="https://"
                                    class="input input-bordered"
                                    required=true
                                />
                            </div>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">{ "Username" }</span></label>
                                <input
                                    value={ (*username_state).clone() }
                                    oninput={on_username_change}
                                    type="text"
                                    placeholder="username"
                                    autocomplete="username"
                                    class="input input-bordered"
                                    required=true
                                />
                            </div>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">{ "Password" }</span></label>
                                <input
                                    value={ (*password_state).clone() }
                                    oninput={on_password_change}
                                    type="password"
                                    placeholder="password"
                                    autocomplete="new-password"
                                    class="input input-bordered"
                                    required=true
                                />
                            </div>
                            <div class="form-control mb-6">
                                <label class="label"><span class="label-text">{ "Password Confirm" }</span></label>
                                <input
                                    value={ (*password_confirm_state).clone() }
                                    oninput={on_password_confirm_change}
                                    type="password"
                                    placeholder="password confirm"
                                    autocomplete="new-password"
                                    class="input input-bordered"
                                    required=true
                                />
                            </div>
                            <div class="form-control btn-group btn-group-vertical">
                                if get_new_user.loading {
                                    <button type="submit" disabled=true class="btn loading">{"Loading"}</button>
                                } else {
                                    if error_tooltip.is_none() {
                                        <button type="submit" class="btn btn-primary">{"Signup"}</button>
                                    } else {
                                        <div class="tooltip tooltip-open tooltip-error" data-tip={error_tooltip.unwrap()}>
                                            <button type="submit" disabled=true class="btn btn-disabled btn-block">{"Signup"}</button>
                                        </div>
                                    }
                                }
                                <Link<Route> to={Route::Login} classes={classes!("btn")}>{"Login Instead?"}</Link<Route>>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
