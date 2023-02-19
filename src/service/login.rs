use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{extract::State, Json};
use jsonwebtoken::{encode, Header};
use tokio_stream::StreamExt;

use crate::{
    error::AuthError,
    keys,
    types::{AuthBody, Claims, DbState, User},
};

pub async fn login(
    State(db_state): State<Arc<DbState>>,
    Json(user): Json<User>,
) -> Result<Json<String>, AuthError> {
    // Check if the user sent the credentials
    if user.phone_number.is_empty() || user.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // Check if the user exists
    let items: Result<Vec<_>, _> = db_state
        .client
        .scan()
        .table_name(&db_state.table)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;

    // Create the authorization token
    // let token = encode(&Header::default(), &claims, &keys::encoding())
    //     .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    // Ok(Json(AuthBody::new(token)))
    Ok(Json(
        items.unwrap()[0]
            .get("user")
            .unwrap()
            .as_s()
            .unwrap()
            .to_string(),
    ))
}
