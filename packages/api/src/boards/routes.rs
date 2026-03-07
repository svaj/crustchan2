use super::handlers::*;
use crate::state::AppState;
use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with, put_with},
};
use axum::routing::delete;

pub fn board_routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post_with(create_board, create_board_docs))
        .api_route("/", get_with(list_boards, list_boards_docs))
        .api_route("/{id}", get_with(get_board, get_board_docs))
        .api_route("/{id}", put_with(update_board, update_board_docs))
        .route("/{id}", delete(delete_board))
        .with_state(state)
}
