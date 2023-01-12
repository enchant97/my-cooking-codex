use std::collections::VecDeque;
use std::rc::Rc;

use yew::{hook, use_context, Reducible, UseReducerHandle};

#[derive(Debug, PartialEq, Clone)]
pub struct Toast {
    /// Message to display to user
    pub message: &'static str,
}

pub enum ToastChange {
    Push(Toast),
    Remove(Toast),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Toasts {
    pub inner: VecDeque<Toast>,
}

impl Toasts {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }
}

impl Reducible for Toasts {
    type Action = ToastChange;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut inner = self.inner.clone();
        match action {
            ToastChange::Push(v) => inner.push_back(v),
            ToastChange::Remove(v) => {
                match inner.iter().position(|toast| *toast == v) {
                    Some(i) => {
                        inner.remove(i).unwrap();
                    }
                    None => (),
                };
            }
        }

        Toasts { inner }.into()
    }
}

pub type ToastsContext = UseReducerHandle<Toasts>;

#[hook]
pub fn use_toasts() -> Option<UseReducerHandle<Toasts>> {
    use_context::<ToastsContext>()
}

/// Method to create a "push toast" change
pub fn create_push_toast_change(toast: Toast) -> ToastChange {
    ToastChange::Push(toast)
}

/// Method to create a "remove toast" change
pub fn create_remove_toast_change(toast: Toast) -> ToastChange {
    ToastChange::Remove(toast)
}
