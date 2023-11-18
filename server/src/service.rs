use crate::entity;

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
