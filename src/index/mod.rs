use anyhow::Result;
use uuid::Uuid;

pub trait VectorIndex {
    fn add(&mut self, id: Uuid, vector: &[f32]) -> Result<()>;
    fn search(&self, vector: &[f32], k: usize) -> Result<Vec<(Uuid, f32)>>;
}

pub trait GraphIndex {
    fn add_edge(&mut self, source: Uuid, target: Uuid, weight: f32) -> Result<()>;
    fn get_neighbors(&self, node: Uuid) -> Result<Vec<(Uuid, f32)>>;
}

// Placeholder implementations
pub struct SimpleVectorIndex {
    // In a real implementation, this would be an HNSW structure
    vectors: Vec<(Uuid, Vec<f32>)>,
}

impl SimpleVectorIndex {
    pub fn new() -> Self {
        Self { vectors: Vec::new() }
    }
}

impl VectorIndex for SimpleVectorIndex {
    fn add(&mut self, id: Uuid, vector: &[f32]) -> Result<()> {
        self.vectors.push((id, vector.to_vec()));
        Ok(())
    }

    fn search(&self, query_vector: &[f32], k: usize) -> Result<Vec<(Uuid, f32)>> {
        // Brute force cosine similarity for now
        let mut scores: Vec<(Uuid, f32)> = self.vectors.iter()
            .map(|(id, vec)| {
                let score = cosine_similarity(query_vector, vec);
                (*id, score)
            })
            .collect();
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(k);
        Ok(scores)
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}
