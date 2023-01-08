use wasm_bindgen::JsCast;
use web_sys::{console, EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let username_state = use_state(String::default);
    let password_state = use_state(String::default);

    let username = (*username_state).clone();
    let password = (*password_state).clone();

    let on_submit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            console::debug_1(&format!("Login submitted: '{}', '{}'", username, password).into());
        })
    };
    let on_username_change = {
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                username_state.set(input.value());
            }
        })
    };
    let on_password_change = {
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                password_state.set(input.value());
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
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">{ "Username" }</span></label>
                                <input oninput={on_username_change} type="text" placeholder="username" autocomplete="username" class="input input-bordered" />
                            </div>
                            <div class="form-control mb-6">
                                <label class="label"><span class="label-text">{ "Password" }</span></label>
                                <input oninput={on_password_change} type="password" placeholder="password" autocomplete="current-password" class="input input-bordered" />
                            </div>
                            <div class="form-control">
                                <button type="submit" class="btn btn-primary">{"Login"}</button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
