use std::fs;

use redis::{aio::ConnectionManager, AsyncCommands, RedisError};

use crate::entity;

async fn setup_postal(mut connection_manager: ConnectionManager) {
    let postals = serde_json::from_str::<Vec<entity::Postal>>(
        &fs::read_to_string("data/postcode.json").unwrap(),
    )
    .unwrap();

    for postal in postals.iter() {
        let key = format!("postal:{}", postal.postcode);
        let _: Result<String, RedisError> = connection_manager
            .set(key.to_owned(), serde_json::to_string(postal).unwrap())
            .await;

        let _ = connection_manager
            .geo_add::<String, (f32, f32, String), String>(
                "locations".to_string(),
                (postal.lon, postal.lat, key),
            )
            .await;
    }
}

async fn setup_service_providers(mut connection_manager: ConnectionManager) {
    let quality_factors = serde_json::from_str::<Vec<entity::QualityFactors>>(
        &fs::read_to_string("data/quality_factor_score.json").unwrap(),
    )
    .unwrap();

    let service_provider_profiles = serde_json::from_str::<Vec<entity::ServiceProviderProfiles>>(
        &fs::read_to_string("data/service_provider_profile.json").unwrap(),
    )
    .unwrap();

    if quality_factors.len() != service_provider_profiles.len() {
        panic!("quality_factors_strings and service_provider_profiles have different lengths");
    }

    let profiles: Vec<entity::Craftsman> = service_provider_profiles
        .into_iter()
        .zip(quality_factors.into_iter())
        .map(
            |(service_provider_profile, quality_factors)| entity::Craftsman {
                quality_factors,
                service_provider_profile,
                rank: None,
                distance: None,
            },
        )
        .collect();

    for profile in profiles.iter() {
        let key = format!("profile:{}", profile.service_provider_profile.id);
        let _: Result<String, RedisError> = connection_manager
            .set(key.to_owned(), serde_json::to_string(profile).unwrap())
            .await;

        let _ = connection_manager
            .geo_add::<String, (f32, f32, String), String>(
                "locations".to_string(),
                (
                    profile.service_provider_profile.lon,
                    profile.service_provider_profile.lat,
                    key,
                ),
            )
            .await;
    }
}

pub async fn setup_redis(connection_manager: ConnectionManager) {
    log::info!("Setting up redis db");
    setup_postal(connection_manager.to_owned()).await;

    log::info!("Added postals to redis");
    setup_service_providers(connection_manager.to_owned()).await;

    log::info!("Added service_providers to redis");
}
