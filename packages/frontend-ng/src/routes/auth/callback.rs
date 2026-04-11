use axum_oidc_client::auth_session::AuthSession;
use tuono_lib::{Props, Response};

#[tuono_lib::handler] // session: AuthSession
async fn auth_callback(session: AuthSession) -> Response {
    println!("custom auth_callback handler.");
    dbg!(&session);
    println!("woo..");
    let expires = session
        .expires
        .map(|e| e.to_string())
        .unwrap_or_else(|| "(no expiry)".to_string());
    let msg = format!("Hello, NERD user! Token expires: {}", expires);
    println!("{msg}");
    Response::Props(Props::new(msg))
}
