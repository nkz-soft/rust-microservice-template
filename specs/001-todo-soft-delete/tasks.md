# Tasks: Soft Delete and Audit Metadata for To-Do Items

**Input**: Design documents from `/specs/001-todo-soft-delete/`
**Prerequisites**: [plan.md](D:/Projects/open-source/nz/rust-microservice-template/specs/001-todo-soft-delete/plan.md), [spec.md](D:/Projects/open-source/nz/rust-microservice-template/specs/001-todo-soft-delete/spec.md), [research.md](D:/Projects/open-source/nz/rust-microservice-template/specs/001-todo-soft-delete/research.md), [data-model.md](D:/Projects/open-source/nz/rust-microservice-template/specs/001-todo-soft-delete/data-model.md), [quickstart.md](D:/Projects/open-source/nz/rust-microservice-template/specs/001-todo-soft-delete/quickstart.md), [audit-todo-item.yaml](D:/Projects/open-source/nz/rust-microservice-template/specs/001-todo-soft-delete/contracts/audit-todo-item.yaml)

**Tests**: Automated verification is required by the specification, so each user story includes test tasks before implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g. `US1`, `US2`, `US3`)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare shared persistence artifacts for soft-delete support

- [X] T001 Add the forward-only soft-delete migration in `src/infrastructure/src/migrations/2026-03-22-000000_soft_delete_todo_items/up.sql`
- [X] T002 Add the matching rollback migration in `src/infrastructure/src/migrations/2026-03-22-000000_soft_delete_todo_items/down.sql`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core contracts and shared wiring that MUST be complete before user stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T003 Update soft-delete columns and query types in `src/domain/src/schema.rs`
- [X] T004 Extend the `ToDoItem` aggregate with deletion metadata and state helpers in `src/domain/src/entities.rs`
- [X] T005 [P] Add soft-delete and audit repository contracts in `src/application/src/repositories.rs`
- [X] T006 [P] Add delete and audit query models in `src/application/src/queries.rs`
- [X] T007 [P] Wire soft-delete and audit handlers through `src/application/src/handlers.rs`
- [X] T008 [P] Expose the new handlers from `src/application/src/services.rs`
- [X] T009 [P] Add audit token configuration models and loaders in `src/application/src/settings.rs`
- [X] T010 Add shared HTTP error mapping for hidden deleted items and audit authorization failures in `src/presentation/src/errors.rs`

**Checkpoint**: Foundation ready; user story implementation can proceed

---

## Phase 3: User Story 1 - Hide Deleted Items from Standard Use (Priority: P1) 🎯 MVP

**Goal**: Replace hard delete with soft delete while keeping standard list, get, and update flows hidden from deleted items

**Independent Test**: Create an item, delete it, then verify standard `GET`, paginated list, and `PUT` flows behave as though the item is unavailable

### Tests for User Story 1

- [X] T011 [P] [US1] Add repository tests for soft-delete filtering and idempotent delete behavior in `src/infrastructure/src/postgres_repositories.rs`
- [X] T012 [P] [US1] Extend HTTP integration coverage for delete, hidden get, hidden list, and blocked update flows in `src/starter/tests/integration.rs`

### Implementation for User Story 1

- [X] T013 [US1] Implement soft-delete-aware row mappings and standard query filters in `src/infrastructure/src/postgres_repositories.rs`
- [X] T014 [US1] Replace hard delete with idempotent soft delete in `src/infrastructure/src/postgres_repositories.rs`
- [X] T015 [US1] Keep deleted items hidden behind existing standard routes in `src/presentation/src/api/app.rs`

**Checkpoint**: User Story 1 is complete when deleted items no longer appear through standard client workflows

---

## Phase 4: User Story 2 - Preserve Deletion Audit Details (Priority: P2)

**Goal**: Capture and preserve deletion timestamp and optional actor identity whenever an item is soft-deleted

**Independent Test**: Delete one item with actor context and one without, then verify `deleted_at` is always set and `deleted_by` is only present when supplied

### Tests for User Story 2

- [X] T016 [P] [US2] Add aggregate tests for deletion metadata and deleted-state helpers in `src/domain/src/entities.rs`
- [X] T017 [P] [US2] Add settings tests for audit token file loading and environment overrides in `src/application/src/settings.rs`
- [X] T018 [P] [US2] Extend HTTP integration coverage for deletion metadata capture scenarios in `src/starter/tests/integration.rs`

### Implementation for User Story 2

- [X] T019 [US2] Persist `deleted_at` and `deleted_by` through repository row models and conversions in `src/infrastructure/src/postgres_repositories.rs`
- [X] T020 [US2] Thread optional deletion actor metadata through delete handlers and service calls in `src/application/src/handlers.rs`
- [X] T021 [US2] Add deletion metadata fields to API response models used for audit views in `src/presentation/src/responses.rs`

