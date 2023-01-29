use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_navigator, Link};

use crate::{
    components::{input::BaseUrlSelector, loading::LoadingButton},
    contexts::prelude::{create_push_toast_change, use_toasts, Toast},
    core::{
        api::{sanitise_base_url, Api},
        effects::{use_login_redirect_effect, LoginState},
        types::user,
    },
    Route,
};

#[function_component(Signup)]
pub fn signup() -> Html {
    let toasts_ctx = use_toasts().unwrap();
    let navigator = use_navigator().unwrap();

    let base_url_state = use_state(AttrValue::default);
    let username_state = use_state(AttrValue::default);
    let password_state = use_state(AttrValue::default);
    let password_confirm_state = use_state(AttrValue::default);
    let error_tooltip_state: UseStateHandle<Option<AttrValue>> = use_state(Option::default);

    let error_tooltip = (*error_tooltip_state).clone();

    // redirect if user is logged in
    use_login_redirect_effect(LoginState::NoLogin, Route::Home);

    // try and create new account
    let get_new_user = {
        let base_url = (*base_url_state).clone();
        let username = (*username_state).clone();
        let password = (*password_state).clone();

        use_async(async move {
            let api_url = sanitise_base_url(base_url.to_string()) + "/api";
            let details = user::CreateUser {
                username: username.to_string(),
                password: password.to_string(),
            };
            Api::new(api_url, None)
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
                        toasts_ctx.dispatch(create_push_toast_change(Toast {
                            message: "failed account creation!",
                        }));
                    }
                    None => match &response.data {
                        Some(_) => {
                            toasts_ctx.dispatch(create_push_toast_change(Toast {
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

    let on_submit = {
        let password = (*password_state).clone();
        let password_confirm = (*password_confirm_state).clone();
        let get_new_user = get_new_user.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if password != password_confirm {
                toasts_ctx.dispatch(create_push_toast_change(Toast {
                    message: "Passwords do not match",
                }));
                return;
            }
            // create new user in background
            get_new_user.run();
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
        let error_tooltip_state = error_tooltip_state.clone();
        let password_confirm = (*password_confirm_state).clone();
        let password_state = password_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let value: AttrValue = input.value().into();
                password_state.set(value.clone());
                if value != password_confirm {
                    error_tooltip_state.set(Some("passwords do not match!".into()));
                } else {
                    error_tooltip_state.set(None);
                }
            }
        })
    };
    let on_password_confirm_change = {
        let error_tooltip_state = error_tooltip_state;
        let password = (*password_state).clone();
        let password_confirm_state = password_confirm_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let value: AttrValue = input.value().into();
                password_confirm_state.set(value.clone());
                if value != password {
                    error_tooltip_state.set(Some("passwords do not match!".into()));
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
                            <BaseUrlSelector onchange={on_base_url_change}/>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">{ "Username" }</span></label>
                                <input
                                    value={ (*username_state).clone() }
                                    oninput={on_username_change}
                                    type="text"
                                    placeholder="e.g. leo"
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
                                    placeholder="e.g. ••••••••"
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
                                    placeholder="e.g. ••••••••"
                                    autocomplete="new-password"
                                    class="input input-bordered"
                                    required=true
                                />
                            </div>
                            <div class="form-control btn-group btn-group-vertical">
                                if get_new_user.loading {
                                    <LoadingButton r#type="submit"/>
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
