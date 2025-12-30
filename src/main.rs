use memory_graph::models::{Memory, MemoryType};
use memory_graph::storage::StorageManager;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Initializing MemoryGraph...");

    // Initialize storage
    let storage = StorageManager::new("memory_graph.db")?;

    // Create a sample memory
    let memory = Memory::new(
        "I love drinking coffee in the morning".to_string(),
        vec![0.1, 0.2, 0.3], // Dummy embedding
        MemoryType::Episodic {
            event_id: None,
            participants: vec!["Mark".to_string()],
            location: Some("Kitchen".to_string()),
        },
    );

    println!("Saving memory: {}", memory.id);
    storage.save_memory(&memory)?;

    // Retrieve the memory
    if let Some(retrieved) = storage.get_memory(memory.id)? {
        println!("Retrieved memory: {:?}", retrieved);
    } else {
        println!("Memory not found!");
    }

    Ok(())
}
