use std::rc::Rc;

use yew::{Reducible, UseReducerHandle};

use crate::core::{storage, types::StoredLogin};

#[derive(Debug, PartialEq, Clone)]
pub struct CurrentLogin {
    pub inner: Option<StoredLogin>,
}

impl CurrentLogin {
    pub fn new() -> Self {
        Self {
            inner: storage::read_login_token(),
        }
    }
}

impl Reducible for CurrentLogin {
    type Action = Option<StoredLogin>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match &action {
            Some(v) => storage::set_login_token(v.clone()),
            None => storage::remove_login_token(),
        };
        CurrentLogin { inner: action }.into()
    }
}

pub type CurrentLoginContext = UseReducerHandle<CurrentLogin>;
