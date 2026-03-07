use aide::transform::TransformOperation;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use entity::report;
use schemars::JsonSchema;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateReportRequest {
    pub post_id: Option<Uuid>,
    pub file_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_identifier_id: Option<Uuid>,
    pub reason: String,
    pub reporter_user_id: Option<Uuid>,
    pub reporter_user_identifier_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct UpdateReportRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReportResponse {
    pub id: Uuid,
    pub post_id: Option<Uuid>,
    pub file_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_identifier_id: Option<Uuid>,
    pub reason: String,
    pub reporter_user_id: Option<Uuid>,
    pub reporter_user_identifier_id: Option<Uuid>,
}

impl From<report::Model> for ReportResponse {
    fn from(model: report::Model) -> Self {
        Self {
            id: model.id,
            post_id: model.post_id,
            file_id: model.file_id,
            user_id: model.user_id,
            user_identifier_id: model.user_identifier_id,
            reason: model.reason,
            reporter_user_id: model.reporter_user_id,
            reporter_user_identifier_id: model.reporter_user_identifier_id,
        }
    }
}

// TODO: Auto ban on report configurable thresholds
pub async fn create_report(
    State(state): State<AppState>,
    Json(req): Json<CreateReportRequest>,
) -> (StatusCode, Json<ReportResponse>) {
    let report_model = report::ActiveModel {
        id: sea_orm::Set(Uuid::now_v7()),
        post_id: sea_orm::Set(req.post_id),
        file_id: sea_orm::Set(req.file_id),
        user_id: sea_orm::Set(req.user_id),
        user_identifier_id: sea_orm::Set(req.user_identifier_id),
        reason: sea_orm::Set(req.reason),
        reporter_user_id: sea_orm::Set(req.reporter_user_id),
        reporter_user_identifier_id: sea_orm::Set(req.reporter_user_identifier_id),
        ..Default::default()
    };

    match report_model.insert(&state.db_conn).await {
        Ok(result) => (StatusCode::CREATED, Json(result.into())),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ReportResponse {
                id: Uuid::nil(),
                post_id: None,
                file_id: None,
                user_id: None,
                user_identifier_id: None,
                reason: "Failed to create report".to_string(),
                reporter_user_id: None,
                reporter_user_identifier_id: None,
            }),
        ),
    }
}

pub async fn list_reports(State(state): State<AppState>) -> Json<Vec<ReportResponse>> {
    match report::Entity::find().all(&state.db_conn).await {
        Ok(reports) => {
            let responses = reports.into_iter().map(|r| r.into()).collect();
            Json(responses)
        }
        Err(_) => Json(vec![]),
    }
}

pub async fn get_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<Option<ReportResponse>>) {
    match report::Entity::find_by_id(id).one(&state.db_conn).await {
        Ok(Some(report)) => (StatusCode::OK, Json(Some(report.into()))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

// TODO: Check perms
pub async fn update_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateReportRequest>,
) -> (StatusCode, Json<Option<ReportResponse>>) {
    let report_result = report::Entity::find_by_id(id).one(&state.db_conn).await;

    match report_result {
        Ok(Some(report)) => {
            let mut report_active: report::ActiveModel = report.into_active_model();

            if let Some(reason) = req.reason {
                report_active.reason = sea_orm::Set(reason);
            }

            match report_active.update(&state.db_conn).await {
                Ok(updated) => (StatusCode::OK, Json(Some(updated.into()))),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

// TODO: Check perms
pub async fn delete_report(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let report_result = report::Entity::find_by_id(id).one(&state.db_conn).await;

    match report_result {
        Ok(Some(report)) => {
            if let Ok(_) = report.delete(&state.db_conn).await {
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

pub fn create_report_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new report")
        .response::<201, Json<ReportResponse>>()
        .response_with::<400, Json<AppError>, _>(|res| res.description("Invalid request"))
}

pub fn list_reports_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all reports")
        .response::<200, Json<Vec<ReportResponse>>>()
}

pub fn get_report_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a specific report by ID")
        .response::<200, Json<ReportResponse>>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Report not found"))
}

pub fn update_report_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update a report")
        .response::<200, Json<ReportResponse>>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Report not found"))
}

pub fn delete_report_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a report")
        .response::<204, ()>()
        .response_with::<404, Json<AppError>, _>(|res| res.description("Report not found"))
}
