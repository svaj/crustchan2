use rustis::client::Client;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

// Shared application state, database connection and redis client
#[derive(Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
    pub cache_conn: Arc<Client>,
}
