mod entity;
mod service;
mod setup;

use axum::{
    extract::{Path, Query, State},
    routing::{get, patch},
    Json, Router,
};
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde_json::Value;
use std::collections::HashMap;

async fn get_craftsmen(
    State(mut state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> String {
    let postalcode = params.get("postalcode").expect("postalcode is required");

    "ok".to_string()
}

struct _PatchCraftsmanRequest {
    max_driving_distance: Option<i32>,
    profile_picture_score: Option<i32>,
    profile_description_score: Option<i32>,
}

async fn patch_craftsman(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<Value>,
) {
    todo!("patching craftsmen {}, {}", user_id, payload);
}

#[derive(Clone)]
struct AppState {
    connection_manager: ConnectionManager,
}

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://redis.server.orb.local:6379").unwrap();
    let connection_manager = ConnectionManager::new(client).await.unwrap();
    setup::setup_redis(connection_manager.to_owned()).await;

    let state = AppState { connection_manager };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/craftsmen", get(get_craftsmen))
        .route("/craftsman/:id", patch(patch_craftsman))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
