use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserIdentifiers {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub user_agent: Option<String>,
    pub additional_info: Option<String>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub last_active: DateTime<Utc>,
}

impl Default for UserIdentifiers {
    fn default() -> UserIdentifiers {
        UserIdentifiers {
            id: Uuid::now_v7(),
            user_id: None,
            user_agent: None,
            additional_info: None,
            created_at: chrono::offset::Utc::now(),
            last_active: chrono::offset::Utc::now(),
        }
    }
}
