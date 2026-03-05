use aide::{
    axum::{
        routing::get_with,
        ApiRouter,
    },
};
use super::handlers::*;
use crate::state::AppState;

pub fn health_routes(state:AppState) -> ApiRouter {
  ApiRouter::new()
    .api_route("/", get_with(health_handler, health_handler_docs))
    .with_state(state)
}