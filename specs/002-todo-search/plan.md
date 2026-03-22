# Implementation Plan: Search Support for To-Do Items

**Branch**: `feature/002-todo-search` | **Date**: 2026-03-22 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/002-todo-search/spec.md`

## Summary

Complete and harden to-do item search on the existing list endpoint by keeping search as a query-parameter-based filter over title and note content, preserving pagination and sort behavior, and aligning OpenAPI, README, and automated tests with the actual runtime contract. The current workspace already contains most of the application, presentation, and PostgreSQL filtering logic, so the implementation focus is verifying behavior end to end, closing documentation gaps, and tightening edge-case handling where necessary.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1, Validator 0.20  
**Storage**: PostgreSQL via Diesel and r2d2 pool  
**Testing**: `cargo test`, integration tests in `src/starter/tests`, crate-level unit tests, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`  
**Target Platform**: Linux-hosted HTTP service  
**Project Type**: Layered Rust web service  
**Performance Goals**: Keep paginated list requests responsive for typical to-do workloads while preserving existing count and page metadata behavior when search is applied  
**Constraints**: Maintain `/api/v1` API stability, keep search on the existing list endpoint, avoid introducing a separate search subsystem, preserve DDD layer boundaries, and avoid breaking clients that omit the search parameter  
**Scale/Scope**: Single aggregate (`ToDoItem`), one existing list endpoint, one repository query path, OpenAPI/README updates, and matching automated verification

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- `PASS`: Layer boundaries stay intact. Search intent is represented in application query models, filtering is implemented in infrastructure, and HTTP parameter validation/documentation stays in presentation.
- `PASS`: Public API remains under `/api/v1` and extends existing query-parameter behavior instead of introducing a breaking endpoint change.
- `PASS`: The feature does not require new persistence schema changes, so there is no migration or data-shape risk in this plan.
- `PASS`: Verification spans unit and integration levels, covering request validation, repository-backed filtering behavior, and contract documentation.
- `PASS`: The design favors the simplest viable approach, using existing substring matching behavior rather than adding ranking, full-text indexing, or a new search service.
- `PASS WITH NOTE`: The current workspace already contains partial implementation for search. The plan therefore emphasizes validation, documentation parity, and edge-case completion to ensure the shipped contract matches the specification.

## Phase 0: Research

### Decisions

1. **Search matching strategy**
   - Decision: Keep case-insensitive substring matching across title and note content for this feature.
   - Rationale: The repository already implements this behavior, it satisfies the issue request for substring or full-text support, and it avoids unnecessary scope growth.
   - Alternatives considered: PostgreSQL full-text search was rejected for this feature because the user-visible requirement does not require ranking or stemming and the simpler behavior is already scaffolded.

2. **API surface**
   - Decision: Expose search exclusively through the existing `GET /api/v1/to-do-items` query parameters.
   - Rationale: This matches the spec, preserves client workflows, and avoids adding a second read path with overlapping semantics.
   - Alternatives considered: A dedicated `/search` endpoint was rejected because it would duplicate list behavior and expand documentation and maintenance cost.

3. **Blank search handling**
   - Decision: Treat whitespace-only search values as invalid input at the presentation boundary and treat an omitted search value as normal listing behavior.
   - Rationale: The current request model already validates blank search values, which keeps semantics explicit and avoids ambiguous "empty filter" behavior.
   - Alternatives considered: Silently normalizing blank values to no search was rejected because it weakens input validation and makes client mistakes harder to detect.

4. **Verification strategy**
   - Decision: Rely on existing integration-test infrastructure plus targeted unit tests for request mapping and validation.
   - Rationale: Search is a public API behavior that is best proven through real HTTP and repository-backed flows, while request-model edge cases remain cheap to validate in unit tests.
   - Alternatives considered: A separate contract-testing framework was rejected because the current repository already uses OpenAPI plus integration tests as the contract-verification mechanism.

## Phase 1: Design & Contracts

### Data Model Changes

- No new persistent entity or schema changes are required.
- `GetAllToDoItemsQuery` remains the application-level carrier for search intent via `search: Option<String>`.
- `GetAllToDoItemsQueryRequest` remains the transport-level query model and owns search validation, normalization, and sort parsing.
- `ToDoItem` remains the only domain entity involved; search is a retrieval concern, not a domain-state mutation.

### Application Design

- Keep search as part of `GetAllToDoItemsQuery` so application handlers continue to express list filtering without transport or Diesel details.
- Keep `ToDoItemRepository::get_all` as the single repository contract for list retrieval, with search applied as part of that query object.
- Avoid introducing a dedicated search service or handler because the existing list handler already matches the required use case.

