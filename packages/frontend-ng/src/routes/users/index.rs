use tuono_lib::{Props, Response};

use axum_oidc_client::auth_session::AuthSession;
#[tuono_lib::handler] // session: AuthSession
async fn user_index(session: AuthSession) -> Response {
    dbg!(&session);
    println!("ASDASDA");
    let expires = session
        .expires
        .map(|e| e.to_string())
        .unwrap_or_else(|| "(no expiry)".to_string());
    let msg = format!("Hello, authenticated user! Token expires: {}", expires);
    println!("{msg}");
    Response::Props(Props::new(msg))
}
