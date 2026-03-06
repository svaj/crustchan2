use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: Uuid,

    pub additional_info: Option<Value>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl Default for UserProfile {
    fn default() -> UserProfile {
        UserProfile {
            id: Uuid::now_v7(),
            additional_info: None,
            created_at: chrono::offset::Utc::now(),
        }
    }
}
