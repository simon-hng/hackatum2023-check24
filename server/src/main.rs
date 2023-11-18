mod entity;
mod service;
mod setup;

use axum::{
    extract::{Path, Query, State},
    routing::{get, patch},
    Json, Router,
};
use redis::{aio::ConnectionManager, geo::Unit};
use redis::{geo::RadiusOptions, AsyncCommands};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

async fn get_craftsmen(
    State(mut state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> String {
    let postalcode = params.get("postalcode").expect("postalcode is required");

    let radius = 10.0;
    let close_craftsmen_ids: Vec<String> = state
        .connection_manager
        .geo_radius_by_member(
            "locations".to_string(),
            format!("postal:{}", postalcode),
            radius,
            Unit::Kilometers,
            RadiusOptions::default(),
        )
        .await
        .unwrap();

    let mut craftsmen: Vec<entity::Craftsman> = vec![];
    for id in close_craftsmen_ids.iter() {
        let craftsman_string: String = state.connection_manager.get(id).await.unwrap();
        let mut craftsman: entity::Craftsman = serde_json::from_str(&craftsman_string).unwrap();
        let distance: Option<f32> = state
            .connection_manager
            .geo_dist(
                "locations",
                format!("postal:{}", postalcode),
                format!("postal:{}", postalcode),
                Unit::Kilometers,
            )
            .await
            .ok();

        craftsman.distance = distance;
        craftsman.rank = service::calculate_rank(&craftsman.quality_factors, distance);
        craftsmen.push(craftsman);
    }

    craftsmen.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    let sorted_and_taken: Vec<entity::Craftsman> = craftsmen.into_iter().take(20).collect();

    serde_json::to_string(&sorted_and_taken).unwrap()
}

#[derive(Deserialize)]
struct PatchCraftsmanRequest {
    max_driving_distance: Option<i32>,
    profile_picture_score: Option<f32>,
    profile_description_score: Option<f32>,
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
