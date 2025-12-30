# Implementation Plan

## Phase 1: Core Scaffolding & Data Models
- [x] Initialize Rust project
- [x] Define `Cargo.toml` dependencies
- [x] Create module structure
- [x] Implement `Memory`, `MemoryType`, `Edge` structs

## Phase 2: Storage Engine
- [x] Setup `redb` manager
- [x] Implement CRUD for Memories (Basic Save/Get/List implemented)

## Phase 3: Indexes
- [x] Implement Graph Adjacency Index (Implemented in `StorageManager` with `redb`)
- [x] Implement Vector Index Interface (Moved to `src/index/vector.rs`, `SimpleVectorIndex` implemented)

## Phase 4: Query Engine
- [x] Implement Query DSL parsing (Structs defined)
- [x] Implement Basic Execution Logic (Implemented in `src/query/engine.rs`)

## Phase 5: Research Instrumentation
- [ ] Implement Coherence Metrics (Semantic-Structural Alignment)
- [ ] Implement Orphan Detection
- [ ] Implement Memory Decay Logic
