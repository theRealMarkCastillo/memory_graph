# Query DSL (JSON)

For v1, MemoryGraph uses a structured JSON Query DSL instead of a custom SQL parser. This ensures type safety, easier parsing, and faster implementation.

## 1. The Query Object

Every query is a JSON object with optional phases: `filter`, `search`, `traverse`, and `rank_by`.

```json
{
  "filter": { ... },    // Step 1: Narrow the search space (Metadata)
  "search": { ... },    // Step 2: Rank by vector similarity
  "traverse": { ... },  // Step 3: Expand graph neighborhood
  "rank_by": { ... },   // Step 4: Custom scoring formula
  "limit": 20
}
```

## 2. Vector Search

Find memories semantically similar to a query string or raw embedding.

```json
// Option A: Text (auto-embedded by server)
{
  "search": {
    "vector": {
      "text": "I love drinking coffee in the morning",
      "threshold": 0.8
    }
  },
  "limit": 5
}

// Option B: Raw embedding (client provides vector)
{
  "search": {
    "vector": {
      "embedding": [0.1, 0.2, ...],
      "threshold": 0.8
    }
  },
  "limit": 5
}
```

## 3. Graph Traversal

Find memories connected to a specific node.

```json
{
  "filter": {
    "id": "memory_123"
  },
  "traverse": {
    "direction": "outbound",  // "inbound", "outbound", "both"
    "edge_types": ["relates_to"],
    "depth": 1
  }
}
```

## 4. Hybrid Queries (The Power Move)

### Example 1: "Context Expansion"
*Find memories about 'coding', then include everything they are directly connected to.*

```json
{
  "filter": {
    "user_id": "mark"
  },
  "search": {
    "vector": {
      "text": "coding in rust"
    }
  },
  "traverse": {
    "direction": "both",
    "edge_types": ["relates_to", "mentioned_in"],
    "depth": 1
  },
  "limit": 20
}
```

### Example 2: "Graph-Constrained Vector Search"
*Find memories that are semantically similar to 'danger', but ONLY if they are within 2 hops of the 'current_situation' node.*

```json
{
  "filter": {
    "graph_reachability": {
      "start_node": "current_situation_node",
      "max_depth": 2
    }
  },
  "search": {
    "vector": {
      "text": "danger"
    }
  },
  "limit": 5
}
```

## 5. Custom Ranking

Override the default scoring with a weighted formula.

```json
{
  "search": {
    "vector": { "text": "childhood home" }
  },
  "rank_by": {
    "weights": {
      "vector_similarity": 0.7,
      "importance": 0.2,
      "recency": 0.1
    }
  },
  "limit": 10
}
```

**Built-in scoring factors:**
- `vector_similarity` — Cosine similarity to query vector
- `importance` — Memory's importance score (0-1)
- `recency` — Decay function based on `last_accessed_at`
- `access_count` — How often this memory has been retrieved

## 6. Data Manipulation (DML)

DML operations are performed via standard REST/gRPC endpoints, not the Query DSL.

*   `POST /memories` - Create memory
*   `POST /edges` - Create edge
*   `PATCH /memories/{id}` - Update metadata
