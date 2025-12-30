# MemoryGraph

**The Memory Substrate for Cognitive AI Agents**

> ⚠️ **Status: Concept / Specification Phase**

**MemoryGraph is a research-grade memory engine for AI agents where semantic similarity and relational context are unified to study coherence, decay, and long-term recall.**

It goes beyond combining vectors and graphs — it provides an opinionated cognitive schema with built-in temporal decay, memory importance scoring, and emergence research instrumentation.

## 🚀 The Problem

**"Vector + Graph" databases already exist** (Neo4j vector indexes, FalkorDB, Weaviate cross-refs). So why MemoryGraph?

Existing solutions bolt vectors onto graphs (or vice versa). They are **storage-first**, not **cognition-first**:

| What Exists | What's Missing |
|-------------|----------------|
| Neo4j + Vector Index | Query planner is graph-first; vectors are second-class properties |
| FalkorDB | Built on Redis; vector is an extension, not unified storage |
| Weaviate | Cross-references ≠ true graph traversal; no Cypher-like queries |
| Microsoft GraphRAG | Orchestration layer, not a database; uses external stores |

**The real gap:**
*   No system treats vectors and edges as **co-equal citizens** in the query planner
*   No system has **cognitive primitives** built-in (temporal decay, importance, memory types)
*   No system provides **emergence research instrumentation** (coherence metrics, orphan detection)

## 💡 The Solution: Cognition-First Memory

MemoryGraph treats **Memories** as cognitive units — not just data records. Each memory has *content* (vector), *context* (edges), and *cognitive state* (importance, recency, access patterns).

### What Makes It Different

| Feature | Neo4j + Vectors | FalkorDB | **MemoryGraph** |
|---------|-----------------|----------|------------------|
| Storage Model | Graph + vector property | Graph + vector extension | **Unified (co-located)** |
| Query Planner | Graph-first | Graph-first | **Hybrid (optimizes across both)** |
| Temporal Decay | ❌ Manual | ❌ Manual | **✅ Built-in** |
| Memory Types | ❌ | ❌ | **✅ Episodic/Semantic/Procedural** |
| Coherence Metrics | ❌ | ❌ | **✅ Research instrumentation** |
| Target Use Case | General graphs | GraphRAG | **Cognitive agents** |

### Core Capabilities

*   **Single Rust Binary:** No JVM, no complex dependencies.
*   **True Hybrid Queries:** Query planner chooses vector-first OR graph-first based on cost.
*   **Cognitive Schema:** Built-in memory types, importance scores, temporal decay.
*   **Emergence Instrumentation:** Coherence metrics, orphan detection, cluster analysis.

## 🛠️ Technical Stack

*   **Language:** Rust (for performance, safety, and low latency)
*   **Storage Backend:** `redb` (embedded ACID key-value store)
*   **Vector Index:** HNSW (Hierarchical Navigable Small World)
*   **Graph Storage:** Bidirectional Adjacency Lists (forward + reverse indexes)
*   **Query Interface:** JSON Query DSL (no custom parser complexity)

## 🏁 Getting Started

### Prerequisites
*   Rust (latest stable) -> [Install Rust](https://rustup.rs/)

### Build and Run Demo
The current prototype includes a CLI demo that initializes the engine, creates sample memories (Episodic & Semantic), links them via a graph edge, and executes a **Hybrid Query** (Vector Search + Graph Traversal).

```bash
# Clone the repository
git clone https://github.com/markcastillo/memory_graph.git
cd memory_graph

# Run the demo
cargo run
```

**Expected Output:**
You should see the engine initialize, save memories, create an edge, and perform a query that finds a memory via vector similarity ("coffee") and then traverses the graph to find a related fact ("airtight containers").

## 📚 Documentation

*   [**Architecture**](docs/ARCHITECTURE.md) - Internal design, storage layout, and indexing strategy.
*   [**Query DSL**](docs/QUERY_LANGUAGE.md) - JSON-based query syntax and examples.
*   [**Research Goals**](docs/RESEARCH_GOALS.md) - The scientific hypotheses on emergent behavior and memory coherence.

## 🧪 Research Focus

MemoryGraph is an **instrument for cognitive architecture research**, not just a database. We aim to answer:

> *"Does storage architecture affect emergent behavior in AI agents?"*

### Novel Research Contributions

1.  **Graph-Aware HNSW** — Modify HNSW neighbor selection to prefer graph edges (publishable algorithm)
2.  **Memory Coherence Metrics** — Quantify semantic-structural alignment in agent memory
3.  **Cognitive Schema Primitives** — First-class support for episodic/semantic/procedural memory types
4.  **Emergence Instrumentation** — Built-in tools for studying self-organization in agent memory

### Testbed

[WhisperEngine](https://github.com/markcastillo/whisperengine-v2) — A multi-character Discord AI platform — serves as the primary research testbed. MemoryGraph will replace its current Qdrant + Neo4j dual-write architecture.

## 📄 License

MIT License (Proposed)
