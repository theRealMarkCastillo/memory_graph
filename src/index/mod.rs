use anyhow::Result;
use uuid::Uuid;

pub mod vector;

pub trait VectorIndex {
    fn add(&mut self, id: Uuid, vector: &[f32]) -> Result<()>;
    fn search(&self, vector: &[f32], k: usize) -> Result<Vec<(Uuid, f32)>>;
}

pub trait GraphIndex {
    fn add_edge(&mut self, source: Uuid, target: Uuid, relation_type: String, weight: f32) -> Result<()>;
    fn get_neighbors(&self, node: Uuid) -> Result<Vec<(Uuid, f32)>>;
}
