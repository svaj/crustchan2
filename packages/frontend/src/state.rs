use axum_gate::{
    gate::oauth2::OAuth2Gate, groups::Group, repositories::sea_orm::SeaOrmRepository, roles::Role,
};
use rustis::client::Client;
use sea_orm::DatabaseConnection;
use std::{fmt, sync::Arc};
use tuono_lib::axum::Router;

// Shared application state, database connection and redis client
#[derive(Clone, Default)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub cache_conn: Option<Arc<Client>>,
    pub account_repository: Option<Arc<SeaOrmRepository>>,
    pub secrets_repository: Option<Arc<SeaOrmRepository>>,
    pub oauth2_gate: OAuth2Gate<Role, Group>,
    pub routes: Router,
    pub router: Router,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "routes: {:?}", self.routes)
    }
}
