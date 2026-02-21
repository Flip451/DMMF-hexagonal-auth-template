use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupCommand {
    pub email: String,
    pub password: String,
}
