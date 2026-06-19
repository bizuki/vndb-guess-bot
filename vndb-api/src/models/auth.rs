use serde::Deserialize;

use crate::ids::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AuthInfo {
    pub id: UserId,
    pub username: String,
    pub permissions: Vec<String>,
}
