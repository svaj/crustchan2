use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// User bans of a post or file or user
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Ban {
    pub id: Uuid,
    pub active: bool, // if false, ban is inactive and should not be enforced, but is kept for record-keeping
    pub banner_user_id: Uuid, // user who issued the ban, null if system ban
    pub post_id: Option<Uuid>, // post being banned.
    pub file_id: Option<Uuid>, // file being banned.
    pub user_id: Option<Uuid>, // user being banned.
    pub soft_ban: bool, // if true, content is only visible to staff and creator, not public.
    pub public_ban: bool, // stripped from public view except for ban info.
    pub soft_reason: String, // visibile to staff and creator.
    pub public_reason: Option<String>, // visibile to public, e.g. "Inappropriate content", "Spam", etc. Optional because some bans may be silent and not visible to public
    pub warn_reason: Option<String>,   // visibile to creator only
    pub staff_note: Option<String>,    // visibile to staff only

    pub additional_info: Option<Value>, // any additional info about the ban, e.g. evidence, notes, etc.
    pub expires: Option<DateTime<Utc>>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}
