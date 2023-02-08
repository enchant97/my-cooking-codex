pub mod recipe;

use yew::prelude::*;

use crate::components::loading::LoadingButton;

#[derive(Properties, PartialEq)]
pub struct ModalControllerProps {
    pub modal: Option<Html>,
}

#[function_component(ModalController)]
pub fn modal_controller(props: &ModalControllerProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal_host")
        .expect("expected to find a #modal_host element");

    if let Some(modal) = props.modal.clone() {
        create_portal(modal, modal_host)
    } else {
        html! {<></>}
    }
}

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: AttrValue,
    pub oncancel: Callback<()>,
    #[prop_or_default]
    pub onsave: Option<Callback<()>>,
    #[prop_or_default]
    pub loading: bool,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal_controller(props: &ModalProps) -> Html {
    let on_cancel_click = {
        let cancel_callback = props.oncancel.clone();
        Callback::from(move |_: MouseEvent| {
            cancel_callback.emit(());
        })
    };
    let on_submit = {
        let save_callback = props.onsave.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(callback) = &save_callback {
                callback.emit(());
            };
        })
    };

    html! {
        <div class="modal modal-open">
            <form class="modal-box" onsubmit={on_submit}>
                <h3 class="font-bold text-lg mb-3">{props.title.clone()}</h3>
                { for props.children.iter() }
                <div class="modal-action">
                    <div class="btn-group">
                        if props.onsave.is_some() && !props.loading {
                            <button type="submit" class="btn btn-primary">{"Save"}</button>
                        } else if props.onsave.is_some() && props.loading {
                            <LoadingButton r#type="submit"/>
                        }
                        <button type="button" onclick={on_cancel_click} class="btn">{"Cancel"}</button>
                    </div>
                </div>
            </form>
        </div>
    }
}
