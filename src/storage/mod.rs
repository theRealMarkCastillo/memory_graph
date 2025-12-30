use crate::models::{Memory, Edge, InboundEdge};
use anyhow::Result;
use redb::{Database, ReadableTable, TableDefinition};
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;

const MEMORY_TABLE: TableDefinition<u128, Vec<u8>> = TableDefinition::new("memories");
const EDGES_OUT: TableDefinition<u128, Vec<u8>> = TableDefinition::new("edges_out");
const EDGES_IN: TableDefinition<u128, Vec<u8>> = TableDefinition::new("edges_in");

pub struct StorageManager {
    db: Database,
}

impl StorageManager {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db = Database::create(path)?;
        Ok(Self { db })
    }

    pub fn save_memory(&self, memory: &Memory) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(MEMORY_TABLE)?;
            let key = memory.id.as_u128();
            let value = serde_json::to_vec(memory)?;
            table.insert(key, value)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn get_memory(&self, id: Uuid) -> Result<Option<Memory>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(MEMORY_TABLE)?;
        let key = id.as_u128();
        
        if let Some(value) = table.get(key)? {
            let memory: Memory = serde_json::from_slice(&value.value())?;
            Ok(Some(memory))
        } else {
            Ok(None)
        }
    }

    pub fn list_memories(&self) -> Result<Vec<Memory>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(MEMORY_TABLE)?;
        let mut memories = Vec::new();
        
        for result in table.iter()? {
            let (_, value) = result?;
            let memory: Memory = serde_json::from_slice(&value.value())?;
            memories.push(memory);
        }
        
        Ok(memories)
    }

    // --- Graph Operations ---

    pub fn add_edge(&self, source: Uuid, target: Uuid, relation_type: String, weight: f32) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        let now = Utc::now();

        // 1. Update Outbound Index (Source -> Target)
        {
            let mut table = write_txn.open_table(EDGES_OUT)?;
            let key = source.as_u128();
            
            let mut edges: Vec<Edge> = if let Some(value) = table.get(key)? {
                serde_json::from_slice(&value.value())?
            } else {
                Vec::new()
            };

            edges.push(Edge {
                target_id: target,
                relation_type: relation_type.clone(),
                weight,
                created_at: now,
            });

            table.insert(key, serde_json::to_vec(&edges)?)?;
        }

        // 2. Update Inbound Index (Target -> Source)
        {
            let mut table = write_txn.open_table(EDGES_IN)?;
            let key = target.as_u128();
            
            let mut edges: Vec<InboundEdge> = if let Some(value) = table.get(key)? {
                serde_json::from_slice(&value.value())?
            } else {
                Vec::new()
            };

            edges.push(InboundEdge {
                source_id: source,
                relation_type,
                weight,
                created_at: now,
            });

            table.insert(key, serde_json::to_vec(&edges)?)?;
        }

        write_txn.commit()?;
        Ok(())
    }

    pub fn get_outbound_edges(&self, id: Uuid) -> Result<Vec<Edge>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(EDGES_OUT)?;
        let key = id.as_u128();
        
        if let Some(value) = table.get(key)? {
            let edges: Vec<Edge> = serde_json::from_slice(&value.value())?;
            Ok(edges)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn get_inbound_edges(&self, id: Uuid) -> Result<Vec<InboundEdge>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(EDGES_IN)?;
        let key = id.as_u128();
        
        if let Some(value) = table.get(key)? {
            let edges: Vec<InboundEdge> = serde_json::from_slice(&value.value())?;
            Ok(edges)
        } else {
            Ok(Vec::new())
        }
    }
}
