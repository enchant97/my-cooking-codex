use super::Modal;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditTitleProps {
    pub title: AttrValue,
    pub onclose: Callback<()>,
}

#[function_component(EditTitle)]
pub fn recipe_content(props: &EditTitleProps) -> Html {
    html! {
        <Modal title={"Edit Title"} oncancel={props.onclose.clone()}>
            <input value={props.title.clone()} class="my-4 input input-bordered w-full"/>
        </Modal>
    }
}
