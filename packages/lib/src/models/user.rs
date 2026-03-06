use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub is_ai: bool,
    pub is_human: bool,
    // HasBans
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub last_login: DateTime<Utc>,
}

impl Default for User {
    fn default() -> User {
        User {
            id: Uuid::now_v7(),
            username: "".to_string(),
            password_hash: "".to_string(),
            display_name: None,
            email: None,
            email_verified: false,
            is_ai: false,
            is_human: true,
            created_at: chrono::offset::Utc::now(),
            last_login: chrono::offset::Utc::now(),
        }
    }
}
