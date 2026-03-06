use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_type = "Uuid")]
    pub id: String,
    pub board_post_number: u64, // post number within the board, from sequence on board.

    pub board_id: Uuid, // board this post belongs to
    #[sea_orm(belongs_to, from = "board_id", to = "id")]
    pub board: HasOne<super::board::Entity>,

    pub thread_id: Uuid, // thread this post belongs to, null if op
    #[sea_orm(belongs_to, from = "thread_id", to = "id")]
    pub thread: HasOne<super::thread::Entity>,

    pub poster_display_name: String, // display name of the poster, (Anon, "John Doe", "My Username")

    pub user_id: Option<Uuid>, // id of the user who made the post, null if not logged in
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    pub user_identifier_id: Option<Uuid>, // user id who created
    #[sea_orm(belongs_to, from = "user_identifier_id", to = "id")]
    pub user_identifier: HasOne<super::user_identifier::Entity>,

    pub deleted: bool, // if true, post is deleted and only visible to admins, does not show for users

    pub ai_slop: bool,  // if true, flags as ai-generated content
    pub approved: bool, // manually approved by mods, visible to everyone, bypasses filters
    pub rejected: bool, // auto rejected by filters or manually rejected by mods, not visible to anyone
    pub sticky: bool,   // shows at top of thread, sorted by creation time, not bumped by new posts

    pub bumped: bool,  // if true, thread was bumped by this post
    pub spoiler: bool, // if true, post contains spoilers, hides images and text until user clicks

    pub op_id: Uuid, // id of post this is in reply to
    // pub op: HasOne<super::post::Entity>
    #[sea_orm(column_type = "Text")]
    pub text: String,

    pub additional_info: Option<Value>,

    // // HasBans
    #[sea_orm(has_many)]
    pub bans: HasMany<super::ban::Entity>,
    // // hasfiles
    #[sea_orm(has_many)]
    pub files: HasMany<super::file::Entity>,

    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
