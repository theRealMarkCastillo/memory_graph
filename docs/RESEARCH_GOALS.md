# Research Goals: The Science of Cognitive Agent Memory

MemoryGraph is a **research instrument**, not just a database. It exists to answer questions that existing vector-graph solutions cannot.

## Positioning: What's Novel

| Claim | Neo4j + Vectors | FalkorDB | **MemoryGraph** |
|-------|-----------------|----------|-----------------|
| Vector + Graph storage | ✅ (bolt-on) | ✅ (extension) | ✅ (**unified**) |
| Hybrid query planner | ❌ Graph-first | ❌ Graph-first | ✅ **Cost-based** |
| Cognitive schema | ❌ | ❌ | ✅ **Memory types** |
| Temporal decay | ❌ Application | ❌ Application | ✅ **Built-in** |
| Coherence metrics | ❌ | ❌ | ✅ **Research instrumentation** |
| Graph-Aware HNSW | ❌ | ❌ | ✅ **Novel algorithm** |

## Primary Hypothesis

**"Storage architecture affects emergent behavior in AI agents."**

Specifically: When vectors and graph edges are stored together with cognitive primitives (decay, importance, memory types), agents develop more coherent long-term memory than with dual-system architectures.

## Core Research Questions

### 1. Coherence Convergence
*   **Question:** Do memories that are semantically similar naturally form graph clusters over time without explicit instruction?
*   **Metric:** *Semantic-Structural Alignment Score*. Correlation between cosine similarity of two nodes and their graph distance (shortest path).
*   **Experiment:** Run an agent for 10,000 turns. Measure if the graph topology aligns with the vector space topology.

### 2. The "Orphan Knowledge" Problem
*   **Question:** Can we automatically detect "orphan" memories—facts that the agent knows (vector accessible) but cannot reason about (graph isolated)?
*   **Goal:** Develop algorithms that proactively suggest edges to connect orphan clusters to the main knowledge graph based on vector proximity.

### 3. Retrieval Quality: Hybrid vs. Disjoint
*   **Question:** Does a hybrid query (Vector + Graph Expansion) provide better context for LLMs than RAG (Vector only) or Graph RAG (Graph only)?
*   **Metric:** *Context Relevance Score* (judged by LLM).
*   **Experiment:** Compare response quality of WhisperEngine using Qdrant+Neo4j vs. MemoryGraph on the same conversation dataset.

### 4. Graph-Aware HNSW (Novel Algorithm)
*   **Technical Research:** Modify HNSW index construction to bias neighbor selection towards existing graph edges.
*   **Hypothesis:** A "Cognitive HNSW" index will provide faster convergence for agent-relevant queries than standard HNSW.
*   **Publication Target:** VLDB, NeurIPS (systems track), or SIGMOD.

### 5. Cognitive Schema Impact
*   **Question:** Does having first-class memory types (episodic, semantic, procedural) improve agent coherence vs. flat metadata?
*   **Metric:** Compare retrieval precision and agent response quality with/without typed memories.
*   **Experiment:** A/B test WhisperEngine with typed vs. untyped memory storage.

## Roadmap for Validation

1.  **Baseline:** Establish metrics for WhisperEngine on Qdrant + Neo4j (current architecture).
2.  **Prototype:** Build MemoryGraph v0.1 (in-memory Rust prototype with cognitive schema).
3.  **Simulation:** Replay 100k WhisperEngine interaction logs through MemoryGraph.
4.  **Analysis:** Compare:
    - Graph topology evolution
    - Retrieval latency and relevance
    - Memory coherence scores over time
5.  **Publication:** Submit findings to:
    - **VLDB/SIGMOD** (database systems track) — for Graph-Aware HNSW
    - **NeurIPS/ICLR** (AI systems track) — for cognitive schema impact
    - **CogSci** — for emergence/coherence findings

## Why Not Just Use Neo4j + Vectors?

We could. And that's a valid baseline. But Neo4j's vector indexes are:
1. **Property-based** — Vectors are node properties, not co-located with edges
2. **Graph-first planner** — The optimizer doesn't know how to use vectors efficiently
3. **No cognitive primitives** — Decay, importance, memory types are application concerns

MemoryGraph is a bet that **cognition-first design** enables research insights that general-purpose databases cannot.
