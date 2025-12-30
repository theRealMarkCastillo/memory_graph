use crate::models::Memory;
use anyhow::Result;
use redb::{Database, ReadableTable, TableDefinition};
use std::path::Path;
use uuid::Uuid;

const MEMORY_TABLE: TableDefinition<u128, Vec<u8>> = TableDefinition::new("memories");

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
}
