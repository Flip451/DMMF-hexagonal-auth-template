use sensitive_data::{EmailRule, SecretRule, Sensitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupCommand {
    pub email: Sensitive<String, EmailRule>,
    pub password: Sensitive<String, SecretRule>,
}
