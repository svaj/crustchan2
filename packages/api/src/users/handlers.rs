use aide::transform::TransformOperation;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use entity::user;
use schemars::JsonSchema;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub is_ai: Option<bool>,
    pub is_human: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct LoginUserRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub is_ai: Option<bool>,
    pub is_human: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub is_ai: bool,
    pub is_human: bool,
}

impl From<user::Model> for UserResponse {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            display_name: model.display_name,
            email: model.email,
            email_verified: model.email_verified,
            is_ai: model.is_ai,
            is_human: model.is_human,
        }
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    let user_model = user::ActiveModel {
        id: sea_orm::Set(Uuid::now_v7()),
        username: sea_orm::Set(req.username),
        password_hash: sea_orm::Set(req.password_hash),
        display_name: sea_orm::Set(req.display_name),
        email: sea_orm::Set(req.email),
        email_verified: sea_orm::Set(false),
        is_ai: sea_orm::Set(req.is_ai.unwrap_or(false)),
        is_human: sea_orm::Set(req.is_human.unwrap_or(false)),
        ..Default::default()
    };

    match user_model.insert(&state.db_conn).await {
        Ok(result) => (StatusCode::CREATED, Json(result.into())),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UserResponse {
                id: Uuid::nil(),
                username: "Error".to_string(),
                display_name: None,
                email: None,
                email_verified: false,
                is_ai: false,
                is_human: false,
            }),
        ),
    }
}

pub fn hash_password(raw_text:String) -> String {
    raw_text
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(req): Json<LoginUserRequest>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    match user::Entity::find()
        .filter(user::Column::Username.eq(req.username))
        .filter(user::Column::PasswordHash.eq(hash_password(req.password)))
        .one(&state.db_conn)
        .await
    {
        Ok(Some(user)) => (StatusCode::OK, Json(Some(user.into()))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn list_users(State(state): State<AppState>) -> Json<Vec<UserResponse>> {
    match user::Entity::find().all(&state.db_conn).await {
        Ok(users) => {
            let responses = users.into_iter().map(|u| u.into()).collect();
            Json(responses)
        }
        Err(_) => Json(vec![]),
    }
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    match user::Entity::find_by_id(id)
        .one(&state.db_conn)
        .await
    {
        Ok(Some(user)) => (StatusCode::OK, Json(Some(user.into()))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    let user_result = user::Entity::find_by_id(id)
        .one(&state.db_conn)
        .await;

    match user_result {
        Ok(Some(user)) => {
            let mut user_active: user::ActiveModel = user.into_active_model();

            if let Some(display_name) = req.display_name {
                user_active.display_name = sea_orm::Set(Some(display_name));
            }
            if let Some(email) = req.email {
                user_active.email = sea_orm::Set(Some(email));
            }
            if let Some(email_verified) = req.email_verified {
                user_active.email_verified = sea_orm::Set(email_verified);
            }
            if let Some(is_ai) = req.is_ai {
                user_active.is_ai = sea_orm::Set(is_ai);
            }
            if let Some(is_human) = req.is_human {
                user_active.is_human = sea_orm::Set(is_human);
            }

            match user_active.update(&state.db_conn).await {
                Ok(updated) => (StatusCode::OK, Json(Some(updated.into()))),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let user_result = user::Entity::find_by_id(id)
        .one(&state.db_conn)
        .await;

    match user_result {
        Ok(Some(user)) => {
            if let Ok(_) = user.delete(&state.db_conn).await {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        Ok(None) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// Documentation functions



pub fn login_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Authenticate a user")
        .response::<201, Json<UserResponse>>()
        .response_with::<400, Json<AppError>, _>(|res| res.description("Invalid request"))
}


pub fn create_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new user")
        .response::<201, Json<UserResponse>>()
        .response_with::<400, Json<AppError>, _>(|res| res.description("Invalid request"))
}

pub fn list_users_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all users").response::<200, Json<Vec<UserResponse>>>()
}

pub fn get_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a specific user by ID")
        .response::<200, Json<UserResponse>>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("User not found"))
}

pub fn update_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update a user")
        .response::<200, Json<UserResponse>>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("User not found"))
}

pub fn delete_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a user")
        .response::<204, ()>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("User not found"))
}
