use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier (UUID v7 for time-sortability)
    pub id: Uuid,
    
    /// The raw text content or payload
    pub content: String,
    
    /// 384D - 1536D vector embedding
    pub embedding: Vec<f32>,
    
    /// Cognitive classification
    pub memory_type: MemoryType,
    
    /// Arbitrary JSON metadata (user_id, source, etc.)
    pub metadata: HashMap<String, Value>,
    
    /// Outgoing edges (relationships)
    pub edges: Vec<Edge>,
    
    /// System timestamps
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    
    /// Cognitive metrics
    pub access_count: u64,
    pub importance: f32,        // 0.0 - 1.0
    pub decay_rate: f32,        // How fast this memory fades
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MemoryType {
    /// Event memories ("I talked to Mark about coffee")
    Episodic { 
        event_id: Option<Uuid>, 
        participants: Vec<String>,
        location: Option<String>,
    },
    /// Fact memories ("Mark likes dark roast")
    Semantic { 
        confidence: f32,      // 0.0 - 1.0
        source: String,       // Where this fact came from
    },
    /// Skill/procedure memories ("How to make pour-over coffee")
    Procedural { 
        success_rate: f32,    // Historical success
        last_executed: Option<DateTime<Utc>>,
    },
    /// Emotional valence memories
    Emotional {
        valence: f32,         // -1.0 (negative) to 1.0 (positive)
        arousal: f32,         // 0.0 (calm) to 1.0 (excited)
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub target_id: Uuid,
    pub relation_type: String, // e.g., "relates_to", "authored_by"
    pub weight: f32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundEdge {
    pub source_id: Uuid,
    pub relation_type: String,
    pub weight: f32,
    pub created_at: DateTime<Utc>,
}

impl Memory {
    pub fn new(content: String, embedding: Vec<f32>, memory_type: MemoryType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            content,
            embedding,
            memory_type,
            metadata: HashMap::new(),
            edges: Vec::new(),
            created_at: now,
            last_accessed_at: now,
            access_count: 0,
            importance: 1.0, // Default importance
            decay_rate: 0.1, // Default decay
        }
    }
}