**Checkpoint**: User Story 2 is complete when deletion metadata is preserved consistently without requiring actor identity

---

## Phase 5: User Story 3 - Review Deleted Item by ID for Audit Purposes (Priority: P3)

**Goal**: Provide a restricted audit-only endpoint that retrieves a deleted item by ID with its deletion metadata

**Independent Test**: Soft-delete an item, call the audit endpoint with a valid token to retrieve it, and verify missing or invalid tokens are rejected

### Tests for User Story 3

- [X] T022 [P] [US3] Update the audit endpoint contract for authorization and deleted-item retrieval in `specs/001-todo-soft-delete/contracts/audit-todo-item.yaml`
- [X] T023 [P] [US3] Extend HTTP integration coverage for authorized and unauthorized audit retrieval in `src/starter/tests/integration.rs`

### Implementation for User Story 3

- [X] T024 [US3] Add audit token request parsing and header validation in `src/presentation/src/requests.rs`
- [X] T025 [US3] Implement the restricted deleted-item retrieval route in `src/presentation/src/api/app.rs`
- [X] T026 [US3] Register audit configuration and route wiring in `src/presentation/src/config.rs`
- [X] T027 [US3] Update OpenAPI registration for the audit endpoint in `src/presentation/src/api/api_doc.rs`
- [X] T028 [US3] Wire audit settings and services into application startup in `src/starter/src/lib.rs`

**Checkpoint**: User Story 3 is complete when authorized audit consumers can retrieve deleted items without exposing them through standard endpoints

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation and final verification across all stories

- [X] T029 [P] Update audit configuration and usage documentation in `README.md`
- [X] T030 [P] Refresh local verification steps for audit access in `specs/001-todo-soft-delete/quickstart.md`
- [X] T031 Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`, and `cargo test --workspace` from `Cargo.toml`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies; start immediately
- **Phase 2: Foundational**: Depends on Phase 1 and blocks all user stories
- **Phase 3: US1**: Depends on Phase 2; defines the MVP
- **Phase 4: US2**: Depends on Phase 2 and builds on soft-delete persistence from US1
- **Phase 5: US3**: Depends on Phase 2 and on the deletion metadata introduced in US2
- **Phase 6: Polish**: Depends on completion of the intended user stories

### User Story Dependencies

- **US1 (P1)**: No user-story dependency after foundational work
- **US2 (P2)**: Depends on US1 soft-delete behavior being in place
- **US3 (P3)**: Depends on US2 deletion metadata and audit configuration support

### Within Each User Story

- Tests should be written before the corresponding implementation tasks
- Repository and domain changes precede route-level integration
- Audit route wiring follows settings and response-model support

### Parallel Opportunities

- `T005` through `T009` can run in parallel once `T003` and `T004` are complete
- `T011` and `T012` can run in parallel for US1
- `T016`, `T017`, and `T018` can run in parallel for US2
- `T022` and `T023` can run in parallel for US3
- `T029` and `T030` can run in parallel during polish

---

## Parallel Example: User Story 1

```bash
Task: "Add repository tests for soft-delete filtering and idempotent delete behavior in src/infrastructure/src/postgres_repositories.rs"
Task: "Extend HTTP integration coverage for delete, hidden get, hidden list, and blocked update flows in src/starter/tests/integration.rs"
```

## Parallel Example: User Story 2

```bash
Task: "Add aggregate tests for deletion metadata and deleted-state helpers in src/domain/src/entities.rs"
Task: "Add settings tests for audit token file loading and environment overrides in src/application/src/settings.rs"
Task: "Extend HTTP integration coverage for deletion metadata capture scenarios in src/starter/tests/integration.rs"
```

## Parallel Example: User Story 3

```bash
Task: "Update the audit endpoint contract for authorization and deleted-item retrieval in specs/001-todo-soft-delete/contracts/audit-todo-item.yaml"
Task: "Extend HTTP integration coverage for authorized and unauthorized audit retrieval in src/starter/tests/integration.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate standard delete and hidden-read behavior before expanding scope

### Incremental Delivery

1. Deliver US1 to establish soft-delete behavior
2. Add US2 to preserve audit metadata without changing standard client behavior
3. Add US3 to expose restricted audit retrieval
4. Finish with documentation and full verification

### Parallel Team Strategy

1. One developer completes Phases 1 and 2
2. A second developer can prepare US1 integration coverage while repository work is underway
3. After US1 lands, US2 metadata work and US3 contract/test preparation can proceed in parallel

---

## Notes

- All tasks follow the required checklist format with explicit IDs and file paths
- `[US1]`, `[US2]`, and `[US3]` labels map directly to the user stories in `spec.md`
- The generated task graph assumes the active feature directory is `specs/001-todo-soft-delete/` because the prerequisite script currently points to a missing stale feature folder
