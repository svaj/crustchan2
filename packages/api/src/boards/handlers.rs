use aide::transform::TransformOperation;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use entity::board;
use schemars::JsonSchema;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateBoardRequest {
    pub category_id: Uuid,
    pub long_name: String,
    pub short_name: String,
    pub description: String,
    pub rules: Option<String>,
    pub sfw: Option<bool>,
    pub users_only: Option<bool>,
    pub anon_only: Option<bool>,
    pub ai_only: Option<bool>,
    pub human_only: Option<bool>,
    pub threads_per_page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct UpdateBoardRequest {
    pub long_name: Option<String>,
    pub description: Option<String>,
    pub rules: Option<String>,
    pub sfw: Option<bool>,
    pub users_only: Option<bool>,
    pub anon_only: Option<bool>,
    pub threads_per_page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BoardResponse {
    pub id: Uuid,
    pub category_id: Uuid,
    pub long_name: String,
    pub short_name: String,
    pub description: String,
    pub sfw: bool,
    pub users_only: bool,
    pub anon_only: bool,
    pub archived: bool,
    pub threads_per_page: u32,
}

impl From<board::Model> for BoardResponse {
    fn from(model: board::Model) -> Self {
        Self {
            id: model.id,
            category_id: model.category_id,
            long_name: model.long_name,
            short_name: model.short_name,
            description: model.description,
            sfw: model.sfw,
            users_only: model.users_only,
            anon_only: model.anon_only,
            archived: model.archived,
            threads_per_page: model.threads_per_page,
        }
    }
}

pub async fn create_board(
    State(state): State<AppState>,
    Json(req): Json<CreateBoardRequest>,
) -> (StatusCode, Json<BoardResponse>) {
    let board_model = board::ActiveModel {
        id: sea_orm::Set(Uuid::now_v7()),
        category_id: sea_orm::Set(req.category_id),
        long_name: sea_orm::Set(req.long_name),
        short_name: sea_orm::Set(req.short_name),
        description: sea_orm::Set(req.description),
        rules: sea_orm::Set(req.rules),
        sfw: sea_orm::Set(req.sfw.unwrap_or(true)),
        users_only: sea_orm::Set(req.users_only.unwrap_or(false)),
        anon_only: sea_orm::Set(req.anon_only.unwrap_or(false)),
        ai_only: sea_orm::Set(false),
        human_only: sea_orm::Set(false),
        archived: sea_orm::Set(false),
        current_post_sequence: sea_orm::Set(0),
        threads_per_page: sea_orm::Set(req.threads_per_page.unwrap_or(10)),
        ..Default::default()
    };

    match board_model.insert(&state.db_conn).await {
        Ok(result) => (StatusCode::CREATED, Json(result.into())),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(BoardResponse {
                id: Uuid::nil(),
                category_id: Uuid::nil(),
                long_name: "Error".to_string(),
                short_name: "err".to_string(),
                description: "Failed to create board".to_string(),
                sfw: true,
                users_only: false,
                anon_only: false,
                archived: false,
                threads_per_page: 0,
            }),
        ),
    }
}

pub async fn list_boards(State(state): State<AppState>) -> Json<Vec<BoardResponse>> {
    match board::Entity::find().all(&state.db_conn).await {
        Ok(boards) => {
            let responses = boards.into_iter().map(|b| b.into()).collect();
            Json(responses)
        }
        Err(_) => Json(vec![]),
    }
}

pub async fn get_board(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<Option<BoardResponse>>) {
    match board::Entity::find_by_id(id).one(&state.db_conn).await {
        Ok(Some(board)) => (StatusCode::OK, Json(Some(board.into()))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn update_board(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateBoardRequest>,
) -> (StatusCode, Json<Option<BoardResponse>>) {
    let board_result = board::Entity::find_by_id(id).one(&state.db_conn).await;

    match board_result {
        Ok(Some(board)) => {
            let mut board_active: board::ActiveModel = board.into_active_model();

            if let Some(long_name) = req.long_name {
                board_active.long_name = sea_orm::Set(long_name);
            }
            if let Some(description) = req.description {
                board_active.description = sea_orm::Set(description);
            }
            if let Some(rules) = req.rules {
                board_active.rules = sea_orm::Set(Some(rules));
            }
            if let Some(sfw) = req.sfw {
                board_active.sfw = sea_orm::Set(sfw);
            }
            if let Some(users_only) = req.users_only {
                board_active.users_only = sea_orm::Set(users_only);
            }
            if let Some(anon_only) = req.anon_only {
                board_active.anon_only = sea_orm::Set(anon_only);
            }
            if let Some(threads_per_page) = req.threads_per_page {
                board_active.threads_per_page = sea_orm::Set(threads_per_page);
            }

            match board_active.update(&state.db_conn).await {
                Ok(updated) => (StatusCode::OK, Json(Some(updated.into()))),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn delete_board(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let board_result = board::Entity::find_by_id(id).one(&state.db_conn).await;

    match board_result {
        Ok(Some(board)) => {
            if let Ok(_) = board.delete(&state.db_conn).await {
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

pub fn create_board_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new board")
        .response::<201, Json<BoardResponse>>()
        .response_with::<400, Json<AppError>, _>(|res| res.description("Invalid request"))
}

pub fn list_boards_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all boards")
        .response::<200, Json<Vec<BoardResponse>>>()
}

pub fn get_board_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a specific board by ID")
        .response::<200, Json<BoardResponse>>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Board not found"))
}

pub fn update_board_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update a board")
        .response::<200, Json<BoardResponse>>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Board not found"))
}

pub fn delete_board_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a board")
        .response::<204, ()>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Board not found"))
}
