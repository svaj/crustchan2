use axum_oidc_client::{
    auth::AuthenticationLayer,
    jwt::{Algorithm, DecodingKey, JwtConfigurationBuilder, JwtLayer, OidcClaims},
};
use std::env;
use tower::{
    ServiceBuilder,
    layer::util::{Identity, Stack},
};
use tower_http::trace::TraceLayer;

#[tuono_lib::middleware]
pub fn root_trace_layer()
-> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http()
}

// #[tuono_lib::middleware]
// pub fn root_cors_layer() -> CorsLayer {
//   CorsLayer::permissive()
// }

use axum_oidc_client::{
    auth::{AuthLayer, CodeChallengeMethod, OAuthConfiguration},
    auth_builder::OAuthConfigurationBuilder,
    auth_cache::AuthCache,
    logout::handle_default_logout::DefaultLogoutHandler,
    sql_cache::{SqlAuthCache, SqlCacheConfig},
};
use std::sync::Arc;

pub fn get_config() -> OAuthConfiguration {
    let auth0_client_id =
        env::var("AUTH0_OAUTH_CLIENT_ID").expect("AUTH0_OAUTH_CLIENT_ID is not set in .env file");
    let auth0_client_secret = env::var("AUTH0_OAUTH_CLIENT_SECRET")
        .expect("AUTH0_OAUTH_CLIENT_SECRET is not set in .env file");
    let auth0_endpoint =
        env::var("AUTH0_OAUTH_ENDPOINT").expect("AUTH0_OAUTH_ENDPOINT is not set in .env file");
    let private_cookie_key =
        env::var("PRIVATE_COOKIE_KEY").expect("PRIVATE_COOKIE_KEY is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    // let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("https://{host}");
    let config = OAuthConfigurationBuilder::default()
        .with_authorization_endpoint(&format!("{auth0_endpoint}authorize"))
        .with_token_endpoint(&format!("{auth0_endpoint}oauth/token"))
        .with_client_id(&auth0_client_id)
        .with_client_secret(&auth0_client_secret)
        .with_redirect_uri(&format!("{server_url}auth/callback"))
        .with_post_logout_redirect_uri(&server_url)
        .with_private_cookie_key(&private_cookie_key)
        .with_scopes(vec!["openid", "email", "profile", "post"])
        .with_code_challenge_method(CodeChallengeMethod::S256)
        .with_session_max_age(30) // 30 minutes
        .with_base_path("/auth") // Optional: customize auth routes (default: "/auth")
        .build();
    println!("Got Auth MW Config!!");
    config.expect("invalid config")
}

pub fn get_cache() -> Arc<dyn AuthCache + Send + Sync> {
    let data = axum_oidc_client::cache::TwoTierAuthCache::new(
        None,
        axum_oidc_client::cache::config::TwoTierCacheConfig::default(),
    )
    .expect("bad data");

    let cache: Arc<dyn AuthCache + Send + Sync> = Arc::new(data);
    cache
}

#[tuono_lib::middleware]
pub fn auth_layer_middleware()
-> ServiceBuilder<Stack<JwtLayer<OidcClaims>, Stack<AuthenticationLayer, Identity>>> {
    dotenvy::dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file");
    let server_url = format!("{host}:{port}");
    let config = get_config();

    let jwt_config = JwtConfigurationBuilder::<OidcClaims>::new()
        .with_decoding_key(DecodingKey::from_secret(jwt_secret.as_bytes()))
        .with_algorithm(Algorithm::HS256)
        .with_audience(vec!["my-client-id".to_string()])
        .build()
        .expect("invalid jwt config");
    /*

    // TODO: IF  dotenv env var uses postgres or redis, use those for session caching. https://github.com/RedSoftwareSystems/axum-oidc-client?tab=readme-ov-file#core-modules
    let sqlite_config = SqlCacheConfig {
        connection_string: "sqlite://session_cache.db".to_string(),
        ..Default::default()
    };

    let sqlite_cache = Arc::new(SqlAuthCache::new(sqlite_config).await.expect("No SQLite cache available"));
    sqlite_cache.init_schema().await.expect("SQLite cache schema failed to generate.");
    */

    // Create cache — L1-only in-memory cache using Moka (requires `moka-cache` feature, enabled by default).
    let cache = get_cache();

    // Create logout handler
    let logout_handler = Arc::new(DefaultLogoutHandler);
    println!("Running at http://{server_url}/");
    let layer = AuthLayer::new(Arc::new(config), cache, logout_handler);
    let svc = ServiceBuilder::new()
        .layer(layer)
        .layer(JwtLayer::new(Arc::new(jwt_config)));

    svc
}
