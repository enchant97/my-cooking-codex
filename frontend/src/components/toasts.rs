use gloo::timers::callback::Timeout;
use yew::{
    prelude::{function_component, html, Html},
    use_effect_with_deps, Properties,
};

use crate::contexts::prelude::{create_remove_toast_change, use_toasts, Toast};

#[derive(Properties, PartialEq)]
struct ToastProps {
    pub toast: Toast,
}

#[function_component(ToastRow)]
fn toast_row(props: &ToastProps) -> Html {
    let toasts_ctx = use_toasts().unwrap();
    {
        let toast = props.toast.to_owned();
        use_effect_with_deps(
            move |_| {
                let timeout = Timeout::new(6_000, move || {
                    toasts_ctx.dispatch(create_remove_toast_change(toast));
                });
                timeout.forget();
            },
            (),
        );
    }
    html! {<div class="alert alert-info"><span>{props.toast.message}</span></div>}
}

#[function_component(Toasts)]
pub fn toasts() -> Html {
    let toasts_ctx = use_toasts().unwrap();
    let toasts = toasts_ctx.inner.to_owned();
    html! {
        <div class="toast toast-top toast-start">
            {
                for toasts.iter().map(|toast| {
                    html!{<ToastRow toast={toast.clone()}/>}
                })
            }
        </div>
    }
}
