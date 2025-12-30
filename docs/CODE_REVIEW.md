# Code & Design Review: MemoryGraph v0.1

## 1. What is "Faked" (Prototype Shortcuts)

### 1.1 Vector Index (`SimpleVectorIndex`)
*   **Current State:** Uses a `Vec<(Uuid, Vec<f32>)>` and performs a brute-force linear scan (O(N)) for every search.
*   **Impact:** Fine for <10k items, but will be extremely slow for production workloads (100k+).
*   **Fix:** Replace with `hnsw_rs` or `lance` to get O(log N) search performance.

### 1.2 Embeddings
*   **Current State:** The system expects the client to provide raw `Vec<f32>` embeddings. There is no internal embedding generation.
*   **Impact:** The user must run a separate Python script or API call to OpenAI/HuggingFace to get vectors before inserting them.
*   **Verdict:** This is actually a **good design choice** for a database. It keeps the core lean. However, a "Client SDK" should handle this.

### 1.3 Graph Traversal
*   **Current State:** Simple BFS expansion in memory.
*   **Impact:** If a node has 10,000 edges, loading them all into a `Vec` in memory during traversal will spike RAM usage.
*   **Fix:** Implement an iterator-based traversal that lazily loads edges from `redb`.

### 1.4 Scoring Logic
*   **Current State:** `Score = ParentScore * 0.5`.
*   **Missing:** The "Cognitive Ranking" formula defined in the architecture (`Recency`, `Importance`, `Decay`) is not yet implemented in `QueryEngine`.
*   **Fix:** Update `QueryEngine` to fetch `Memory` metadata and apply the full formula.

## 2. What is Missing (Gap Analysis)

### 2.1 API Layer
*   **Missing:** There is no HTTP or gRPC server. The current "demo" is just a CLI tool running the library directly.
*   **Next Step:** Add `axum` (HTTP) or `tonic` (gRPC) to expose `save_memory` and `query` endpoints.

### 2.2 Research Instrumentation
*   **Missing:**
    *   **Coherence Metrics:** No code to calculate "Semantic-Structural Alignment".
    *   **Orphan Detection:** No background job to find isolated nodes.
    *   **Decay System:** No background job to decrease `importance` over time.

### 2.3 Concurrency
*   **Missing:** The `QueryEngine` is synchronous. `redb` handles storage concurrency, but the query logic blocks the thread.
*   **Next Step:** Make `QueryEngine::execute` async.

## 3. Recommendations

### Immediate Priorities (v0.2)
1.  **Implement "Cognitive Scoring":** Update `QueryEngine` to use `recency` and `importance` in the ranking.
2.  **Add API Server:** Create a simple `axum` server so we can talk to it from Python.
3.  **Integrate Real HNSW:** Swap `SimpleVectorIndex` for `hnsw_rs`.

### Research Priorities
1.  **Implement Coherence Metrics:** This is the core differentiator. We need a function that runs periodically and outputs a "Coherence Score" for the whole graph.

## 4. Conclusion
The current implementation is a **valid functional prototype**. It proves the data model and the hybrid query concept. It is not yet "production ready" due to the O(N) vector search and lack of API, but it is ready for **research experiments** on small datasets (<10k memories).
