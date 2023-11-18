use redis::geo::{RadiusOptions, Unit};
use redis::AsyncCommands;
use crate::{AppState, entity};
use crate::entity::Craftsman;

pub async fn get_craftsmen_by_postalcode(state: &mut AppState, postalcode: &String) -> Vec<Craftsman> {
    let radius = 105.0;
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

    let postal: String = state
        .connection_manager
        .get(format!("postal:{}", postalcode))
        .await
        .unwrap();

    let postal: entity::Postal = serde_json::from_str(&postal).unwrap();

    let close_craftsmen_ids: Vec<String> = close_craftsmen_ids
        .into_iter()
        .filter(|s| s.contains("profile:"))
        .collect();

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
    craftsmen
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
