use chrono::{Duration, Utc};
use domain::models::auth::{AuthService, AuthServiceError, Claims};
use domain::models::user::UserId;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub struct JwtAuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtAuthService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
}

impl AuthService for JwtAuthService {
    fn issue_token(&self, user_id: UserId) -> Result<String, AuthServiceError> {
        let now = Utc::now();
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
