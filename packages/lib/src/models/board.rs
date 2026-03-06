use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Board {
    pub id: Uuid,
    pub long_name: String,             // e.g. "Random"
    pub short_name: String,            // e.g. "b" for "/b/"
    pub description: String,           // e.g. "A board for random discussion"
    pub rules: Option<String>,         // Board rules in markdown format
    pub sfw: bool,                     // Safe for work content only
    pub users_only: bool,              // Only registered users can post
    pub anon_only: bool,               // Only anonymous users can post / force anonymous posting
    pub ai_only: bool,                 // Only AI-generated content allowed
    pub human_only: bool,              // Only human-generated content allowed
    pub archived: bool, // No new threads allowed, but existing threads are still visible
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

    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BoardInput {
    pub long_name: String,
    pub short_name: String,
    pub description: String,
    pub rules: Option<String>,

    // poster / content restrictions
    pub sfw: bool,
    pub users_only: bool,
    pub anon_only: bool,
    pub ai_only: bool,
    pub human_only: bool,
    pub min_image_width: Option<u32>, // minimum image width in pixels
    pub min_image_height: Option<u32>, // minimum image height in pixels
    pub max_image_width: Option<u32>, // maximum image width in pixels
    pub max_image_height: Option<u32>, // maximum image height in pixels
    pub max_filesize: Option<u64>,    // maximum image filesize in bytes

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
}

impl From<BoardInput> for Board {
    fn from(board: BoardInput) -> Board {
        Board {
            id: Uuid::now_v7(),
            long_name: board.long_name,
            short_name: board.short_name,
            description: board.description,
            rules: board.rules,
            // content restrictions, high level
            sfw: board.sfw,
            users_only: board.users_only,
            anon_only: board.anon_only,
            ai_only: board.ai_only,
            human_only: board.human_only,
            archived: false,
            current_post_sequence: 0,

            min_image_width: None,  // minimum image width in pixels
            min_image_height: None, // minimum image height in pixels
            max_image_width: None,  // maximum image width in pixels
            max_image_height: None, // maximum image height in pixels
            max_filesize: None,     // maximum image filesize in bytes

            max_thread_bumps: board.max_thread_bumps, // maximum number of times a thread can be bumped by new posts
            max_thread_replies: board.max_thread_replies, // maximum number of replies in a thread before it is auto-locked
            threads_per_page: board.threads_per_page,     // number of threads to show per page
            max_files_per_post: board.max_files_per_post, // maximum number of files allowed per
            min_files_per_thread: board.min_files_per_thread, // minimum number of files required in a thread
            min_files_per_post: board.min_files_per_post, // minimum number of files required in a p
            max_post_length: board.max_post_length, // maximum number of characters allowed in a post
            max_thread_subject_length: board.max_thread_subject_length, // maximum number of characters allowed in
            max_webm_duration: board.max_webm_duration, // maximum duration of webm files in seconds
            allow_audio_posts: board.allow_audio_posts, // if true, users can upload audio files (webm again)
            allow_spoiler_posts: board.allow_spoiler_posts, // if true, users can mark posts and files as spoilers, hiding them until clicked
            allow_spoiler_images: board.allow_spoiler_images, // if true, users can mark files as spoilers, hiding them until clicked

            created_at: chrono::offset::Utc::now(),
        }
    }
}
