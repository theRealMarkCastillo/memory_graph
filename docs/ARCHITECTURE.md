# MemoryGraph Architecture

## 1. System Overview

MemoryGraph is a single-node (initially), embedded or server-based memory engine written in Rust. It is designed for read-heavy, write-moderate workloads typical of AI agents (10:1 read:write ratio).

### Core Components

```mermaid
graph TD
    Client[Client (Python/Rust)] --> API[gRPC / HTTP API]
    API --> QueryPlanner[Query Planner]
    QueryPlanner --> Executor[Query Executor]
    Executor --> VectorIndex[HNSW Index]
    Executor --> GraphIndex[Adjacency Index]
    Executor --> Storage[redb KV Store]
    Executor --> Metadata[Inverted Index]
```

## 2. Data Model

The fundamental unit is a `Memory`. Unlike traditional graph DBs where nodes and edges are distinct first-class citizens, MemoryGraph uses a **Memory-Centric** model where edges are properties of the Memory.

### Memory Structure (Rust Struct)

```rust
struct Memory {
    /// Unique identifier (UUID v7 for time-sortability)
    id: Uuid,
    
    /// The raw text content or payload
    content: String,
    
    /// 384D - 1536D vector embedding
    embedding: Vec<f32>,
    
    /// Cognitive classification (key differentiator from other DBs)
    memory_type: MemoryType,
    
    /// Arbitrary JSON metadata (user_id, source, etc.)
    metadata: HashMap<String, Value>,
    
    /// Outgoing edges (relationships)
    edges: Vec<Edge>,
    
    /// System timestamps
    created_at: DateTime<Utc>,
    last_accessed_at: DateTime<Utc>,
    
    /// Cognitive metrics (built-in, not application-managed)
    access_count: u64,
    importance: f32,        // 0.0 - 1.0, updated by access patterns
    decay_rate: f32,        // How fast this memory fades
}

/// Cognitive Memory Types - First-class schema support
enum MemoryType {
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

struct Edge {
    target_id: Uuid,
    relation_type: String, // e.g., "relates_to", "authored_by"
    weight: f32,
    created_at: DateTime<Utc>,
}
```

### Why Cognitive Schema Matters

**Neo4j/FalkorDB approach:** Store any node with any properties. Memory types are application concerns.

**MemoryGraph approach:** Memory types are **first-class**. The query planner can optimize differently for episodic vs semantic retrieval. Research metrics can track type distributions over time.

This is opinionated by design — MemoryGraph is for **cognitive agents**, not general-purpose graphs.

## 3. Storage Engine

### 3.1 Persistence Layer
**Decision: Use an Embedded Key-Value Store.**
Instead of writing a custom WAL and SSTable implementation (which risks data durability bugs), MemoryGraph acts as a logic layer on top of a battle-tested embedded KV store.

*   **Primary Backend:** `redb` (Pure Rust, ACID, fast) or `rocksdb` (via bindings).
*   **Data Layout:**
    *   `memories:{uuid}` -> `bincode(MemoryStruct)`
    *   `edges:out:{uuid}` -> `[Edge, ...]`
    *   `edges:in:{uuid}` -> `[SourceUuid, ...]` (Reverse Index)

### 3.2 Indexing Strategy

MemoryGraph maintains four co-located indexes:

1.  **Vector Index (HNSW):**
    *   **Phase 1:** Standard HNSW implementation (via `hnsw_rs` or `lance`).
    *   **Phase 2:** Graph-Aware HNSW where neighbor selection prefers existing graph edges.

2.  **Graph Index (Adjacency List):**
    *   **Forward Index:** `HashMap<Uuid, Vec<Edge>>` for `(A) -> (B)` traversal.
    *   **Reverse Index:** `HashMap<Uuid, Vec<Uuid>>` for `(A) <- (B)` traversal (incoming edges).
    *   *Crucial for:* "What triggered this memory?" queries.

3.  **Metadata Index (Inverted Index):**
    *   `HashMap<Field, RoaringBitmap>` for fast filtering.
    *   Allows queries like `WHERE user_id = "user_123"`.

## 4. Query Execution Pipeline

The "Secret Sauce" of MemoryGraph is the **Hybrid Query Planner** — what differentiates it from Neo4j's bolt-on vector indexes.

### The Problem with Existing Solutions

**Neo4j (2024 vector indexes):**
```cypher
// Step 1: Vector search (returns node IDs)
CALL db.index.vector.queryNodes('embeddings', 10, $query_vec)
YIELD node, score
// Step 2: Graph expansion (separate operation)
MATCH (node)-[r]->(neighbor)
RETURN node, neighbor
```
The planner doesn't know vectors and graph are related. It can't optimize across them.

**MemoryGraph:**
The planner sees the full query and chooses the optimal execution path:
- **Vector-first:** When semantic similarity dominates ("find memories about coffee")
- **Graph-first:** When structure dominates ("find memories 2 hops from user_123")
- **Interleaved:** When both matter ("find similar memories within my conversation graph")

### Execution Example

**Query:** "Find relevant memories about 'coffee' from 'user_123' and their related concepts."

**MemoryGraph Execution Plan:**
1.  **Filter Step:** Use Metadata Index to get candidate set `S` (user="123").
2.  **Vector Step:** Perform HNSW search *restricted* to set `S` (using bitmasking). Get top `K` nodes.
3.  **Traversal Step:** Immediately expand edges from top `K` nodes within the same engine memory space.
4.  **Cognitive Ranking:** Score results using:
    ```
    Score = (α * VectorSimilarity) 
          + (β * GraphCentrality) 
          + (γ * Recency) 
          + (δ * Importance)
    ```
    Note: `Recency` and `Importance` are **built-in cognitive metrics**, not application-computed.

## 5. Concurrency & Consistency

*   **Concurrency:** Rust's `tokio` for async I/O. `RwLock` for index protection.
*   **Consistency:** `redb` provides ACID durability. In-memory indexes (HNSW, Graph) are rebuilt on startup or updated asynchronously.
*   **Isolation:** Snapshot isolation for queries (readers don't block writers).

## 6. Scalability

*   **Phase 1 (Single Node):** Capable of handling ~5M memories (sufficient for most single-agent deployments).
*   **Phase 2 (Sharding):** Sharding by `AgentID` or `UserID`. Cross-shard edges are "weak references" (require 2-step lookup).

## 7. Rust Crate Dependencies (Proposed)

*   `tokio`: Async runtime.
*   `serde`: Serialization.
*   `redb`: Embedded ACID key-value store (Pure Rust).
*   `hnsw_rs`: HNSW vector indexing.
*   `tonic`: gRPC definition.
*   `roaring`: Bitmaps for filtering.
