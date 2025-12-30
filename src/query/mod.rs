use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod engine;

#[derive(Debug, Deserialize, Serialize)]
pub struct Query {
    pub filter: Option<Filter>,
    pub search: Option<Search>,
    pub traverse: Option<Traverse>,
    pub rank_by: Option<RankBy>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Filter {
    #[serde(flatten)]
    pub criteria: Value, // Flexible for now
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Search {
    pub vector: VectorSearch,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VectorSearch {
    pub text: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub threshold: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Traverse {
    pub direction: String, // "inbound", "outbound", "both"
    pub edge_types: Option<Vec<String>>,
    pub depth: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RankBy {
    // Placeholder for custom ranking logic
    pub formula: String,
}
