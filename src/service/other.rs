use axum::{async_trait, extract::FromRequestParts, RequestPartsExt, TypedHeader};
use headers::{authorization::Bearer, Authorization};
use http::request::Parts;
use jsonwebtoken::{decode, Validation};

use crate::{error::AuthError, keys, types::Claims};

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!("Welcome to the protected area :)\nYour data:\n ",))
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data =
            decode::<Claims>(bearer.token(), &keys::decoding(), &Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
