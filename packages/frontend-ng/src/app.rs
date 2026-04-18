
pub mod state;
pub mod jwt;
pub mod routes{
  pub mod middlewares;
}

pub mod app {
  use std::{env, sync::LazyLock};
use futures::executor::block_on;
use rustis::client::Client;
use sea_orm::Database;
use tuono_lib::axum::Router;
use std::sync::Arc;
use axum_gate::{gate::Gate, groups::Group, prelude::Account, repositories::sea_orm::{SeaOrmRepository, models}, roles::Role};

use super::state::AppState;
use super::jwt::get_jwt;

pub async fn gen_state() -> AppState {
    dotenvy::dotenv().ok();

    let _subscriber = LazyLock::new(|| {     tracing_subscriber::fmt()
      .with_target(false)
      .compact()
      .init()
    });


    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    dotenvy::dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
        let auth0_client_id =
        env::var("AUTH0_OAUTH_CLIENT_ID").expect("AUTH0_OAUTH_CLIENT_ID is not set in .env file");
    let auth0_client_secret = env::var("AUTH0_OAUTH_CLIENT_SECRET")
        .expect("AUTH0_OAUTH_CLIENT_SECRET is not set in .env file");
    let auth0_endpoint =
        env::var("AUTH0_OAUTH_ENDPOINT").expect("AUTH0_OAUTH_ENDPOINT is not set in .env file");

    let server_url = format!("{host}:{port}");
    let db_conn = &Database::connect(db_url)
        .await
        .expect("Database connection failed");

    // sync Entity schemas
    db_conn
        .get_schema_builder()
        .register(entity::ban::Entity)
        .register(entity::category::Entity)
        .register(entity::board::Entity)
        .register(entity::file::Entity)
        .register(entity::post::Entity)
        .register(entity::report::Entity)
        .register(entity::thread::Entity)
        .register(entity::user_identifier::Entity)
        .register(entity::user_permission::Entity)
        .register(entity::user_profile::Entity)
        .register(entity::user::Entity)
        .register(models::account::Entity)
        .register(models::credentials::Entity)
        .register(models::permission_mapping::Entity)
        .sync(db_conn)
        .await.expect("Error connecting to DB");

    // Set up storage (dev-friendly in-memory backends)

    let jwt = get_jwt();

    let account_repository = Arc::new(SeaOrmRepository::new(&db_conn).unwrap());
    dbg!("Account repository initialized.");
    let secrets_repository = Arc::clone(&account_repository);
    dbg!("Secrets repository initialized.");

    let ttl_secs = 60 * 60 * 24;
    let oauth2_gate = Gate::oauth2_with_jwt("Crustchan", jwt.clone(), 3600)
    .auth_url(format!("{auth0_endpoint}authorize"))
    .token_url(format!("{auth0_endpoint}oauth/token"))
    .client_id(auth0_client_id)
    .client_secret(auth0_client_secret)
    .redirect_url(format!("{server_url}auth/callback"))
    .add_scope("openid")
    .add_scope("email")
    .with_jwt_codec("Crustchan", Arc::clone(&jwt), ttl_secs)
    
    .with_account_repository(account_repository.clone())
    // Map provider token response to your Account<Role, Group>:
    .with_account_mapper(|token_resp| {
        Box::pin(async move {
          dbg!(token_resp);
            println!("UH LOADING ACCOUNT");
            // fetch userinfo as needed, then construct Account<Role, Group>
            Ok(Account::<Role, Group>::new("user@example.com", &[], &[]))
        })
    });

    let Ok(routes) = oauth2_gate.routes("/auth") else {
        panic!("No oauth routes");
    };






        let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST is not set in .env file");
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT is not set in .env file");
    let redis_url_full = format!("redis://{redis_host}:{redis_port}");
    let cache_conn = Arc::new(Client::connect(redis_url_full).await.unwrap());

    let router =Router::new().merge(routes.clone()).clone();
    let state: AppState = AppState {
        db_conn: db_conn.to_owned(),
        cache_conn:Some(cache_conn),
        account_repository:Some(account_repository),
        secrets_repository:Some(secrets_repository),
        oauth2_gate:oauth2_gate,
        routes,
        router,
    };
    
    state
}

pub static  STATE: LazyLock< AppState> = LazyLock::new(|| { block_on( async{gen_state().await})});

pub async fn main() -> AppState {
  return STATE.clone()
}

pub async fn get_app_state() -> AppState {
  return STATE.clone();
}


}
pub use app::main;