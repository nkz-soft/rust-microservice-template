# Tasks: Search Support for To-Do Items

**Input**: Design documents from `/specs/002-todo-search/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: This feature explicitly requires automated verification for matching, non-matching, and unchanged default-list behavior.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm the current search scaffold and establish the contract artifacts that implementation will target

- [X] T001 Confirm current search behavior for title, note, blank-input, and no-search requests in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`
- [X] T002 [P] Validate the documented search contract against `D:\Projects\open-source\nz\rust-microservice-template\specs\002-todo-search\contracts\todo-search-list.yaml` and `D:\Projects\open-source\nz\rust-microservice-template\specs\002-todo-search\quickstart.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Ensure the shared list-query boundary is ready before story-specific verification and documentation work

**⚠️ CRITICAL**: No user story work should begin until this phase is complete

- [X] T003 Verify and, if needed, tighten search normalization and blank-input handling in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`
- [X] T004 Verify and, if needed, tighten shared search filtering and count-query consistency in `D:\Projects\open-source\nz\rust-microservice-template\src\infrastructure\src\postgres_repositories.rs`

**Checkpoint**: Search boundary and repository filtering are ready for story-level verification

---

## Phase 3: User Story 1 - Find Relevant To-Do Items by Search Term (Priority: P1) 🎯 MVP

**Goal**: Deliver correct matching behavior for title and note searches while excluding non-matching and deleted items

**Independent Test**: Create items with matching titles, matching notes, and non-matching content; query `GET /api/v1/to-do-items?search=...`; verify only matching active items are returned.

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T005 [P] [US1] Add integration tests for title matches, note matches, and exclusion of known non-matching items in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`
- [X] T006 [P] [US1] Add unit coverage for normalized search mapping and blank-search rejection in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`

### Implementation for User Story 1

- [X] T007 [US1] Implement or correct active-item search filtering across title and note content in `D:\Projects\open-source\nz\rust-microservice-template\src\infrastructure\src\postgres_repositories.rs`
- [X] T008 [US1] Verify list query mapping preserves the normalized search term from request to application query in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`
- [X] T009 [US1] Verify the list endpoint returns searched results through the standard flow in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs`

**Checkpoint**: User Story 1 should now be independently functional and testable as the MVP

---

## Phase 4: User Story 2 - Combine Search with Standard Listing Workflow (Priority: P2)

**Goal**: Preserve normal list behavior without search and ensure search works predictably with pagination and sorting

**Independent Test**: Query the list endpoint with and without `search`, and with `search` plus `page`, `page_size`, and `sort`; verify the endpoint preserves standard listing semantics and stable metadata.

### Tests for User Story 2 ⚠️

- [X] T010 [P] [US2] Add integration tests proving omitted `search` preserves existing list behavior and `search` works correctly with `page`, `page_size`, and `sort` in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`
- [X] T011 [P] [US2] Add unit coverage for search-plus-sort query mapping in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`

### Implementation for User Story 2

- [X] T012 [US2] Ensure paginated list metadata remains consistent between filtered item queries and filtered count queries in `D:\Projects\open-source\nz\rust-microservice-template\src\infrastructure\src\postgres_repositories.rs`
- [X] T013 [US2] Ensure default list behavior remains unchanged when `search` is omitted in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`
- [X] T014 [US2] Ensure deleted items remain excluded from searched list results in `D:\Projects\open-source\nz\rust-microservice-template\src\infrastructure\src\postgres_repositories.rs`

**Checkpoint**: User Stories 1 and 2 should now both work independently through the standard list endpoint

---

## Phase 5: User Story 3 - Understand Search Behavior from API Documentation (Priority: P3)

**Goal**: Document the shipped search contract clearly in OpenAPI and repository-facing docs

**Independent Test**: Review the generated OpenAPI descriptions and README examples to confirm they describe the `search` parameter, searchable fields, and invalid blank-input behavior.

### Tests for User Story 3 ⚠️

- [X] T015 [P] [US3] Add or refine API contract verification for list-query parameter documentation in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`
- [X] T016 [US3] Verify generated OpenAPI output for `GET /api/v1/to-do-items` documents the `search` parameter, searchable fields, and blank-search validation behavior using `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\api_doc.rs`

### Implementation for User Story 3

- [X] T017 [US3] Update search parameter descriptions, including blank-search rejection semantics, in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`
- [X] T018 [US3] Update list endpoint OpenAPI wording to document searchable fields and `400 Bad Request` behavior for blank search input in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs`
- [X] T019 [US3] Align README search examples and document that blank `search` values are rejected with `400 Bad Request` in `D:\Projects\open-source\nz\rust-microservice-template\README.md`

**Checkpoint**: All user stories should now be independently functional, documented, and testable

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and cleanup across all stories

- [X] T020 [P] Run full verification commands for the feature (`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`, `cargo test --workspace`) from `D:\Projects\open-source\nz\rust-microservice-template`
- [X] T021 Update `D:\Projects\open-source\nz\rust-microservice-template\specs\002-todo-search\quickstart.md` if final implementation details or validation steps changed during execution

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - blocks all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational completion
- **User Story 2 (Phase 4)**: Depends on Foundational completion and should build on verified US1 list-search behavior
- **User Story 3 (Phase 5)**: Depends on the effective behavior from US1 and US2 being settled
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - no dependency on later stories
- **User Story 2 (P2)**: Depends on the shared list-search implementation path validated by US1
- **User Story 3 (P3)**: Depends on final behavior from US1 and US2 so the documentation reflects shipped semantics

### Within Each User Story

- Tests should be written and observed failing before implementation
- Request/query validation changes should precede endpoint or documentation adjustments when both are needed
- Repository filtering fixes should precede final integration verification
- Documentation updates should happen after behavior is stable

### Parallel Opportunities

- T002 can run in parallel with T001
- T005 and T006 can run in parallel
- T010 and T011 can run in parallel
- T015 can run in parallel with behavior verification work once the endpoint contract is stable
- T019 can run independently after implementation is complete

---

## Parallel Example: User Story 1

```text
Task: "Add integration coverage for title-match, note-match, and non-match search scenarios in D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs"
Task: "Add unit coverage for normalized search mapping and blank-search rejection in D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs"
```

---

## Parallel Example: User Story 2

```text
Task: "Add integration coverage for omitted-search behavior and search combined with pagination and sorting in D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs"
Task: "Add unit coverage for search-plus-sort query mapping in D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Stop and validate search by title/note independently

### Incremental Delivery

1. Complete Setup + Foundational
2. Deliver User Story 1 as the MVP search behavior
3. Add User Story 2 to preserve full list-workflow compatibility
4. Add User Story 3 to finalize OpenAPI and README parity
5. Finish with full verification and quickstart validation

### Parallel Team Strategy

1. One developer verifies and fixes shared request/repository behavior in Phase 2
2. A second developer prepares integration coverage for US1/US2 in `src/starter/tests/integration.rs`
3. Once behavior stabilizes, documentation work for US3 can proceed in parallel with final verification

---

## Notes

- [P] tasks target different files or can be completed without waiting on another unfinished task in the same phase
- [Story] labels map directly to the three user stories in `spec.md`
- The task list assumes minimal code changes because much of the search scaffolding already exists
- The suggested MVP scope is **User Story 1 only**
