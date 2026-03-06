use super::handlers::*;
use crate::state::AppState;
use aide::axum::{ApiRouter, routing::get_with};

pub fn health_routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get_with(health_handler, health_handler_docs))
        .with_state(state)
}
