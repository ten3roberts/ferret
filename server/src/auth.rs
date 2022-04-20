use std::sync::mpsc;

use axum::{
    async_trait,
    extract::{
        rejection::{StringRejection, TypedHeaderRejection},
        FromRequest, RequestParts, TypedHeader,
    },
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{self, AlgorithmParameters, JwkSet},
    DecodingKey, EncodingKey, Header, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};

impl std::fmt::Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\n", self.sub)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

static JWKS: OnceCell<JwkSet> = OnceCell::const_new();
// tokio::spawn(async move {
// });

// });

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        tracing::info!("Validating claims");

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|e| AuthError::HeaderError(e))?;

        let token = bearer.token();
        dbg!(token);
        let header = decode_header(token).map_err(|e| AuthError::InvalidToken(e))?;
        let kid = match header.kid {
            Some(k) => k,
            None => return Err(AuthError::MissingKid),
        };

        tracing::info!("Parsed kid");

        let jwks = JWKS
            .get_or_init(|| async move {
                let response =
                    reqwest::get("https://dev-cqwzutzq.us.auth0.com/.well-known/jwks.json")
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                serde_json::from_str(&response).unwrap()
            })
            .await;

        if let Some(j) = jwks.find(&kid) {
            match j.algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                    let mut validation = Validation::new(j.common.algorithm.unwrap());
                    validation.validate_exp = false;
                    let token = decode::<Self>(token, &decoding_key, &validation)
                        .map_err(|e| AuthError::InvalidToken(e))?
                        .claims;

                    tracing::info!("{:?}", token);
                    Ok(token)
                }
                _ => {
                    tracing::error!("Token is not RSA encrypted");
                    unreachable!("this should be a RSA")
                }
            }
        } else {
            tracing::error!("Missing jwks for kid");
            return Err(AuthError::MissingKid);
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::MissingCredentials => StatusCode::BAD_REQUEST,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken(_) => StatusCode::BAD_REQUEST,
            AuthError::HeaderError(_) => StatusCode::BAD_REQUEST,
            AuthError::MissingKid => StatusCode::BAD_REQUEST,
        };
        let body = Json(json!({
            "error": self.to_string(),
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Wrong credetials")]
    WrongCredentials,
    #[error("Missing kid")]
    MissingKid,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Failed to create token")]
    TokenCreation,
    #[error("Malformed auth header {0}")]
    HeaderError(TypedHeaderRejection),
    #[error("Invalid token: {0}")]
    InvalidToken(jsonwebtoken::errors::Error),
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
