use memory_graph::models::{Memory, MemoryType};
use memory_graph::storage::StorageManager;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Initializing MemoryGraph...");

    // Initialize storage
    let storage = StorageManager::new("memory_graph.db")?;

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

    println!("Saving memories...");
    storage.save_memory(&memory1)?;
    storage.save_memory(&memory2)?;

    // Create an edge between them
    println!("Creating edge: {} -> {}", memory1.id, memory2.id);
    storage.add_edge_inherent(memory1.id, memory2.id, "relates_to".to_string(), 0.8)?;

    // Verify outbound edges
    let outbound = storage.get_outbound_edges(memory1.id)?;
    println!("Outbound edges for {}: {:?}", memory1.id, outbound);
    assert_eq!(outbound.len(), 1);
    assert_eq!(outbound[0].target_id, memory2.id);

    // Verify inbound edges
    let inbound = storage.get_inbound_edges(memory2.id)?;
    println!("Inbound edges for {}: {:?}", memory2.id, inbound);
    assert_eq!(inbound.len(), 1);
    assert_eq!(inbound[0].source_id, memory1.id);

    println!("Graph operations verified successfully!");

    Ok(())
}
