use tuono_lib::axum::body::Body;
use tuono_lib::axum::http::Request;
use tuono_lib::axum::http::StatusCode;

#[tuono_lib::api(GET)]
pub async fn health_check(_req: Request<Body>) -> StatusCode {
    StatusCode::OK
}
