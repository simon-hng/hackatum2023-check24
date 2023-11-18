mod entity;

use axum::{
    extract::{Path, Query, State},
    routing::{get, patch},
    Json, Router,
};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Commands};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
struct AppState {
    connection_manager: ConnectionManager,
}

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://redis.server.orb.local:6379").unwrap();
    let connection_manager = ConnectionManager::new(client).await.unwrap();

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

struct Profile {
    profile_id: i32,
    profile_picture_score: f32,
    profile_description_score: f32,
}

fn calculate_rank(profile: Profile) -> f32 {
    let profile_score =
        0.4 * profile.profile_picture_score + 0.6 * profile.profile_description_score;

    // TODO: Get distance from redis
    let distance = 0.0;

    let default_distance = 80.0;
    let distance_score = 1.0 - (distance / default_distance);
    let distance_weight = if distance > default_distance {
        0.01
    } else {
        0.15
    };

    distance_weight * distance_score + (1.0 - distance_weight) * profile_score
}

async fn get_craftsmen(
    State(mut state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> String {
    let postalcode = params.get("postalcode").expect("postalcode is required");

    let postal: HashMap<String, String> = state
        .connection_manager
        .hgetall(format!("postal:{}", postalcode))
        .await
        .unwrap();

    println!(
        "{:?}, {:?}",
        postal.get("lat").unwrap(),
        postal.get("lon").unwrap()
    );

    todo!(
        "Get craftsmen in the area of {}, {}",
        postal.get("lat").unwrap(),
        postal.get("lon").unwrap()
    );
    // todo!("Rank craftsmen by distance, profile picture score, profile description score, and review score");

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
