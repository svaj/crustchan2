use super::handlers::*;
use crate::state::AppState;
use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with, put_with},
};
use axum::routing::delete;

pub fn ban_routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post_with(create_ban, create_ban_docs))
        .api_route("/", get_with(list_bans, list_bans_docs))
        .api_route("/{id}", get_with(get_ban, get_ban_docs))
        .api_route("/{id}", put_with(update_ban, update_ban_docs))
        .route("/{id}", delete(delete_ban))
        .with_state(state)
}
