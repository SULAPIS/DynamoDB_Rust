use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{extract::State, Json};
use jsonwebtoken::{encode, Header};

use crate::{
    error::AuthError,
    keys,
    types::{AuthBody, Claims, User},
};

pub async fn login(
    State(table): State<Arc<String>>,
    Json(user): Json<User>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if user.phone_number.is_empty() || user.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &keys::encoding())
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
