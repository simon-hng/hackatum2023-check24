use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Craftsman {
    pub id: i32,
    pub name: String, // firstname + lastname
    pub ranking_score: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostcodeExtensionDistanceGroup {
    #[serde(rename = "group_a")]
    GroupA,
    #[serde(rename = "group_b")]
    GroupB,
    #[serde(rename = "group_c")]
    GroupC,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Postal {
    pub postcode: String,
    pub lon: f32,
    pub lat: f32,
    pub postcode_extension_distance_group: PostcodeExtensionDistanceGroup,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QualityFactors {
    pub profile_id: i32,
    pub profile_picture_score: f32,
    pub profile_description_score: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceProviderProfiles {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub street: String,
    pub house_number: String,
    pub lon: f32,
    pub lat: f32,
    pub max_driving_distance: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    #[serde(flatten)]
    pub quality_factors: QualityFactors,
    #[serde(flatten)]
    pub service_provider_profile: ServiceProviderProfiles,
}
