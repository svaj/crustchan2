use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct File {
    pub id: Uuid,

    pub post_id: String,
    pub user_id: Option<Uuid>,
    pub user_identifiers_id: Option<Uuid>, // HasUserIdentifier

    pub approved: bool, // manually approved by staff, available on site (unless banned later)
    pub rejected: bool, // auto rejected by filters not available on site
    pub deleted: bool,  // not available on site

    // HasBans
    pub spoiler: bool, // whether the file is marked as a spoiler, if true, the file will be blurred and require user interaction to view

    pub description: Option<String>,
    pub file_display_name: String,
    pub gallery_order: Option<u32>,
    pub file_size: u64, // size of the file after processing (resizing, compression, etc), this is the size of the file that is actually available on the site
    pub file_dimensions: String,
    pub file_original_name: String,
    pub file_original_size: u64, // size of the file uploaded to us

    pub additional_info: Option<Value>, // exif, ai analysis scores, etc.
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FileInput {
    pub post_id: String,
    pub file_display_name: String,
    pub description: Option<String>,
    pub gallery_order: Option<u32>,
    pub file_size: u64,
    pub file_dimensions: String,
    pub file_original_name: String,
    pub file_original_size: u64,
    pub spoiler: bool,
}

impl From<FileInput> for File {
    fn from(file: FileInput) -> File {
        File {
            id: Uuid::now_v7(),
            post_id: file.post_id,
            user_id: None,
            user_identifiers_id: None,
            deleted: false,
            approved: false,
            rejected: false,
            spoiler: file.spoiler,
            file_display_name: file.file_display_name,
            description: file.description,
            gallery_order: file.gallery_order,
            file_size: file.file_size,
            file_dimensions: file.file_dimensions,
            file_original_name: file.file_original_name,
            file_original_size: file.file_size,
            additional_info: None,
            created_at: chrono::offset::Utc::now(),
        }
    }
}

impl Default for File {
    fn default() -> File {
        File {
            id: Uuid::now_v7(),
            post_id: "".to_string(),
            user_id: None,
            user_identifiers_id: None,

            deleted: false,

            approved: false,
            rejected: false,

            spoiler: false,
            description: None,
            gallery_order: None,
            file_display_name: "".to_string(),
            file_size: 0,
            file_dimensions: "".to_string(),
            file_original_name: "".to_string(),
            file_original_size: 0,
            additional_info: None,
            created_at: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FetchFileInput {
    id: String,
}
