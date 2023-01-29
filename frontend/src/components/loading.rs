use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    pub r#type: &'static str,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(LoadingButton)]
pub fn button(props: &LoadingProps) -> Html {
    html! {
        <button
            type={props.r#type}
            disabled=true
            class={classes!("btn", "loading", props.classes.clone())}
        >{"Loading"}</button>
    }
}
