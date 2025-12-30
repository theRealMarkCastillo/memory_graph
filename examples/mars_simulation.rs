use memory_graph::models::Memory;
use memory_graph::storage::StorageManager;
use memory_graph::index::{VectorIndex, vector::SimpleVectorIndex};
use memory_graph::query::{Query, Search, VectorSearch, Traverse, engine::QueryEngine};
use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;

fn main() -> Result<()> {
    println!("--- Mars Colony AI Simulation ---");

    // 1. Initialize Engine
    let storage = StorageManager::new("mars_colony.db")?;
    let mut vector_index = SimpleVectorIndex::new();

    // 2. Load Data
    println!("Loading dataset from data/mars_colony.json...");
    let file = File::open("data/mars_colony.json")?;
    let reader = BufReader::new(file);
    let memories: Vec<Memory> = serde_json::from_reader(reader)?;

    // 3. Ingest Data
    for mem in &memories {
        storage.save_memory(mem)?;
        vector_index.add(mem.id, &mem.embedding)?;
    }
    println!("Ingested {} memories.", memories.len());

    // 4. Create Edges (Simulating Knowledge Graph Construction)
    // Edge 1: "Sensor anomaly" (0) -> "Sector 7 houses fusion conduit" (1)
    // Relation: "located_at" / "involves"
    let id_anomaly = memories[0].id;
    let id_sector_info = memories[1].id;
    storage.add_edge_inherent(id_anomaly, id_sector_info, "related_context".to_string(), 0.9)?;

    // Edge 2: "Sector 7 houses fusion conduit" (1) -> "Fusion conduits emit thermal spikes..." (2)
    // Relation: "explained_by"
    let id_physics = memories[2].id;
    storage.add_edge_inherent(id_sector_info, id_physics, "physical_principle".to_string(), 0.85)?;

    // Edge 3: "Fusion conduits emit thermal spikes..." (2) -> "Commander Lewis authorized coolant flush" (3)
    // Relation: "caused_by_action" (Coolant flush -> Low pressure -> Thermal spikes)
    let id_action = memories[3].id;
    storage.add_edge_inherent(id_physics, id_action, "potential_cause".to_string(), 0.7)?;

    println!("Knowledge Graph constructed.");

    // 5. Run Scenarios

    // Scenario A: "Why is there a thermal spike?"
    // The agent sees "thermal spike" (vector match) and needs to find the root cause (graph traversal).
    println!("\n--- Scenario A: Root Cause Analysis ---");
    println!("Query: 'thermal spike' + 2-hop traversal");
    
    let query_a = Query {
        filter: None,
        search: Some(Search {
            vector: VectorSearch {
                text: Some("thermal spike".to_string()),
                embedding: Some(vec![0.8, 0.1, 0.1]), // Matches anomaly
                threshold: None,
            }
        }),
        traverse: Some(Traverse {
            direction: "outbound".to_string(),
            edge_types: None,
            depth: Some(2), // Deep traversal to find the Commander's action
        }),
        rank_by: None,
        limit: Some(10),
    };

    let engine = QueryEngine::new(&storage, &vector_index);
    let results_a = engine.execute(query_a)?;

    for (i, mem) in results_a.iter().enumerate() {
        println!("{}. [{}] {} (Score: High)", i+1, mem.id, mem.content);
    }

    // Scenario B: "What did Commander Lewis do?"
    // Pure vector search might find the action, but we want to see if it connects to the anomaly.
    println!("\n--- Scenario B: Contextual Recall ---");
    println!("Query: 'Commander Lewis' + Inbound Traversal (What did this affect?)");

    let query_b = Query {
        filter: None,
        search: Some(Search {
            vector: VectorSearch {
                text: Some("Commander Lewis".to_string()),
                embedding: Some(vec![0.1, 0.1, 0.9]), // Matches action
                threshold: None,
            }
        }),
        traverse: Some(Traverse {
            direction: "inbound".to_string(), // Look backwards: What points to this?
            edge_types: None,
            depth: Some(2),
        }),
        rank_by: None,
        limit: Some(10),
    };

    let results_b = engine.execute(query_b)?;
    for (i, mem) in results_b.iter().enumerate() {
        println!("{}. [{}] {}", i+1, mem.id, mem.content);
    }

    Ok(())
}
