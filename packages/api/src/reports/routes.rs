use super::handlers::*;
use crate::state::AppState;
use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with, put_with},
};
use axum::routing::delete;

pub fn report_routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post_with(create_report, create_report_docs))
        .api_route("/", get_with(list_reports, list_reports_docs))
        .api_route("/{id}", get_with(get_report, get_report_docs))
        .api_route("/{id}", put_with(update_report, update_report_docs))
        .route("/{id}", delete(delete_report))
        .with_state(state)
}
