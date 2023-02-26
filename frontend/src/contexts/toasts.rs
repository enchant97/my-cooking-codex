use std::collections::VecDeque;
use std::rc::Rc;

use yew::{hook, use_context, Reducible, UseReducerHandle};

#[derive(Debug, PartialEq, Clone)]
pub struct Toast {
    /// Message to display to user
    pub message: String,
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
                if let Some(i) = inner.iter().position(|toast| *toast == v) {
                    inner.remove(i).unwrap();
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

/// Method to push a "push toast" change
pub fn push_toast(ctx: &UseReducerHandle<Toasts>, toast: Toast) {
    ctx.dispatch(ToastChange::Push(toast));
}

/// Method to push a "remove toast" change
pub fn remove_toast(ctx: &UseReducerHandle<Toasts>, toast: Toast) {
    ctx.dispatch(ToastChange::Remove(toast));
}
