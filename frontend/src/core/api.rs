use super::types::{Login, LoginToken, StoredLogin};
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use std::convert::From;

pub fn sanitise_base_url(base: String) -> String {
    let base = match base.strip_suffix('/') {
        Some(v) => v.to_owned(),
        None => base,
    };
    base
}

#[derive(Debug)]
pub enum ApiInternalError {
    Connection,
    Deserialization,
    Generic(gloo_net::Error),
}

#[derive(Debug)]
pub struct ApiResponseError {
    pub status_code: u16,
}

#[derive(Debug)]
pub enum ApiError {
    Internal(ApiInternalError),
    Response(ApiResponseError),
}

impl ApiError {
    pub fn from_response_result(
        response: Result<gloo_net::http::Response, gloo_net::Error>,
    ) -> Result<gloo_net::http::Response, Self> {
        match response {
            Ok(v) => Ok(v),
            Err(err) => match err {
                gloo_net::Error::JsError(_) => {
                    Err(ApiError::Internal(ApiInternalError::Connection))
                }
                err => Err(ApiError::Internal(ApiInternalError::Generic(err))),
            },
        }
    }

    pub async fn check_json_response_ok<T>(response: gloo_net::http::Response) -> Result<T, Self>
    where
        T: DeserializeOwned,
    {
        match response.ok() {
            false => Err(ApiError::Response(ApiResponseError {
                status_code: response.status(),
            })),
            true => match response.json::<T>().await {
                Err(err) => match err {
                    gloo_net::Error::SerdeError(_) => {
                        Err(ApiError::Internal(ApiInternalError::Deserialization))
                    }
                    err => Err(ApiError::Internal(ApiInternalError::Generic(err))),
                },
                Ok(v) => Ok(v),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Api {
    base_url: String,
    login_token: Option<LoginToken>,
}

impl Api {
    pub fn new(base: String, token: Option<LoginToken>) -> Self {
        Api {
            base_url: sanitise_base_url(base),
            login_token: token,
        }
    }

    fn get_authorization_value(&self) -> Option<String> {
        match &self.login_token {
            Some(token) => Some(format!("{} {}", token.r#type, token.token)),
            None => None,
        }
    }

    pub async fn post_login(&self, login: &Login) -> Result<LoginToken, ApiError> {
        let req_url = self.base_url.clone() + "/login/";
        let response = ApiError::from_response_result(
            Request::post(&req_url).json(login).unwrap().send().await,
        )?;
        ApiError::check_json_response_ok::<LoginToken>(response).await
    }
}

impl From<StoredLogin> for Api {
    fn from(login: StoredLogin) -> Self {
        Api {
            base_url: login.api_url,
            login_token: Some(login.token),
        }
    }
}
