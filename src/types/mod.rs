use serde::{Deserialize, Serialize};

pub mod auth_body;
pub mod user;
pub use auth_body::*;
pub use user::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}
