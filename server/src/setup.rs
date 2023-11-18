use std::fs;

use redis::{aio::ConnectionManager, AsyncCommands};

use crate::entity;

async fn setup_postal(mut connection_manager: ConnectionManager) {
    let postals = serde_json::from_str::<Vec<entity::Postal>>(
        &fs::read_to_string("data/postcode.json").unwrap(),
    )
    .unwrap();

    for postal in postals.iter() {
        let postalcode = postal.postcode.to_owned();
        let _ = connection_manager
            .set::<String, String, String>(
                format!("postal:{}", postalcode),
                serde_json::to_string(postal).unwrap(),
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

    let profiles: Vec<entity::Profile> = service_provider_profiles
        .into_iter()
        .zip(quality_factors.into_iter())
        .map(
            |(service_provider_profile, quality_factors)| entity::Profile {
                quality_factors,
                service_provider_profile,
            },
        )
        .collect();

    for profile in profiles.iter() {
        let profile_id = profile.service_provider_profile.id;
        let _ = connection_manager
            .set::<String, String, String>(
                format!("profile:{}", profile_id),
                serde_json::to_string(profile).unwrap(),
            )
            .await;
    }

    todo!()
}

pub async fn setup_redis(connection_manager: ConnectionManager) {
    setup_postal(connection_manager.to_owned()).await;
    setup_service_providers(connection_manager.to_owned()).await;
}
