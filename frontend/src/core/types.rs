use serde::{Deserialize, Serialize};

pub mod recipe;
pub mod stats;
pub mod user;

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LoginToken {
    pub r#type: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoredLogin {
    pub api_url: String,
    pub token: LoginToken,
}
