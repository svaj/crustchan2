use serde::Serialize;
use tuono_lib::{Props, Response, Type};

#[derive(Serialize, Type)]
struct AdminIndexData {
    title: &'static str,
}

#[tuono_lib::handler]
async fn admin_index() -> Response {
    Response::Props(Props::new(AdminIndexData {
        title: "Crustchan Admin",
    }))
}
