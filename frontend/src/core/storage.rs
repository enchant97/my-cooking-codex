use gloo_storage::{LocalStorage, Storage};
use super::types::StoredLogin;

const LOGIN_DETAILS_KEY: &str = "login-details";

pub fn read_login_token() -> Option<StoredLogin> {
    LocalStorage::get::<StoredLogin>(LOGIN_DETAILS_KEY).ok()
}

pub fn set_login_token(login: StoredLogin) {
    LocalStorage::set(LOGIN_DETAILS_KEY, login).unwrap()
}
