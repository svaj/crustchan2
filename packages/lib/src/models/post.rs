use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::models::file::FileInput;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub board_post_number: u64, // post number within the board, from sequence on board.
    pub board_id: Uuid,
    pub thread_id: Uuid,
    pub poster_display_name: String, // display name of the poster, (Anon, "John Doe", "My Username")
    pub user_id: Option<Uuid>,       // id of the user who made the post, null if not logged in
    pub user_identifiers_id: Option<Uuid>, // HasUserIdentifier

    pub deleted: bool, // if true, post is deleted and only visible to admins, does not show for users

    pub ai_slop: bool,  // if true, flags as ai-generated content
    pub approved: bool, // manually approved by mods, visible to everyone, bypasses filters
    pub rejected: bool, // auto rejected by filters or manually rejected by mods, not visible to anyone
    pub sticky: bool,   // shows at top of thread, sorted by creation time, not bumped by new posts

    pub bumped: bool,        // if true, thread was bumped by this post
    pub spoiler: bool, // if true, post contains spoilers, hides images and text until user clicks
    pub op_id: Option<Uuid>, // id of post this is in reply to
    pub text: String,

    pub additional_info: Option<Value>,

    // HasBans
    // hasfiles
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PostInput {
    pub board_id: Uuid,
    pub thread_id: Uuid,
    pub poster_display_name: String,
    pub ip_hash: String,
    pub js_fingerprint: Option<String>,
    pub spoiler: bool,

    pub op_id: Option<Uuid>,
    pub additional_info: Option<Value>,
    pub text: String,
    pub files: Vec<FileInput>,
}

impl From<PostInput> for Post {
    fn from(post: PostInput) -> Post {
        Post {
            id: Uuid::now_v7(),
            board_post_number: 0,
            text: post.text,
            board_id: post.board_id,
            thread_id: post.thread_id,
            user_id: None,
            user_identifiers_id: None,
            poster_display_name: post.poster_display_name,
            deleted: false,
            approved: false,
            rejected: false,
            sticky: false,
            bumped: false,
            ai_slop: false,
            spoiler: post.spoiler,
            op_id: post.op_id,
            additional_info: post.additional_info,
            created_at: chrono::offset::Utc::now(),
        }
    }
}

impl Default for Post {
    fn default() -> Post {
        Post {
            id: Uuid::now_v7(),
            board_post_number: 0,
            text: "".to_string(),
            board_id: Uuid::now_v7(),
            thread_id: Uuid::now_v7(),
            user_id: None,
            user_identifiers_id: None,
            poster_display_name: "Anonymous".to_string(),
            deleted: false,
            approved: false,
            rejected: false,
            sticky: false,
            bumped: false,
            ai_slop: false,
            spoiler: false,
            op_id: None,
            additional_info: None,
            created_at: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FetchPostInput {
    id: String,
}
