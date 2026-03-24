# Tasks: Observability for Service Operations

**Input**: Design documents from `/specs/004-observability-metrics/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: This feature explicitly requires automated verification for successful requests, failed requests, request ID propagation, and metrics retrieval.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare the workspace for tracing-native observability and align feature artifacts with the implementation entry points

- [X] T001 Add observability dependencies to `D:\Projects\open-source\nz\rust-microservice-template\Cargo.toml` and `D:\Projects\open-source\nz\rust-microservice-template\src\starter\Cargo.toml`
- [X] T002 [P] Review and align the observability contract and local validation steps in `D:\Projects\open-source\nz\rust-microservice-template\specs\004-observability-metrics\contracts\observability-http.yaml` and `D:\Projects\open-source\nz\rust-microservice-template\specs\004-observability-metrics\quickstart.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish configuration and composition-root observability wiring that all user stories depend on

**⚠️ CRITICAL**: No user story work should begin until this phase is complete

- [X] T003 Extend observability runtime settings and environment override coverage in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\settings.rs`
- [X] T004 [P] Create shared tracing and metrics setup helpers in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\observability.rs`
- [X] T005 Replace legacy logger startup with tracing subscriber initialization in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\main.rs`
- [X] T006 Update HTTP server composition to install shared observability middleware and exporter state in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\lib.rs`

**Checkpoint**: Runtime configuration and shared observability wiring are ready for story-level delivery

---

## Phase 3: User Story 1 - Trace Individual Requests End to End (Priority: P1) 🎯 MVP

**Goal**: Deliver structured request tracing with stable request correlation IDs propagated through operational records and HTTP responses

**Independent Test**: Send successful and failing requests through the service, verify the same request ID is returned in the response and emitted for the traced request flow, and confirm inbound request IDs are preserved.

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T007 [P] [US1] Add integration tests for generated request IDs, preserved inbound request IDs, and failed-request correlation in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`
- [X] T008 [P] [US1] Add unit coverage for observability settings defaults and request ID helper behavior in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\settings.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\observability.rs`

### Implementation for User Story 1

- [X] T009 [US1] Implement request ID extraction, generation, and response-header propagation middleware in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\observability.rs`
- [X] T010 [US1] Replace Actix `Logger` request logging with tracing-aware request instrumentation in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\lib.rs`
- [X] T011 [US1] Document request correlation response headers for versioned API endpoints in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\api_doc.rs`

**Checkpoint**: User Story 1 should now be independently functional and testable as the MVP

---

## Phase 4: User Story 2 - Monitor Service Health Through Metrics (Priority: P2)

**Goal**: Expose Prometheus metrics for normal request traffic, latency, and errors through `/metrics` without polluting those metrics with `/metrics` scrapes

**Independent Test**: Exercise normal API endpoints and error paths, fetch `/metrics`, and verify request count, latency, and error metrics are present while `/metrics` requests do not inflate normal request measurements.

### Tests for User Story 2 ⚠️

- [X] T012 [P] [US2] Add integration tests for metrics endpoint availability, request/error metric emission, and `/metrics` exclusion behavior in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`
- [X] T013 [P] [US2] Add unit coverage for metrics route classification and exporter setup helpers in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\observability.rs`

### Implementation for User Story 2

- [X] T014 [US2] Implement Prometheus exporter initialization and metrics rendering helpers in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\observability.rs`
- [X] T015 [US2] Add the `/metrics` HTTP endpoint and route wiring in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\mod.rs`, `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\lib.rs`, and `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\lib.rs`
- [X] T016 [US2] Record request count, latency, and error metrics for normal request flows while excluding `/metrics` traffic in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\observability.rs`
- [X] T017 [US2] Document the `/metrics` endpoint contract and any API-surface changes in `D:\Projects\open-source\nz\rust-microservice-template\specs\004-observability-metrics\contracts\observability-http.yaml` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\api_doc.rs`

**Checkpoint**: User Stories 1 and 2 should now work independently through response tracing and `/metrics`

---

## Phase 5: User Story 3 - Apply Observability Data in Daily Operations (Priority: P3)

**Goal**: Provide operational guidance so developers and operators can retrieve metrics, use request IDs, and answer common traffic, latency, and failure questions

**Independent Test**: Review the repository documentation and quickstart flow to confirm it explains request ID behavior, metrics retrieval, and example operational checks without reading implementation code.

### Tests for User Story 3 ⚠️

- [X] T018 [P] [US3] Add or refine documentation-focused verification for observability API descriptions in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\api_doc.rs`

