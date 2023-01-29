pub mod recipe;

use yew::prelude::*;

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
    let on_save_click = {
        let save_callback = props.onsave.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(callback) = &save_callback {
                callback.emit(());
            };
        })
    };

    html! {
        <div class="modal modal-open">
            <div class="modal-box">
                <h3 class="font-bold text-lg">{props.title.clone()}</h3>
                { for props.children.iter() }
                <div class="modal-action">
                    <div class="btn-group">
                        if props.onsave.is_some() && !props.loading {
                            <button onclick={on_save_click} class="btn btn-primary">{"Save"}</button>
                        } else if props.onsave.is_some() && props.loading {
                            <button type="submit" disabled=true class="btn loading">{"Loading"}</button>
                        }
                        <button onclick={on_cancel_click} class="btn">{"Cancel"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
