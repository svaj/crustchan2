use tuono_lib::{Props, Response};
use tuono_lib::axum::extract::Extension;

use axum_gate::{accounts::Account, groups::Group, roles::Role};
#[tuono_lib::handler] // session: AuthSession
async fn user_index(Extension(account): Extension<Account<Role,Group>>) -> Response {
    dbg!(&account);
    println!("ASDASDA");
    let msg = format!("Hello, authenticated user! Token expires: {}", account.account_id);
    println!("{msg}");
    Response::Props(Props::new(msg))
}
