use std::rc::Rc;

use yew::{Reducible, UseReducerHandle};

use crate::core::{api::Api, storage, types::StoredLogin};

#[derive(Debug, PartialEq, Clone)]
pub struct CurrentLogin {
    pub inner: Option<StoredLogin>,
    pub http_api: Option<Api>,
}

impl CurrentLogin {
    pub fn new() -> Self {
        let login = storage::read_login_token();
        let http_api = match &login {
            Some(v) => Some(Api::from(v.clone())),
            None => None,
        };
        Self {
            inner: login,
            http_api,
        }
    }
}

impl Reducible for CurrentLogin {
    type Action = Option<StoredLogin>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let http_api = match &action {
            Some(v) => {
                storage::set_login_token(v.clone());
                Some(Api::from(v.clone()))
            }
            None => {
                storage::remove_login_token();
                None
            }
        };
        CurrentLogin {
            inner: action,
            http_api,
        }
        .into()
    }
}

pub type CurrentLoginContext = UseReducerHandle<CurrentLogin>;
