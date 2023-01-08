use super::types::{Login, LoginToken};
use gloo_net::http::Request;

pub struct Api {
    base_url: String,
}

impl Api {
    pub fn new(base: String) -> Self {
        Api { base_url: base }
    }

    pub async fn post_login(&self, login: &Login) -> Option<LoginToken> {
        let req_url = self.base_url.clone() + "api/login/";
        let response = Request::post(&req_url)
            .json(login)
            .unwrap()
            .send()
            .await
            .unwrap();
        Some(response.json::<LoginToken>().await.unwrap())
    }
}