### Implementation for User Story 3

- [X] T019 [US3] Update operator-facing observability guidance, configuration examples, and sample checks in `D:\Projects\open-source\nz\rust-microservice-template\README.md`
- [X] T020 [US3] Align feature quickstart steps and operational validation notes in `D:\Projects\open-source\nz\rust-microservice-template\specs\004-observability-metrics\quickstart.md`

**Checkpoint**: All user stories should now be independently functional, documented, and testable

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and cleanup across all stories

- [X] T021 [P] Run full verification commands for the feature (`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`, `cargo test --workspace`) from `D:\Projects\open-source\nz\rust-microservice-template`
- [X] T022 Review and finalize observability plan artifacts for implementation parity in `D:\Projects\open-source\nz\rust-microservice-template\specs\004-observability-metrics\plan.md`, `D:\Projects\open-source\nz\rust-microservice-template\specs\004-observability-metrics\research.md`, and `D:\Projects\open-source\nz\rust-microservice-template\specs\004-observability-metrics\data-model.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies, can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational completion
- **User Story 2 (Phase 4)**: Depends on Foundational completion and builds on shared observability wiring from US1
- **User Story 3 (Phase 5)**: Depends on the shipped behavior from US1 and US2 being stable enough to document accurately
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational and is the MVP
- **User Story 2 (P2)**: Depends on the shared middleware and request instrumentation path established for US1
- **User Story 3 (P3)**: Depends on final request ID and metrics behavior from US1 and US2

### Within Each User Story

- Tests should be written and observed failing before implementation
- Shared helper or settings work should land before route- or doc-level changes that depend on it
- Middleware or metrics implementation should precede final integration verification
- Documentation updates should follow stable runtime behavior

### Parallel Opportunities

- T002 can run in parallel with T001
- T004 can run in parallel with T003
- T007 and T008 can run in parallel
- T012 and T013 can run in parallel
- T018 can run in parallel with T019 once runtime behavior is stable
- T021 can run in parallel with T022 at the end

---

## Parallel Example: User Story 1

```text
Task: "Add integration tests for generated request IDs, preserved inbound request IDs, and failed-request correlation in D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs"
Task: "Add unit coverage for observability settings defaults and request ID helper behavior in D:\Projects\open-source\nz\rust-microservice-template\src\application\src\settings.rs and D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\observability.rs"
```

---

## Parallel Example: User Story 2

```text
Task: "Add integration tests for metrics endpoint availability, request/error metric emission, and /metrics exclusion behavior in D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs"
Task: "Add unit coverage for metrics route classification and exporter setup helpers in D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\observability.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Stop and validate request tracing and response request ID behavior independently

### Incremental Delivery

1. Complete Setup + Foundational
2. Deliver User Story 1 as the first usable observability increment
3. Add User Story 2 for Prometheus metrics and `/metrics`
4. Add User Story 3 for operational guidance and documentation parity
5. Finish with full verification and artifact cleanup

### Parallel Team Strategy

1. One developer handles settings and composition-root wiring in Phase 2
2. A second developer prepares US1 and US2 integration coverage in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`
3. Once runtime behavior stabilizes, documentation work for US3 can proceed in parallel with final verification

---

## Notes

- [P] tasks target different files or can be completed without waiting on another unfinished task in the same phase
- [Story] labels map directly to the three user stories in `spec.md`
- The suggested MVP scope is **User Story 1 only**
- The task list assumes observability code stays in composition and HTTP-boundary layers, with no business-layer behavior changes
