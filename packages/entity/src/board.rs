use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "boards")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub long_name: String,  // e.g. "Random"
    pub short_name: String, // e.g. "b" for "/b/"
    #[sea_orm(column_type = "Text")]
    pub description: String, // e.g. "A board for random discussion"
    #[sea_orm(column_type = "Text", nullable)]
    pub rules: Option<String>, // Board rules in markdown format
    pub sfw: bool,          // Safe for work content only
    pub users_only: bool,   // Only registered users can post
    pub anon_only: bool,    // Only anonymous users can post / force anonymous posting
    pub ai_only: bool,      // Only AI-generated content allowed
    pub human_only: bool,   // Only human-generated content allowed
    pub archived: bool,     // No new threads allowed, but existing threads are still visible
    pub current_post_sequence: u64, // current thread/post number sequence for this board, used to generate post numbers
    pub min_image_width: Option<u32>, // minimum image width in pixels
    pub min_image_height: Option<u32>, // minimum image height in pixels
    pub max_image_width: Option<u32>, // maximum image width in pixels
    pub max_image_height: Option<u32>, // maximum image height in pixels
    pub max_filesize: Option<u64>,  // maximum image filesize in bytes

    pub max_thread_bumps: Option<u32>, // maximum number of times a thread can be bumped by new posts
    pub max_thread_replies: Option<u32>, // maximum number of replies in a thread before it is auto-locked
    pub threads_per_page: u32,           // number of threads to show per page
    pub max_files_per_post: Option<u32>, // maximum number of files allowed per
    pub min_files_per_thread: Option<u32>, // minimum number of files required in a thread
    pub min_files_per_post: Option<u32>, // minimum number of files required in a post

    pub max_post_length: Option<u32>, // maximum number of characters allowed in a post
    pub max_thread_subject_length: Option<u32>, // maximum number of characters allowed in
    pub max_webm_duration: Option<u32>, // maximum duration of webm files in seconds
    pub allow_audio_posts: bool,      // if true, users can upload audio files (webm again)

    pub allow_spoiler_posts: bool, // if true, users can mark posts and files as spoilers, hiding them until clicked
    pub allow_spoiler_images: bool, // if true, users can mark files as spoilers, hiding them until clicked

    #[sea_orm(has_many)]
    pub threads: HasMany<super::thread::Entity>,

    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
