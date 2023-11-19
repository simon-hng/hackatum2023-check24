use redis::geo::{RadiusOptions, Unit};
use redis::{AsyncCommands, RedisError};
use redis::aio::ConnectionManager;
use crate::{AppState, entity};
use crate::entity::{Craftsman, PostcodeExtensionDistanceGroup};

async fn get_postal(state: &mut AppState, postalcode: &String) -> entity::Postal {
    let postal: String = state
        .connection_manager
        .get(format!("postal:{}", postalcode))
        .await
        .unwrap();
    let postal: entity::Postal = serde_json::from_str(&postal).unwrap();
    postal
}

async fn get_required_craftsmen(state: &mut AppState, postalcode: &String, required_craftsmen: i32, extension_group: PostcodeExtensionDistanceGroup) -> Vec<String> {
    let mut radius = match extension_group {
        PostcodeExtensionDistanceGroup::GroupA => { 10.0 }
        PostcodeExtensionDistanceGroup::GroupB => { 20.0 }
        PostcodeExtensionDistanceGroup::GroupC => { 40.0 }
    };

    let mut close_craftsmen_ids: Vec<String> = vec![];

    while close_craftsmen_ids.len() < required_craftsmen as usize {
        close_craftsmen_ids = state
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

        radius *= 2.0;
    }

    // postals are in the same bucket to caculate the distance with redis
    close_craftsmen_ids
        .into_iter()
        .filter(|s| s.contains("profile:"))
        .collect()
}

// Gets the craftsmen by postalcode, sorted by rank.
// Always returns 20 craftsmen.
pub async fn get_craftsmen_by_postalcode(state: &mut AppState, postalcode: &String, page: i32) -> Vec<entity::Craftsman> {
    let required_craftsmen = page * 20;

    let postal = get_postal(state, postalcode).await;
    let close_craftsmen_ids = get_required_craftsmen(state, postalcode, required_craftsmen, postal.postcode_extension_distance_group).await;

    let mut craftsmen: Vec<entity::Craftsman> = vec![];
    for id in close_craftsmen_ids.iter() {
        let craftsman_string: String = state.connection_manager.get(id).await.unwrap();
        let mut craftsman: entity::Craftsman = serde_json::from_str(&craftsman_string).unwrap();

        let distance: Option<f64> = state
            .connection_manager
            .geo_dist(
                "locations",
                format!("postal:{}", postalcode),
                format!("profile:{}", craftsman.service_provider_profile.id),
                Unit::Kilometers,
            )
            .await
            .ok();

        craftsman.distance = distance;

        // Filter out craftsmen who are too far away
        if let Some(distance) = distance {
            let post_extension = postal.postcode_extension_distance_group.get_extension_in_km().to_owned();
            let max_driving_distance = (craftsman.service_provider_profile.max_driving_distance / 1000.0) +
                post_extension;

            if distance >= max_driving_distance {
                continue;
            }
        }

        craftsman.rank = calculate_rank(&craftsman.quality_factors, distance);
        craftsmen.push(craftsman);
    }

    craftsmen.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    craftsmen.iter().skip((page - 1) as usize * 20).take(20).cloned().collect()
}

pub fn calculate_rank(
    quality_factors: &entity::QualityFactors,
    distance: Option<f64>,
) -> Option<f64> {
    if distance.is_none() {
        return None;
    };

    let distance = distance.unwrap();

    let profile_score = 0.4 * quality_factors.profile_picture_score
        + 0.6 * quality_factors.profile_description_score;

    let default_distance = 80.0;
    let distance_score = 1.0 - (distance / default_distance);
    let distance_weight = if distance > default_distance {
        0.01
    } else {
        0.15
    };

    Some(distance_weight * distance_score + (1.0 - distance_weight) * profile_score)
}

#[cfg(test)]
mod tests {
    use crate::entity;
    use crate::service::calculate_rank;

    #[test]
    fn zero_score() {
        let actual = calculate_rank(
            &entity::QualityFactors {
                profile_id: 1,
                profile_picture_score: 0.0,
                profile_description_score: 0.0,
            },
            Some(80.0),
        ).unwrap();
        assert_eq!(actual, 0.0);
    }

    #[test]
    fn all_params_zero() {
        let actual = calculate_rank(
            &entity::QualityFactors {
                profile_id: 1,
                profile_picture_score: 0.0,
                profile_description_score: 0.0,
            },
            Some(0.0),
        ).unwrap();
        assert_eq!(actual, 0.15);
    }

    #[test]
    fn best_score() {
        let actual = calculate_rank(
            &entity::QualityFactors {
                profile_id: 1,
                profile_picture_score: 3.0,
                profile_description_score: 3.0,
            },
            Some(0.0),
        ).unwrap();
        assert_eq!(actual, 2.6999999999999997);
    }

    #[test]
    fn best_craftsman_default_distance() {
        let actual = calculate_rank(
            &entity::QualityFactors {
                profile_id: 1,
                profile_picture_score: 3.0,
                profile_description_score: 3.0,
            },
            Some(80.0),
        ).unwrap();
        assert_eq!(actual, 2.55);
    }

    #[test]
    fn best_craftsman_twice_default_distance() {
        let actual = calculate_rank(
            &entity::QualityFactors {
                profile_id: 1,
                profile_picture_score: 3.0,
                profile_description_score: 3.0,
            },
            Some(160.0),
        ).unwrap();
        assert_eq!(actual, 2.96);
    }
}

pub async fn upsert_profile(connection_manager: &mut ConnectionManager, profile: &Craftsman) {
    let key = format!("profile:{}", profile.service_provider_profile.id);
    let _: Result<String, RedisError> = connection_manager
        .set(key.to_owned(), serde_json::to_string(profile).unwrap())
        .await;

    let _ = connection_manager
        .geo_add::<String, (f64, f64, String), String>(
            "locations".to_string(),
            (
                profile.service_provider_profile.lon,
                profile.service_provider_profile.lat,
                key,
            ),
        )
        .await;
}
