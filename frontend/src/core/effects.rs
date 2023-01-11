use yew::{hook, use_context, use_effect_with_deps};
use yew_router::prelude::use_navigator;

use crate::{contexts::CurrentLoginContext, Route};

pub enum LoginState {
    NoLogin,
    HasLogin,
}

/// redirect when login state is not the required state
#[hook]
pub fn use_login_redirect_effect(require_state: LoginState, redirect_to: Route) {
    let login_ctx = use_context::<CurrentLoginContext>().unwrap();
    let current_login = login_ctx.login.to_owned();
    let navigator = use_navigator().unwrap();
    use_effect_with_deps(
        move |_| match (current_login, require_state) {
            (Some(_), LoginState::NoLogin) | (None, LoginState::HasLogin) => {
                navigator.push(&redirect_to);
            }
            (Some(_), LoginState::HasLogin) | (None, LoginState::NoLogin) => (),
        },
        login_ctx,
    );
}
