use aide::transform::TransformOperation;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use entity::ban;
use schemars::JsonSchema;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateBanRequest {
    pub active: bool,
    pub banner_user_id: Uuid,
    pub post_id: Option<Uuid>,
    pub file_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_identifier_id: Option<Uuid>,
    pub soft_ban: bool,
    pub public_ban: bool,
    pub soft_reason: Option<String>,
    pub public_reason: Option<String>,
    pub warn_reason: Option<String>,
    pub staff_note: Option<String>,
    pub additional_info: Option<Value>,
    pub expires: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct UpdateBanRequest {
    pub active: Option<bool>,
    pub banner_user_id: Option<Uuid>,
    pub post_id: Option<Uuid>,
    pub file_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_identifier_id: Option<Uuid>,
    pub soft_ban: Option<bool>,
    pub public_ban: Option<bool>,
    pub soft_reason: Option<String>,
    pub public_reason: Option<String>,
    pub warn_reason: Option<String>,
    pub staff_note: Option<String>,
    pub additional_info: Option<Value>,
    pub expires: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BanResponse {
    pub id: Uuid,
    pub active: bool,
    pub banner_user_id: Uuid,
    pub post_id: Option<Uuid>,
    pub file_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_identifier_id: Option<Uuid>,
    pub soft_ban: bool,
    pub public_ban: bool,
    pub soft_reason: Option<String>,
    pub public_reason: Option<String>,
    pub warn_reason: Option<String>,
    pub expires: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<ban::Model> for BanResponse {
    fn from(model: ban::Model) -> Self {
        Self {
            id: model.id,
            active: model.active,
            banner_user_id: model.banner_user_id,
            post_id: model.post_id,
            file_id: model.file_id,
            user_id: model.user_id,
            user_identifier_id: model.user_identifier_id,
            soft_ban: model.soft_ban,
            public_ban: model.public_ban,
            soft_reason: model.soft_reason,
            public_reason: model.public_reason,
            warn_reason: model.warn_reason,
            expires: model.expires,
            created_at: model.created_at,
        }
    }
}

pub async fn create_ban(
    State(state): State<AppState>,
    Json(req): Json<CreateBanRequest>,
) -> (StatusCode, Json<BanResponse>) {
    let ban_model = ban::ActiveModel {
        id: sea_orm::Set(Uuid::now_v7()),
        active: sea_orm::Set(req.active),
        banner_user_id: sea_orm::Set(req.banner_user_id),
        post_id: sea_orm::Set(req.post_id),
        file_id: sea_orm::Set(req.file_id),
        user_id: sea_orm::Set(req.user_id),
        user_identifier_id: sea_orm::Set(req.user_identifier_id),
        soft_ban: sea_orm::Set(req.soft_ban),
        public_ban: sea_orm::Set(req.public_ban),
        soft_reason: sea_orm::Set(req.soft_reason),
        public_reason: sea_orm::Set(req.public_reason),
        warn_reason: sea_orm::Set(req.warn_reason),
        staff_note: sea_orm::Set(req.staff_note),
        additional_info: sea_orm::Set(req.additional_info),
        expires: sea_orm::Set(req.expires),
        ..Default::default()
    };

    match ban_model.insert(&state.db_conn).await {
        Ok(result) => (StatusCode::CREATED, Json(result.into())),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(build_error_ban_response()),
        ),
    }
}

fn build_error_ban_response() -> BanResponse {
    BanResponse {
        id: Uuid::nil(),
        active: false,
        banner_user_id: Uuid::nil(),
        post_id: None,
        file_id: None,
        user_id: None,
        user_identifier_id: None,
        soft_ban: false,
        public_ban: false,
        soft_reason: None,
        public_reason: None,
        warn_reason: None,
        expires: None,
        created_at: DateTime::UNIX_EPOCH,
    }
}

pub async fn list_bans(State(state): State<AppState>) -> Json<Vec<BanResponse>> {
    match ban::Entity::find().all(&state.db_conn).await {
        Ok(bans) => {
            let responses = bans.into_iter().map(|b| b.into()).collect();
            Json(responses)
        }
        Err(_) => Json(vec![]),
    }
}

pub async fn get_ban(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<Option<BanResponse>>) {
    match ban::Entity::find_by_id(id).one(&state.db_conn).await {
        Ok(Some(ban)) => (StatusCode::OK, Json(Some(ban.into()))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn update_ban(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateBanRequest>,
) -> (StatusCode, Json<Option<BanResponse>>) {
    let ban_result = ban::Entity::find_by_id(id).one(&state.db_conn).await;

    match ban_result {
        Ok(Some(ban)) => {
            let mut ban_active: ban::ActiveModel = ban.into_active_model();

            if let Some(active) = req.active {
                ban_active.active = sea_orm::Set(active);
            }
            if let Some(banner_user_id) = req.banner_user_id {
                ban_active.banner_user_id = sea_orm::Set(banner_user_id);
            }
            if let Some(post_id) = req.post_id {
                ban_active.post_id = sea_orm::Set(Some(post_id));
            }
            if let Some(file_id) = req.file_id {
                ban_active.file_id = sea_orm::Set(Some(file_id));
            }
            if let Some(user_id) = req.user_id {
                ban_active.user_id = sea_orm::Set(Some(user_id));
            }
            if let Some(user_identifier_id) = req.user_identifier_id {
                ban_active.user_identifier_id = sea_orm::Set(Some(user_identifier_id));
            }
            if let Some(soft_ban) = req.soft_ban {
                ban_active.soft_ban = sea_orm::Set(soft_ban);
            }
            if let Some(public_ban) = req.public_ban {
                ban_active.public_ban = sea_orm::Set(public_ban);
            }
            if let Some(soft_reason) = req.soft_reason {
                ban_active.soft_reason = sea_orm::Set(Some(soft_reason));
            }
            if let Some(public_reason) = req.public_reason {
                ban_active.public_reason = sea_orm::Set(Some(public_reason));
            }
            if let Some(warn_reason) = req.warn_reason {
                ban_active.warn_reason = sea_orm::Set(Some(warn_reason));
            }
            if let Some(staff_note) = req.staff_note {
                ban_active.staff_note = sea_orm::Set(Some(staff_note));
            }
            if let Some(additional_info) = req.additional_info {
                ban_active.additional_info = sea_orm::Set(Some(additional_info));
            }
            if let Some(expires) = req.expires {
                ban_active.expires = sea_orm::Set(Some(expires));
            }

            match ban_active.update(&state.db_conn).await {
                Ok(updated) => (StatusCode::OK, Json(Some(updated.into()))),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn delete_ban(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let ban_result = ban::Entity::find_by_id(id).one(&state.db_conn).await;

    match ban_result {
        Ok(Some(ban)) => match ban.delete(&state.db_conn).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Ok(None) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// Documentation functions

pub fn create_ban_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new ban")
        .response::<201, Json<BanResponse>>()
        .response_with::<400, Json<AppError>, _>(|res| res.description("Invalid request"))
}

pub fn list_bans_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all bans")
        .response::<200, Json<Vec<BanResponse>>>()
}

pub fn get_ban_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a specific ban by ID")
        .response::<200, Json<BanResponse>>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Ban not found"))
}

pub fn update_ban_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update a ban")
        .response::<200, Json<BanResponse>>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Ban not found"))
}

pub fn delete_ban_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a ban")
        .response::<204, ()>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Ban not found"))
}
