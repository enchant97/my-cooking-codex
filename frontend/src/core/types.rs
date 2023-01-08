use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginToken {
    pub r#type: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoredLogin {
    pub api_url: String,
    pub token: LoginToken,
}