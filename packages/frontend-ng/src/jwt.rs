use std::env;
use std::sync::Arc;

use axum_gate::{
    gate::Gate,
    groups::Group,
    jsonwebtoken,
    prelude::{Account, JsonWebToken, JsonWebTokenOptions, JwtClaims},
    repositories::sea_orm::{SeaOrmRepository, models},
    roles::Role,
};

pub fn get_jwt() -> Arc<JsonWebToken<JwtClaims<Account<Role, Group>>>> {
    dotenvy::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not st in .env file");
    // Create a JWT codec. Use a persistent key in production (e.g., env/secret manager).
    let options = JsonWebTokenOptions {
        enc_key: jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
        dec_key: jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_bytes()),
        header: Some(axum_gate::jsonwebtoken::Header::default()),
        validation: Some(axum_gate::jsonwebtoken::Validation::default()),
    };
    Arc::new(JsonWebToken::<JwtClaims<Account<Role, Group>>>::new_with_options(options))
}