### Infrastructure Design

- Keep PostgreSQL filtering in `PostgresToDoItemRepository::get_all` and the internal filtered-query builder.
- Ensure the same search filter is applied consistently to both the list query and the `total_items` count query so pagination metadata remains accurate.
- Preserve deleted-item filtering as part of the same query path so search does not reintroduce soft-deleted records into standard list results.
- If gaps are found during implementation, keep fixes localized to the query builder instead of spreading conditional search logic across handlers or routes.

### Presentation Design

- Keep `GET /api/v1/to-do-items` as the only public entry point for search.
- Keep `GetAllToDoItemsQueryRequest` as the boundary for:
  - page and page-size validation
  - blank search rejection
  - search normalization
  - sort parsing
- Ensure OpenAPI descriptions clearly state that search is optional, case-insensitive, and applied to title and note content.
- Keep problem-details responses for invalid query parameters.

### Documentation Design

- Align README list-query examples with the actual search contract.
- Ensure OpenAPI parameter descriptions match runtime behavior and acceptance scenarios.
- Avoid documenting implementation choices such as `ILIKE`, indexes, or Diesel-specific details in user-facing docs.

## Phase 2: Implementation Plan

1. **Audit current search behavior against the spec**
   - Confirm list filtering behavior for title matches, note matches, and non-matching items.
   - Confirm omitted-search behavior remains unchanged.
   - Confirm blank-search behavior is intentionally rejected and documented.

2. **Tighten presentation and contract documentation**
   - Update endpoint descriptions and parameter docs where wording is incomplete or inconsistent.
   - Verify README examples and OpenAPI metadata reflect the shipped behavior.
   - Keep the search contract aligned across spec, OpenAPI, and README.

3. **Harden verification**
   - Add or refine integration tests for:
     - title-only matches
     - note-only matches
     - non-matching searches returning empty results
     - omitted search preserving normal list behavior
     - search combined with existing list controls
   - Add or refine unit tests for request validation and normalization behavior.

4. **Apply minimal code fixes if verification exposes gaps**
   - Limit changes to request parsing, repository query construction, or response/documentation wiring.
   - Avoid refactoring unrelated list or search-adjacent code.

## Project Structure

### Documentation (this feature)

```text
specs/
└── 002-todo-search/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/
    │   └── todo-search-list.yaml
    └── checklists/
        └── requirements.md
```

### Source Code (repository root)

```text
src/
├── domain/
│   └── src/
│       ├── entities.rs
│       └── lib.rs
├── application/
│   └── src/
│       ├── handlers.rs
│       ├── queries.rs
│       ├── repositories.rs
│       └── services.rs
├── infrastructure/
│   └── src/
│       └── postgres_repositories.rs
├── presentation/
│   └── src/
│       ├── api/
│       │   ├── app.rs
│       │   └── api_doc.rs
│       ├── requests.rs
│       └── responses.rs
└── starter/
    └── tests/
        └── integration.rs
```

**Structure Decision**: Keep the existing layered workspace and complete the feature through narrow updates in the current application, infrastructure, presentation, and integration-test surfaces. No new crate, endpoint family, or persistence model is needed.

## Test Strategy

- Unit tests:
  - `GetAllToDoItemsQueryRequest` rejects blank search values
  - `GetAllToDoItemsQueryRequest` trims and maps valid search values correctly
  - sort and search parameters can coexist without invalid mapping
- Integration tests:
  - searching by title returns matching items
  - searching by note returns matching items
  - non-matching searches return an empty page payload
  - requests without `search` preserve existing listing behavior
  - search plus pagination/sort continues to produce stable metadata and ordering
  - deleted items remain excluded even when their title or note matches the search term
- Verification commands:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets --all-features`
  - `cargo test --workspace`

## Risks and Mitigations

- **Risk**: Search behavior may already exist but not be fully covered, leading to false confidence.
  - Mitigation: Add end-to-end integration cases for each acceptance path instead of relying on spot checks.
- **Risk**: Documentation could drift from the actual API contract.
  - Mitigation: Treat README and OpenAPI wording as part of the feature completion criteria and verify them in the same change.
- **Risk**: Search filtering could affect pagination metadata incorrectly if count and item queries diverge.
  - Mitigation: Keep filtering centralized in the repository query builder and verify `meta.total_items` in integration tests.

## Complexity Tracking

No constitution violations currently require exception handling.
