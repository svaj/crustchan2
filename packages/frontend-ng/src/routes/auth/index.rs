use tuono_lib::{Props, Response};

#[tuono_lib::handler] // session: AuthSession
async fn auth_index() -> Response {
    println!("custom auth_index handler.");
    // let expires = session.expires
    // .map(|e| e.to_string())
    // .unwrap_or_else(|| "(no expiry)".to_string());
    let msg = format!("Hello, INDEX user! Token expires: {}", "expires");

    Response::Props(Props::new(msg))
}
