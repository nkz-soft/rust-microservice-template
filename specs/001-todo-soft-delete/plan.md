# Implementation Plan: Soft Delete and Audit Metadata for To-Do Items

**Branch**: `001-todo-soft-delete` | **Date**: 2026-03-22 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/001-todo-soft-delete/spec.md`

## Summary

Replace hard deletion of to-do items with soft deletion by extending the to-do aggregate and persistence schema with deletion metadata, filtering deleted rows out of standard reads, and adding one restricted audit endpoint for fetching a deleted item by ID. The design stays within the current layered DDD structure, uses forward-only Diesel migrations, preserves `/api/v1` stability for standard clients, and adds verification at unit, repository, HTTP integration, and OpenAPI levels.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1  
**Storage**: PostgreSQL via Diesel and r2d2 pool  
**Testing**: `cargo test`, integration tests in `src/starter/tests`, unit tests in crate modules, `cargo fmt --check`, `cargo clippy --all-targets --all-features`  
**Target Platform**: Linux-hosted HTTP service  
**Project Type**: Layered Rust web service  
**Performance Goals**: Preserve existing request-path behavior for standard reads and keep list queries paginated and index-friendly after soft-delete filtering  
**Constraints**: Maintain `/api/v1` versioning, keep standard endpoints hiding deleted items, introduce only forward-only Diesel migrations, no broad new auth subsystem, no breaking response changes for non-deleted items  
**Scale/Scope**: Single bounded feature affecting one aggregate (`ToDoItem`), one database table, one new restricted retrieval path, and related tests/docs

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- `PASS`: Layer boundaries remain explicit. Domain owns deletion state and invariants, application owns repository contracts and handlers, infrastructure owns Diesel and PostgreSQL implementation, presentation owns route/header validation and problem-details mapping, starter owns wiring.
- `PASS`: API contract remains versioned under `/api/v1`; standard endpoint behavior for active items remains stable.
- `PASS`: Persistence changes are delivered through forward-only Diesel migrations with matching schema updates.
- `PASS`: New behavior includes success-path and failure-path verification at the proper levels.
- `PASS`: Configuration-based restriction for audit access fits the constitution requirement that runtime behavior be configurable.
- `PASS WITH ASSUMPTION`: Because the service has no existing auth subsystem, restricted audit access will use a narrow configuration-backed audit token header for the new audit endpoint instead of introducing a full authentication framework in this feature.

## Phase 0: Research

### Decisions

1. **Soft delete representation**
   - Decision: Extend `to_do_items` with nullable `deleted_at` and nullable `deleted_by`.
   - Rationale: Matches the specification directly, preserves current row identity/versioning, and lets standard queries filter on deletion state without adding a second table.
   - Alternatives considered: Separate audit table was rejected because it adds more persistence and mapping complexity than this feature requires.

2. **Standard endpoint behavior for deleted items**
   - Decision: Standard `GET /to-do-items/{id}` and `PUT /to-do-items/{id}` treat deleted records as `404 Not Found`.
   - Rationale: Deleted items must be unavailable through normal access, and `404` preserves the current client-facing contract pattern used by missing items.
   - Alternatives considered: `410 Gone` was rejected because it leaks deletion state through standard endpoints and would expand API contract complexity.

3. **Restricted audit access mechanism**
   - Decision: Add a new audit-only item-by-id read path protected by a configured shared audit token header.
   - Rationale: The repository has no auth model today; a narrow configured token satisfies “restricted” access with minimal new surface area and can be implemented entirely in presentation/starter/settings without cross-cutting auth infrastructure.
   - Alternatives considered: Full role-based auth was rejected as out of scope; exposing deleted items through existing endpoints was rejected because it conflicts with the spec’s default-hidden requirement.

4. **Delete idempotency**
   - Decision: Repeated delete requests for an already deleted item return success and do not rewrite deletion metadata.
   - Rationale: This avoids duplicate audit mutations and simplifies client retries.
   - Alternatives considered: Returning `404` on repeat delete was rejected because the row still exists and retry behavior becomes less predictable.

## Phase 1: Design & Contracts

### Data Model Changes

- Extend `domain::ToDoItem` with `deleted_at: Option<DateTime<Utc>>` and `deleted_by: Option<Uuid>` or equivalent actor identifier type selected during implementation.
- Add explicit domain helpers to mark deletion once and to expose whether an item is active or deleted.
- Preserve current lifecycle fields (`created_at`, `updated_at`, `version`, `due_at`) and optimistic locking semantics for active updates.

### Application Design

- Extend `ToDoItemRepository` with audit-aware read and soft-delete semantics rather than adding persistence logic to handlers.
- Introduce or adapt command/query handlers for:
  - soft delete by ID with optional actor identity
  - audit read of deleted item by ID
- Keep standard get/list/update handlers delegating deleted-item visibility to repository filters and not branching on Diesel details.

### Infrastructure Design

- Add a Diesel migration to append `deleted_at` and `deleted_by` columns to `to_do_items`.
- Update Diesel schema and PostgreSQL repository structs/mappers for the new columns.
- Replace hard `DELETE` with an `UPDATE` that sets deletion metadata only if the row is still active.
- Apply `deleted_at IS NULL` filtering to standard list and get queries.
- Add a dedicated repository path for retrieving a deleted item by ID for the audit flow.
- Preserve query efficiency by explicitly filtering on deletion state in both count and item queries.

### Presentation Design

- Keep existing endpoints under `/api/v1/to-do-items` unchanged for active-item behavior.
- Add a restricted audit route under `/api/v1/audit/to-do-items/{id}` or equivalent audit-prefixed path for deleted item retrieval only.
- Validate the audit token header in presentation before invoking the audit query handler.
- Map missing/invalid audit token to problem-details responses without leaking internal details.
- Ensure OpenAPI documents the new audit endpoint and its required header while standard endpoints continue to omit deleted items.

### Configuration Design

- Extend `application::Settings` with an audit section for a required token value or disabled state.
- Support both config-file and environment-variable overrides for audit settings, consistent with current configuration patterns.
- Wire the audit configuration into starter and presentation through typed app data rather than globals.

### Observability

- Add structured logs for soft-delete operations and audit endpoint access attempts, avoiding token leakage.
- Keep logs at info/warn level appropriate to current project patterns.

## Phase 2: Implementation Plan

1. **Domain and application contracts**
   - Add deletion metadata fields and deletion-state helpers to `domain::ToDoItem`.
   - Extend repository traits and application command/query types for soft delete and audit read.
   - Update service wiring to expose the new handler where needed.

2. **Persistence and repository implementation**
   - Add forward-only Diesel migration for `deleted_at` and `deleted_by`.
   - Regenerate/update Diesel schema and repository row structs.
   - Convert repository delete into soft delete with idempotent behavior.
   - Filter deleted rows out of standard `get_all` and `get_by_id`.
   - Add repository method for audit retrieval of a deleted item by ID.

3. **Presentation and configuration**
   - Extend `Settings` and config loading with audit token settings.
   - Add typed presentation support for audit access configuration.
   - Implement the new restricted audit endpoint and header validation.
   - Update problem-details mappings and OpenAPI registration.

4. **Verification and documentation**
   - Add unit tests for domain deletion-state helpers and settings parsing.
   - Add repository tests for soft delete filtering and idempotent delete behavior where feasible.
   - Extend HTTP integration tests to cover delete, hidden standard reads, blocked updates, audit retrieval success, and audit retrieval unauthorized cases.
   - Update README and OpenAPI examples for audit behavior and configuration.

## Project Structure

### Documentation (this feature)

```text
specs/
└── 001-todo-soft-delete/
    ├── spec.md
    ├── plan.md
    ├── tasks.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/
    │   └── audit-todo-item.yaml
    └── checklists/
        └── requirements.md
