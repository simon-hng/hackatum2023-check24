use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PostcodeExtensionDistanceGroup {
    #[serde(rename = "group_a")]
    GroupA,
    #[serde(rename = "group_b")]
    GroupB,
    #[serde(rename = "group_c")]
    GroupC,
}

impl PostcodeExtensionDistanceGroup {
    pub fn get_extension_in_km(self) -> f64 {
        match self {
            PostcodeExtensionDistanceGroup::GroupA => { 0.0 }
            PostcodeExtensionDistanceGroup::GroupB => { 2.0 }
            PostcodeExtensionDistanceGroup::GroupC => { 5.0 }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Postal {
    pub postcode: String,
    pub lon: f64,
    pub lat: f64,
    pub postcode_extension_distance_group: PostcodeExtensionDistanceGroup,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualityFactors {
    pub profile_id: i32,
    pub profile_picture_score: f64,
    pub profile_description_score: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceProviderProfiles {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub street: String,
    pub house_number: String,
    pub lon: f64,
    pub lat: f64,
    pub max_driving_distance: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Craftsman {
    #[serde(flatten)]
    pub quality_factors: QualityFactors,
    #[serde(flatten)]
    pub service_provider_profile: ServiceProviderProfiles,
    pub rank: Option<f64>,
    pub distance: Option<f64>,
}
