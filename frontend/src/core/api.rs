use super::types::{Login, LoginToken};
use gloo_net::http::Request;

pub fn sanitise_base_url(base: String) -> String {
    let base = match base.strip_suffix('/') {
        Some(v) => v.to_owned(),
        None => base,
    };
    base
}

pub struct Api {
    base_url: String,
}

impl Api {
    pub fn new(base: String) -> Self {
        Api {
            base_url: sanitise_base_url(base),
        }
    }

    pub async fn post_login(&self, login: &Login) -> Option<LoginToken> {
        let req_url = self.base_url.clone() + "/login/";
        let response = Request::post(&req_url)
            .json(login)
            .unwrap()
            .send()
            .await
            .unwrap();
        Some(response.json::<LoginToken>().await.unwrap())
    }
}
