use aide::transform::TransformOperation;
use axum::response::Json;
use serde_json::{Value, json};

pub async fn health_handler() -> Json<Value> {
    Json(json!({"status": "OK"}))
}

pub fn health_handler_docs(op: TransformOperation) -> TransformOperation {
    op.description("Respond if healthy.")
        .response::<200, Json<Value>>()
}
