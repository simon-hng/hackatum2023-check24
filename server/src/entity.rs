use serde::Deserialize;

#[derive(Deserialize)]
pub struct Craftsman {
    pub id: i32,
    pub name: String, // firstname + lastname
    pub ranking_score: f64,
}
