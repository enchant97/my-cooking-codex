pub mod login;
pub mod toasts;

/// Module used to re-export frequently used items, to reduce imports.
pub mod prelude {
    pub use super::login::use_login;
    pub use super::toasts::{push_toast, remove_toast, use_toasts, Toast};
}
