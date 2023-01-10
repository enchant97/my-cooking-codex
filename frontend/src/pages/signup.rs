use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
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

    let username = (*username_state).clone();
    let password = (*password_state).clone();
    let password_confirm = (*password_confirm_state).clone();
    let api_url = (*api_url_state).clone();

    // redirect if user is logged in
    use_login_redirect_effect(LoginState::NoLogin, Route::Home);
    // get the default api base url from current window location
    {
        let api_url_state = api_url_state.clone();
        use_effect_with_deps(
            move |_| {
                match gloo_utils::window().location().origin() {
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
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if password != password_confirm {
                toasts_ctx.dispatch(ToastChange::Push(Toast {
                    message: "Passwords do not match",
                }));
                return;
            }

            let api_url = sanitise_base_url(api_url.clone());
            let details = user::CreateUser {
                username: username.clone(),
                password: password.clone(),
            };

            let navigator = navigator.clone();
            let toasts_ctx = toasts_ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Api::new(api_url.clone(), None)
                    .post_create_account(&details)
                    .await
                {
                    Ok(_) => {
                        toasts_ctx.dispatch(ToastChange::Push(Toast {
                            message: "Account Created",
                        }));
                        navigator.push(&Route::Login);
                    }
                    Err(_) => {
                        // TODO handle the actual errors
                        toasts_ctx.dispatch(ToastChange::Push(Toast {
                            message: "failed account creation!",
                        }));
                    }
                };
            });
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
        let password_state = password_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                password_state.set(input.value());
            }
        })
    };
    let on_password_confirm_change = {
        let password_confirm_state = password_confirm_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                password_confirm_state.set(input.value());
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
                                <button type="submit" class="btn btn-primary">{"Signup"}</button>
                                <Link<Route> to={Route::Login} classes={classes!("btn")}>{"Login Instead?"}</Link<Route>>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
