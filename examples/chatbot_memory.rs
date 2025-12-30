use memory_graph::models::{Memory, MemoryType};
use memory_graph::storage::StorageManager;
use memory_graph::index::{VectorIndex, vector::SimpleVectorIndex};
use memory_graph::query::{Query, Search, VectorSearch, Traverse, engine::QueryEngine};
use anyhow::Result;
use std::collections::HashMap;
use serde_json::json;

fn main() -> Result<()> {
    println!("--- AI Chatbot Memory Example ---");

    // 1. Initialize Engine
    // In a real app, you'd persist this to disk.
    let storage = StorageManager::new("chatbot_memory.db")?;
    let mut vector_index = SimpleVectorIndex::new();

    // 2. Simulate a Conversation
    println!("Simulating conversation...");

    // Turn 1: User introduces themselves
    // "Hi, I'm Alice. I love hiking."
    // Embedding: [0.1, 0.8, 0.2] (Simulated "hiking/outdoors" vector)
    let user_msg_1 = Memory::new(
        "User: Hi, I'm Alice. I love hiking.".to_string(),
        vec![0.1, 0.8, 0.2], 
        MemoryType::Episodic {
            event_id: None,
            participants: vec!["Alice".to_string(), "Bot".to_string()],
            location: None,
        }
    );
    storage.save_memory(&user_msg_1)?;
    vector_index.add(user_msg_1.id, &user_msg_1.embedding)?;

    // Turn 2: Bot extracts a fact
    // Fact: "Alice likes hiking"
    // Embedding: [0.1, 0.9, 0.1] (Stronger "hiking" vector)
    let mut fact_hiking = Memory::new(
        "Alice likes hiking".to_string(),
        vec![0.1, 0.9, 0.1],
        MemoryType::Semantic {
            confidence: 0.95,
            source: "user_conversation".to_string(),
        }
    );
    // Add metadata
    let mut meta = HashMap::new();
    meta.insert("topic".to_string(), json!("hobbies"));
    fact_hiking.metadata = meta;

    storage.save_memory(&fact_hiking)?;
    vector_index.add(fact_hiking.id, &fact_hiking.embedding)?;

    // Link the fact to the conversation that generated it
    storage.add_edge_inherent(
        fact_hiking.id, 
        user_msg_1.id, 
        "derived_from".to_string(), 
        1.0
    )?;
    println!("Stored conversation and extracted fact.");

    // Turn 3: User asks for a recommendation later
    // "What should I do this weekend?"
    // Embedding: [0.2, 0.7, 0.3] (Simulated "activity/weekend" vector, close to hiking)
    println!("\n--- New User Query ---");
    println!("User: 'What should I do this weekend?'");
    
    // 3. Bot performs a Memory Search
    // The bot searches for memories related to the user's query to generate a personalized response.
    let query = Query {
        filter: None, // Could filter by user_id here
        search: Some(Search {
            vector: VectorSearch {
                text: Some("weekend activity recommendation".to_string()),
                embedding: Some(vec![0.2, 0.7, 0.3]), 
                threshold: Some(0.8), // High threshold for relevance
            }
        }),
        // We also want to traverse to see *why* we know this (the source conversation)
        traverse: Some(Traverse {
            direction: "outbound".to_string(), // Look for what this fact is connected to
            edge_types: Some(vec!["derived_from".to_string()]),
            depth: Some(1),
        }),
        rank_by: None,
        limit: Some(5),
    };

    let engine = QueryEngine::new(&storage, &vector_index);
    let results = engine.execute(query)?;

    println!("\n--- Retrieved Context ---");
    if results.is_empty() {
        println!("No relevant memories found.");
    } else {
        for (i, mem) in results.iter().enumerate() {
            match &mem.memory_type {
                MemoryType::Semantic { confidence, .. } => {
                    println!("{}. [FACT] {} (Confidence: {})", i+1, mem.content, confidence);
                },
                MemoryType::Episodic { participants, .. } => {
                    println!("{}. [EVENT] {} (Participants: {:?})", i+1, mem.content, participants);
                },
                _ => println!("{}. {}", i+1, mem.content),
            }
        }
        
        println!("\nBot Response Generation:");
        println!("Based on the fact '{}', I suggest you go for a hike!", results[0].content);
    }

    Ok(())
}
