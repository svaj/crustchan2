use serde::Serialize;
use tuono_lib::{Props, Response, Type};

#[derive(Serialize, Type)]
struct MyResponse2<'a> {
    subtitle: &'a str,
}

#[tuono_lib::handler]
async fn root_index() -> Response {
    Response::Props(Props::new(MyResponse2 {
        subtitle: "root index",
    }))
}
