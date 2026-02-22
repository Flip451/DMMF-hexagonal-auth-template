use std::sync::Arc;

use chrono::Duration;
use domain::clock::Clock;
use domain::models::auth::{AuthService, AuthServiceError, Claims};
use domain::models::user::UserId;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub struct JwtAuthService<C: Clock> {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    clock: Arc<C>,
}

impl<C: Clock> JwtAuthService<C> {
    pub fn new(secret: &str, clock: Arc<C>) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            clock,
        }
    }
}

impl<C: Clock> AuthService for JwtAuthService<C> {
    fn issue_token(&self, user_id: UserId) -> Result<String, AuthServiceError> {
        let now = self.clock.now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::hours(24)).timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            iat,
            exp,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthServiceError::IssuanceFailed(e.into()))
    }

    fn verify_token(&self, token: &str) -> Result<Claims, AuthServiceError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| match *e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthServiceError::TokenExpired,
                jsonwebtoken::errors::ErrorKind::InvalidToken
                | jsonwebtoken::errors::ErrorKind::InvalidSignature
                | jsonwebtoken::errors::ErrorKind::InvalidIssuer
                | jsonwebtoken::errors::ErrorKind::InvalidAudience
                | jsonwebtoken::errors::ErrorKind::InvalidSubject => AuthServiceError::InvalidToken,
                _ => AuthServiceError::VerificationFailed(e.into()),
            })
    }
}
