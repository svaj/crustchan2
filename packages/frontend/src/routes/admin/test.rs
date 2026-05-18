use tuono_lib::{Props, Response};

#[tuono_lib::handler] // session: AuthSession
async fn admin_test() -> Response {
    println!("ADMIN TEST OK");
    Response::Props(Props::new("fsdfsdf"))
}
