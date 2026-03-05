use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// User gets banned for something they posted, this is a document of the ban
#[derive(Debug, Serialize, Clone, Deserialize,ToSchema)]
pub struct Report {
    pub id: String,
}