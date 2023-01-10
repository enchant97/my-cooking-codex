use crate::core::api::Api;
use std::rc::Rc;

use yew::{Reducible, UseReducerHandle};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpApi {
    pub inner: Option<Api>,
}

impl HttpApi {
    pub fn new(api: Option<Api>) -> Self {
        Self { inner: api }
    }
}

impl Reducible for HttpApi {
    type Action = Option<Api>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        HttpApi { inner: action }.into()
    }
}

pub type HttpApiContext = UseReducerHandle<HttpApi>;