```

### Source Code (repository root)

```text
src/
├── domain/
│   └── src/
│       ├── entities.rs
│       └── schema.rs
├── application/
│   └── src/
│       ├── handlers.rs
│       ├── queries.rs
│       ├── repositories.rs
│       ├── services.rs
│       └── settings.rs
├── infrastructure/
│   └── src/
│       ├── errors.rs
│       ├── migrations/
│       └── postgres_repositories.rs
├── presentation/
│   └── src/
│       ├── api/
│       │   ├── app.rs
│       │   └── api_doc.rs
│       ├── config.rs
│       ├── errors.rs
│       ├── requests.rs
│       └── responses.rs
└── starter/
    ├── src/
    │   └── lib.rs
    └── tests/
        └── integration.rs
```

**Structure Decision**: Keep the existing layered workspace and implement the feature as narrowly scoped changes across the existing crates. No new crate or subsystem is introduced.

## Test Strategy

- Domain unit tests:
  - marking an active item as deleted sets metadata once
  - deleted items report inactive state
- Settings unit tests:
  - audit token loads from file and env override
- Infrastructure/repository tests:
  - soft delete keeps row present
  - standard get/list exclude deleted rows
  - repeated delete is idempotent and preserves first deletion metadata
  - audit read returns deleted item only
- Presentation/integration tests:
  - delete request succeeds and subsequent standard `GET` returns `404`
  - deleted item is excluded from paginated list results
  - `PUT` on deleted item returns `404`
  - audit endpoint with valid token returns deleted item and deletion metadata
  - audit endpoint without token or wrong token returns an authorization failure response
- Verification commands:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets --all-features`
  - `cargo test --workspace`

## Risks and Mitigations

- **Risk**: Introducing a new restricted endpoint without an existing auth subsystem could grow in scope.
  - Mitigation: Use a single configuration-backed audit token header and keep the endpoint read-only and item-by-id only.
- **Risk**: Query regressions could let deleted items leak into standard list counts or pages.
  - Mitigation: Apply deletion filtering in both count and item queries and cover pagination in integration tests.
- **Risk**: Delete semantics could conflict with optimistic locking assumptions.
  - Mitigation: Treat deleted items as unavailable for standard updates and cover stale/update-after-delete flows in tests.

## Complexity Tracking

No constitution violations currently require exception handling.
