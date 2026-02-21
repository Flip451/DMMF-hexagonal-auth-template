use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginQuery {
    pub email: String,
    pub password: String,
}
