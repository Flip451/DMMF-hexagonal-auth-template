pub mod error;
pub use error::{EmailError, PasswordError, UserError};

use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    // placeholder
}
