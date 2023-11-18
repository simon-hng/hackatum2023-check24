use crate::entity;

pub fn calculate_rank(
    quality_factors: &entity::QualityFactors,
    distance: Option<f32>,
) -> Option<f32> {
    if distance.is_none() {
        return None;
    };

    let distance = distance.unwrap();

    let profile_score = 0.4 * quality_factors.profile_picture_score as f32
        + 0.6 * quality_factors.profile_description_score as f32;

    let default_distance = 80.0;
    let distance_score = 1.0 - (distance / default_distance);
    let distance_weight = if distance > default_distance {
        0.01
    } else {
        0.15
    };

    Some(distance_weight * distance_score + (1.0 - distance_weight) * profile_score)
}
