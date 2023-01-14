use std::rc::Rc;

use yew::{Reducible, UseReducerHandle, use_context, hook};

use crate::core::{api::Api, storage, types::StoredLogin};

#[derive(Debug, PartialEq, Clone)]
pub struct CurrentLogin {
    pub login: Option<StoredLogin>,
    pub http_api: Option<Api>,
}

impl CurrentLogin {
    pub fn new() -> Self {
        let login = storage::read_login_token();
        let http_api = login.as_ref().map(|v| Api::from(v.clone()));
        Self {
            login,
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
            login: action,
            http_api,
        }
        .into()
    }
}

pub type CurrentLoginContext = UseReducerHandle<CurrentLogin>;

#[hook]
pub fn use_login() -> Option<UseReducerHandle<CurrentLogin>> {
    use_context::<CurrentLoginContext>()
}
