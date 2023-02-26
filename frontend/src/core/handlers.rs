use yew::UseReducerHandle;

use crate::{
    contexts::{login::CurrentLogin, prelude::Toast},
    core::api::ApiInternalError,
};

use super::api::ApiError;

/// Convert an API error to a toast message,
/// 'when' is added to the message to describe the action that failed
pub fn api_error_to_toast(error: &ApiError, when: &str) -> Toast {
    match error {
        ApiError::Internal(e) => match e {
            ApiInternalError::Connection => {
                return Toast {
                    message: format!("Action failed as could not connect to server, when {when}"),
                };
            }
            _ => {
                gloo::console::error!(format!("Internal error handled: {:?}, when {when}", e));
                Toast {
                    message: format!("Internal error occurred; action failed, when {when}"),
                }
            }
        },
        ApiError::Response(e) => match e.status_code {
            404 => Toast {
                message: format!("Action failed as resource was not found, when {when}"),
            },
            _ => Toast {
                message: format!(
                    "Action failed received status code '{}', when {when}",
                    e.status_code
                ),
            },
        },
    }
}

/// Logout the user if the API returns a 401
pub fn logout_on_401(error: &ApiError, login_ctx: &UseReducerHandle<CurrentLogin>) {
    if let ApiError::Response(e) = error {
        if e.status_code == 401 {
            login_ctx.dispatch(None);
        }
    }
}
