mod create_task;
mod custom_json_extractor;
mod get_one_task;
mod hello_world;
mod validate_with_serde;

use axum::{
    body::Body,
    routing::{get, post},
    Extension, Router,
};
use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use get_one_task::get_one_task;
use sea_orm::DatabaseConnection;
use validate_with_serde::validate_with_serde;

pub async fn create_routes(database: DatabaseConnection) -> Router<Body> {
    Router::new()
        .route("/hello_world", get(hello_world::hello_world))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks/:task_id", get(get_one_task))
        .layer(Extension(database))
}
