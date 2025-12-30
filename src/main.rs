use memory_graph::models::{Memory, MemoryType};
use memory_graph::storage::StorageManager;
use memory_graph::index::{VectorIndex, vector::SimpleVectorIndex};
use memory_graph::query::{Query, Search, VectorSearch, Traverse, engine::QueryEngine};
use anyhow::Result;

fn main() -> Result<()> {
    println!("Initializing MemoryGraph...");

    // Initialize storage
    let storage = StorageManager::new("memory_graph.db")?;
    
    // Initialize vector index (in-memory for now)
    let mut vector_index = SimpleVectorIndex::new();

    // Create two sample memories
    let memory1 = Memory::new(
        "I love drinking coffee in the morning".to_string(),
        vec![0.1, 0.2, 0.3], 
        MemoryType::Episodic {
            event_id: None,
            participants: vec!["Mark".to_string()],
            location: Some("Kitchen".to_string()),
        },
    );

    let memory2 = Memory::new(
        "Coffee beans are best stored in airtight containers".to_string(),
        vec![0.15, 0.25, 0.35],
        MemoryType::Semantic {
            confidence: 0.95,
            source: "Barista Handbook".to_string(),
        },
    );
    
    let memory3 = Memory::new(
        "Rust is a systems programming language".to_string(),
        vec![0.9, 0.8, 0.7], // Far away vector
        MemoryType::Semantic {
            confidence: 1.0,
            source: "Rust Book".to_string(),
        },
    );

    println!("Saving memories...");
    storage.save_memory(&memory1)?;
    storage.save_memory(&memory2)?;
    storage.save_memory(&memory3)?;
    
    // Add to vector index
    vector_index.add(memory1.id, &memory1.embedding)?;
    vector_index.add(memory2.id, &memory2.embedding)?;
    vector_index.add(memory3.id, &memory3.embedding)?;

    // Create an edge between them
    println!("Creating edge: {} -> {}", memory1.id, memory2.id);
    storage.add_edge_inherent(memory1.id, memory2.id, "relates_to".to_string(), 0.8)?;

    // --- Query Demo ---
    println!("\n--- Executing Hybrid Query ---");
    
    // Query: Find memories similar to "coffee" (vec: [0.1, 0.2, 0.3]) 
    // AND traverse to find related memories.
    let query = Query {
        filter: None,
        search: Some(Search {
            vector: VectorSearch {
                text: None,
                embedding: Some(vec![0.1, 0.2, 0.3]), // Exact match for memory1
                threshold: None,
            }
        }),
        traverse: Some(Traverse {
            direction: "outbound".to_string(),
            edge_types: None,
            depth: Some(1),
        }),
        rank_by: None,
        limit: Some(5),
    };

    let engine = QueryEngine::new(&storage, &vector_index);
    let results = engine.execute(query)?;

    println!("Found {} results:", results.len());
    for mem in results {
        println!("- [{:?}] {} (ID: {})", mem.memory_type, mem.content, mem.id);
    }

    Ok(())
}
