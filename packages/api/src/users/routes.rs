use super::handlers::*;
use crate::state::AppState;
use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with, put_with},
};
use axum::routing::delete;

pub fn user_routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post_with(create_user, create_user_docs))
        .api_route("/login", post_with(login_user, login_user_docs))
        .api_route("/", get_with(list_users, list_users_docs))
        .api_route("/{id}", get_with(get_user, get_user_docs))
        .api_route("/{id}", put_with(update_user, update_user_docs))
        .route("/{id}", delete(delete_user))
        .with_state(state)
}
