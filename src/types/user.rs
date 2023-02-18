use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub user_id: String,
    pub password: String,
    pub phone_number: String,
    pub user: String,
}
