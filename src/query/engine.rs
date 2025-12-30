use crate::index::VectorIndex;
use crate::models::Memory;
use crate::query::{Query, Filter};
use crate::storage::StorageManager;
use anyhow::Result;
use uuid::Uuid;
use std::collections::{HashSet, HashMap};

pub struct QueryEngine<'a, V: VectorIndex> {
    storage: &'a StorageManager,
    vector_index: &'a V,
}

impl<'a, V: VectorIndex> QueryEngine<'a, V> {
    pub fn new(storage: &'a StorageManager, vector_index: &'a V) -> Self {
        Self {
            storage,
            vector_index,
        }
    }

    pub fn execute(&self, query: Query) -> Result<Vec<Memory>> {
        let mut candidates: HashSet<Uuid> = HashSet::new();
        let mut scores: HashMap<Uuid, f32> = HashMap::new();

        // Step 1: Vector Search (Primary Driver)
        let mut vector_search_performed = false;
        if let Some(search) = &query.search {
             if let Some(embedding) = &search.vector.embedding {
                 let limit = query.limit.unwrap_or(10);
                 let results = self.vector_index.search(embedding, limit)?;
                 for (id, score) in results {
                     candidates.insert(id);
                     scores.insert(id, score);
                 }
                 vector_search_performed = true;
             }
        }

        // Fallback: If no vector search, load all (Naive for v0)
        if !vector_search_performed {
            let all_memories = self.storage.list_memories()?;
            for mem in all_memories {
                candidates.insert(mem.id);
                scores.insert(mem.id, 1.0);
            }
        }

        // Step 2: Filter (Post-filtering)
        if let Some(filter) = &query.filter {
            let mut filtered_candidates = HashSet::new();
            for id in &candidates {
                if let Some(mem) = self.storage.get_memory(*id)? {
                    if self.matches_filter(&mem, filter) {
                        filtered_candidates.insert(*id);
                    }
                }
            }
            candidates = filtered_candidates;
        }

        // Step 3: Traversal (Graph Expansion)
        if let Some(traverse) = &query.traverse {
            let mut expanded_candidates = candidates.clone();
            for id in &candidates {
                // Default to outbound for now if not specified or "outbound"
                if traverse.direction == "outbound" || traverse.direction == "both" {
                    let edges = self.storage.get_outbound_edges(*id)?;
                    for edge in edges {
                        // TODO: Check edge types
                        expanded_candidates.insert(edge.target_id);
                        // Decay score for hops (simple heuristic)
                        let parent_score = *scores.get(id).unwrap_or(&1.0);
                        scores.entry(edge.target_id).or_insert(parent_score * 0.5); 
                    }
                }
                
                if traverse.direction == "inbound" || traverse.direction == "both" {
                     let edges = self.storage.get_inbound_edges(*id)?;
                     for edge in edges {
                        expanded_candidates.insert(edge.source_id);
                        let parent_score = *scores.get(id).unwrap_or(&1.0);
                        scores.entry(edge.source_id).or_insert(parent_score * 0.5);
                     }
                }
            }
            candidates = expanded_candidates;
        }

        // Step 4: Fetch, Sort and Return
        let mut result_memories = Vec::new();
        for id in candidates {
            if let Some(mem) = self.storage.get_memory(id)? {
                result_memories.push(mem);
            }
        }
        
        // Sort by score descending
        result_memories.sort_by(|a, b| {
            let score_a = scores.get(&a.id).unwrap_or(&0.0);
            let score_b = scores.get(&b.id).unwrap_or(&0.0);
            score_b.partial_cmp(score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply limit
        if let Some(limit) = query.limit {
            if result_memories.len() > limit {
                result_memories.truncate(limit);
            }
        }

        Ok(result_memories)
    }

    fn matches_filter(&self, memory: &Memory, filter: &Filter) -> bool {
        // 1. Check ID
        if let Some(id_val) = filter.criteria.get("id") {
            if let Some(id_str) = id_val.as_str() {
                if let Ok(filter_id) = Uuid::parse_str(id_str) {
                    if memory.id != filter_id {
                        return false;
                    }
                }
            }
        }
        
        // 2. Check Memory Type (simple string match on the enum variant name if possible, 
        // or just check if the JSON representation contains the type)
        // For now, let's assume the user passes "memory_type": "Episodic" in criteria
        if let Some(type_val) = filter.criteria.get("memory_type") {
             if let Some(type_str) = type_val.as_str() {
                 let mem_json = serde_json::to_value(&memory.memory_type).unwrap();
                 // mem_json is {"type": "Episodic", "data": {...}}
                 if let Some(actual_type) = mem_json.get("type") {
                     if actual_type.as_str() != Some(type_str) {
                         return false;
                     }
                 }
             }
        }

        true
    }
}
