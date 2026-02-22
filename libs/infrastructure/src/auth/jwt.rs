use std::sync::Arc;

use chrono::Duration;
use domain::clock::Clock;
use domain::models::user::UserId;
use domain::usecase::auth::{AuthService, AuthServiceError, AuthToken, Claims};
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
    fn issue_token(&self, user_id: UserId) -> Result<AuthToken, AuthServiceError> {
        let now = self.clock.now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::hours(24)).timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            iat,
            exp,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map(AuthToken::from)
            .map_err(|e| AuthServiceError::IssuanceFailed(anyhow::Error::from(e)))
    }

    fn verify_token(&self, token: &AuthToken) -> Result<Claims, AuthServiceError> {
        decode::<Claims>(
            token.expose_as_str(),
            &self.decoding_key,
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| match *e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthServiceError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::InvalidIssuer
            | jsonwebtoken::errors::ErrorKind::InvalidAudience
            | jsonwebtoken::errors::ErrorKind::InvalidSubject => AuthServiceError::InvalidToken,
            _ => AuthServiceError::VerificationFailed(anyhow::Error::from(e)),
        })
    }
}
