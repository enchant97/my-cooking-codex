use yew::{
    prelude::{function_component, html, Html},
    use_context, use_effect_with_deps,
};

use crate::{
    contexts::{CurrentLoginContext, HttpApiContext},
    core::effects::{use_login_redirect_effect, LoginState},
    Route,
};

#[function_component(Logout)]
pub fn logout() -> Html {
    let login_ctx = use_context::<CurrentLoginContext>().unwrap();
    let api_ctx = use_context::<HttpApiContext>().unwrap();
    use_login_redirect_effect(LoginState::HasLogin, Route::Login);
    {
        let login_ctx = login_ctx.clone();
        let api_ctx = api_ctx.clone();
        use_effect_with_deps(
            move |_| {
                login_ctx.dispatch(None);
                api_ctx.dispatch(None);
            },
            (),
        );
    }
    html! {}
}
