# Tasks: Explicit Error Types by Application Layer

**Input**: Design documents from `/specs/005-typed-layer-errors/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are required for this feature because the specification explicitly requires automated verification of layer-boundary translations and HTTP error mappings.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Capture the feature-level contract artifacts and confirm the target files for the refactor.

- [X] T001 Review and align feature documentation in `specs/005-typed-layer-errors/spec.md`, `specs/005-typed-layer-errors/plan.md`, and `specs/005-typed-layer-errors/contracts/http-error-contract.yaml`
- [X] T002 [P] Add the implementation entry points for typed errors in `src/application/src/lib.rs`, `src/infrastructure/src/lib.rs`, and `src/presentation/src/lib.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the shared typed-error foundation that all user stories depend on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T003 Create application-layer error types and result aliases in `src/application/src/errors.rs`
- [X] T004 Update repository trait signatures to use typed application errors in `src/application/src/repositories.rs`
- [X] T005 Update handler signatures and exports to propagate typed application errors in `src/application/src/handlers.rs` and `src/application/src/lib.rs`
- [X] T006 Refine infrastructure error definitions for repository translation in `src/infrastructure/src/errors.rs` and `src/infrastructure/src/lib.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Keep Layer Failures Explicit and Contained (Priority: P1) 🎯 MVP

**Goal**: Make application and infrastructure boundaries expose explicit typed failures instead of generic cross-layer `anyhow` flow.

**Independent Test**: Trigger representative not-found, version-conflict, and unexpected repository failures and verify the application boundary exposes only the defined application error categories.

### Tests for User Story 1

- [X] T007 [P] [US1] Add repository error translation tests in `src/infrastructure/src/postgres_repositories.rs`
- [X] T008 [P] [US1] Add application handler error propagation tests in `src/application/src/services.rs`

### Implementation for User Story 1

- [X] T009 [US1] Replace normal-path `anyhow::Result` repository return types with typed application results in `src/application/src/repositories.rs`
- [X] T010 [US1] Refactor query handlers to propagate typed application errors in `src/application/src/handlers.rs`
- [X] T011 [US1] Refactor in-memory and service test doubles to emit typed application errors in `src/application/src/services.rs` and `src/starter/tests/memory_management_tests.rs`
- [X] T012 [US1] Refactor PostgreSQL repository methods to translate persistence failures into typed boundary errors in `src/infrastructure/src/postgres_repositories.rs`
- [X] T013 [US1] Remove no-longer-needed normal-path `anyhow` usage from application and infrastructure crates in `src/application/Cargo.toml`, `src/infrastructure/src/postgres_repositories.rs`, and `src/application/src/handlers.rs`

**Checkpoint**: User Story 1 should now expose explicit typed failures at the application boundary without leaking infrastructure internals

---

## Phase 4: User Story 2 - Return Stable HTTP Error Responses (Priority: P2)

**Goal**: Map stable application errors to stable HTTP problem-details responses without exposing internal implementation details.

**Independent Test**: Exercise representative HTTP failure flows and verify the service returns the documented `404`, `412`, and sanitized `500` problem responses from application error categories.

### Tests for User Story 2

- [X] T014 [P] [US2] Add presentation-layer error mapping tests in `src/presentation/src/errors.rs`
- [X] T015 [P] [US2] Add HTTP integration tests for representative failure mappings in `src/starter/tests/integration.rs`

### Implementation for User Story 2

- [X] T016 [US2] Replace `From<anyhow::Error>` downcasting with application-error mapping in `src/presentation/src/errors.rs`
- [X] T017 [US2] Update HTTP handlers to use the typed application error contract in `src/presentation/src/api/app.rs`
- [X] T018 [US2] Align problem-details response documentation with stable failure categories in `src/presentation/src/responses.rs` and `src/presentation/src/api/app.rs`
- [X] T019 [US2] Update the HTTP contract artifact with the finalized outward error mappings in `specs/005-typed-layer-errors/contracts/http-error-contract.yaml`

**Checkpoint**: User Story 2 should now return stable problem-details responses from application error categories with no leaked internal details

---

## Phase 5: User Story 3 - Verify Error Mapping with Automated Tests (Priority: P3)

**Goal**: Lock the boundary and HTTP mapping behavior down with maintainable automated verification and contributor guidance.

**Independent Test**: Run the automated test suite covering representative layer-boundary and HTTP error mappings and confirm regressions in outward behavior are detected.

### Tests for User Story 3

- [X] T020 [P] [US3] Add focused tests for unexpected internal failure sanitization in `src/presentation/src/errors.rs` and `src/starter/tests/integration.rs`
- [X] T021 [P] [US3] Add regression coverage for success-path behavior after the error refactor in `src/starter/tests/integration.rs`

### Implementation for User Story 3

- [X] T022 [US3] Update quick validation steps for the feature in `specs/005-typed-layer-errors/quickstart.md`
- [X] T023 [US3] Update contributor-facing error-handling guidance in `README.md`
- [X] T024 [US3] Verify the plan-aligned test matrix and contract notes in `specs/005-typed-layer-errors/plan.md` and `specs/005-typed-layer-errors/research.md`

**Checkpoint**: User Story 3 should now have durable regression coverage and supporting documentation for future maintainers

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup and full verification across all stories

- [X] T025 Run formatting and lint verification with `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features`
- [ ] T026 Run workspace verification with `cargo test --workspace`
- [X] T027 [P] Reconcile final feature documentation in `specs/005-typed-layer-errors/spec.md`, `specs/005-typed-layer-errors/quickstart.md`, and `specs/005-typed-layer-errors/contracts/http-error-contract.yaml`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational completion
- **User Story 2 (Phase 4)**: Depends on Foundational completion and benefits from User Story 1 typed error contracts
- **User Story 3 (Phase 5)**: Depends on User Story 1 and User Story 2 behavior being implemented
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - no dependency on other user stories
- **User Story 2 (P2)**: Depends on User Story 1 because HTTP mapping requires the typed application error contract
- **User Story 3 (P3)**: Depends on User Story 1 and User Story 2 because it hardens the completed mappings with regression coverage and docs

### Within Each User Story

- Tests should be written before or alongside implementation and must fail before the corresponding implementation is considered complete
- Error contracts before repository and handler refactors
- Repository and handler refactors before HTTP mapping changes
- HTTP mapping before final regression and documentation hardening

### Parallel Opportunities

- `T002` can run in parallel with `T001`
- `T007` and `T008` can run in parallel within User Story 1
- `T014` and `T015` can run in parallel within User Story 2
- `T020` and `T021` can run in parallel within User Story 3
- `T027` can run in parallel with the final verification commands once implementation is stable

---

## Parallel Example: User Story 1

```bash
# Launch User Story 1 test work together:
Task: "Add repository error translation tests in src/infrastructure/src/postgres_repositories.rs"
Task: "Add application handler error propagation tests in src/application/src/services.rs"
```

---

## Parallel Example: User Story 2

```bash
# Launch User Story 2 verification work together:
Task: "Add presentation-layer error mapping tests in src/presentation/src/errors.rs"
Task: "Add HTTP integration tests for representative failure mappings in src/starter/tests/integration.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Stop and validate typed application and infrastructure boundary behavior before touching HTTP mapping

### Incremental Delivery

1. Complete Setup + Foundational to establish typed error primitives
2. Deliver User Story 1 to remove generic cross-layer error flow
3. Deliver User Story 2 to lock stable HTTP problem-details behavior
4. Deliver User Story 3 to harden tests and documentation
5. Finish with workspace-wide verification and contract cleanup

### Parallel Team Strategy

1. One developer completes Foundation tasks
2. After Foundation:
   - Developer A: User Story 1 repository and handler refactor
   - Developer B: User Story 2 HTTP mapping tests and contract updates once User Story 1 types are available
   - Developer C: User Story 3 docs and regression additions after HTTP mappings stabilize

---

## Notes

- All tasks follow the required checklist format with task ID, optional `[P]`, optional story label, and exact file paths
- The suggested MVP scope is User Story 1 because it establishes the typed boundary contract required by later stories
- Total tasks: 27
- Task count by story:
  - US1: 7 tasks
  - US2: 6 tasks
  - US3: 5 tasks
- Parallel opportunities identified: 5
- T026 remains open because `cargo test --workspace` requires Docker for the existing `starter` integration suite, and Docker was not available in this session
