mod entity;
mod service;
mod setup;

use axum::{
    extract::{Path, Query, State},
    Json,
    Router, routing::{get, patch},
};
use dotenvy::dotenv;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;

async fn get_craftsmen(
    State(mut state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> String {
    let postalcode = params.get("postalcode").expect("postalcode is required");
    let page = params.get("page").map(|s| s.parse::<i32>().unwrap()).unwrap_or(1);

    let craftsmen = service::get_craftsmen_by_postalcode(&mut state, postalcode, page).await;
    let sorted_and_taken: Vec<entity::Craftsman> = craftsmen.into_iter().take(20).collect();

    serde_json::to_string(&sorted_and_taken).unwrap()
}

#[derive(Deserialize)]
struct PatchCraftsmanRequest {
    max_driving_distance: Option<f64>,
    profile_picture_score: Option<f64>,
    profile_description_score: Option<f64>,
}

async fn patch_craftsman(
    State(mut state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<PatchCraftsmanRequest>,
) -> Json<Value> {
    let craftsman_string: String = state
        .connection_manager
        .get(format!("profile:{}", user_id))
        .await
        .unwrap();

    let mut craftsman: entity::Craftsman = serde_json::from_str(&craftsman_string).unwrap();

    if let Some(max_driving_distance) = payload.max_driving_distance {
        craftsman.service_provider_profile.max_driving_distance = max_driving_distance;
    }
    if let Some(profile_picture_score) = payload.profile_picture_score {
        craftsman.quality_factors.profile_picture_score = profile_picture_score;
    }
    if let Some(profile_description_score) = payload.profile_description_score {
        craftsman.quality_factors.profile_description_score = profile_description_score;
    }

    Json(json!({
        "id": user_id,
        "updated": {
            "maxDrivingDistance": payload.max_driving_distance,
            "profilePictureScore": payload.profile_picture_score,
            "profileDescriptionScore": payload.profile_description_score
        }
    }))
}

#[derive(Clone)]
pub struct AppState {
    pub connection_manager: ConnectionManager,
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenv().ok();

    log::info!("Starting up");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url).unwrap();
    let connection_manager = ConnectionManager::new(client).await.unwrap();
    setup::setup_redis(connection_manager.to_owned()).await;

    let state = AppState { connection_manager };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/craftsmen", get(get_craftsmen))
        .route("/craftsman/:id", patch(patch_craftsman))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
