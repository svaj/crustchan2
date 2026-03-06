use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// A thread is an initial posting on a board, which will have 1-n posts.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Thread {
    pub id: Uuid,
    pub subject: String,
    pub board_id: Uuid,
    pub user_id: Option<Uuid>,
    pub user_identifiers_id: Option<Uuid>, // user_identifiers_id of the poster, for IP tracking and bans, etc.

    pub deleted: bool, // if true, post is deleted and only visible to admins, does not show for users
    pub ai_slop: bool, // if true, flags as ai-generated content
    pub approved: bool, // manually approved by mods, visible to everyone, bypasses filters
    pub rejected: bool, // auto rejected by filters or manually rejected by mods, not visible to anyone
    pub locked: bool,
    pub sticky: bool,

    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub last_bumped_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub last_replied_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ThreadInput {
    pub subject: String,
    pub board_id: Uuid,
}

impl From<ThreadInput> for Thread {
    fn from(post: ThreadInput) -> Thread {
        Thread {
            id: Uuid::now_v7(),
            subject: post.subject,
            board_id: post.board_id,
            user_id: None,
            user_identifiers_id: None,
            deleted: false,
            ai_slop: false,
            approved: false,
            rejected: false,
            locked: false,
            sticky: false,
            created_at: chrono::offset::Utc::now(),
            last_bumped_at: chrono::offset::Utc::now(),
            last_replied_at: chrono::offset::Utc::now(),
        }
    }
}

impl Default for Thread {
    fn default() -> Thread {
        Thread {
            id: Uuid::now_v7(),
            subject: "".to_string(),
            board_id: Uuid::now_v7(),
            user_id: None,
            user_identifiers_id: None,
            deleted: false,
            locked: false,
            approved: false,
            rejected: false,
            sticky: false,
            ai_slop: false,
            created_at: chrono::offset::Utc::now(),
            last_bumped_at: chrono::offset::Utc::now(),
            last_replied_at: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FetchThreadInput {
    id: String,
}
