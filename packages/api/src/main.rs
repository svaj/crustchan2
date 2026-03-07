use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::{Extension, Json, http::StatusCode};
// use crustchan_migration::{Migrator, MigratorTrait};
use rustis::client::Client;
use sea_orm::Database;
use std::env;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

pub mod boards;
pub mod docs;
pub mod errors;
pub mod health;
pub mod reports;
pub mod state;

use boards::routes::board_routes;
use docs::docs_routes;
use errors::AppError;
use health::routes::health_routes;
use reports::routes::report_routes;
use state::AppState;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
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
        .register(entity::user_permissions::Entity)
        .register(entity::user_profile::Entity)
        .register(entity::user::Entity)
        .sync(db_conn)
        .await?;

    // Migrator::up(&db_conn, None).await.unwrap();

    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST is not set in .env file");
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT is not set in .env file");
    let redis_url_full = format!("redis://{redis_host}:{redis_port}");
    let cache_conn = Arc::new(Client::connect(redis_url_full).await.unwrap());

    let state = AppState {
        db_conn: db_conn.to_owned(),
        cache_conn,
    };
    let mut api = OpenApi::default();

    let app = ApiRouter::new()
        .nest_api_service("/status", health_routes(state.clone()))
        .nest_api_service("/boards", board_routes(state.clone()))
        .nest_api_service("/reports", report_routes(state.clone()))
        .nest_api_service("/docs", docs_routes(state.clone()))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api))) // Arc is very important here or you will face massive memory and performance issues
        // .route("/api/graphql", get(graphql_playground))
        // .route("/api/graphql", post(graphql_handler))
        // .nest_service(
        //     "/static",
        //     get_service(ServeDir::new(concat!(
        //         env!("CARGO_MANIFEST_DIR"),
        //         "/static"
        //     )))
        //     .handle_error(|error| async move {
        //         (
        //             StatusCode::INTERNAL_SERVER_ERROR,
        //             format!("Unhandled internal error: {error}"),
        //         )
        //     }),
        // )
        // .nest_service(
        //     "/admin",
        //     get_service(
        //         ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/admin")).fallback(
        //             ServeFile::new(concat!(
        //                 env!("CARGO_MANIFEST_DIR"),
        //                 "/../assets/admin/index.html"
        //             ) ),
        //         ),
        //     ),
        // )
        .layer(CookieManagerLayer::new())
        .with_state(state);

    println!("Example docs are accessible at http://{server_url}/docs");
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Crustchan")
        .summary("An imageboard for the modern web")
        .description(include_str!("../../../README.md"))
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<Json<AppError>, _>(|res| {
            res.example(AppError {
                error: "some error happened".to_string(),
                error_details: None,
                error_id: Uuid::nil(),
                // This is not visible.
                status: StatusCode::IM_A_TEAPOT,
            })
        })
}
