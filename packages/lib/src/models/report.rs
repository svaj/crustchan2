use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// User reports of a post or file
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub post_id: Option<Uuid>,
    pub file_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub reason: String,
    pub reporter_user_id: Option<Uuid>,
    pub reporter_user_identifiers_id: Option<Uuid>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}
