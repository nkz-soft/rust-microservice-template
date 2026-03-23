# Tasks: Authentication and Authorization Support

**Input**: Design documents from `/specs/003-auth-access-control/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: This feature explicitly requires automated verification for unauthenticated, authenticated, and forbidden flows on protected endpoints.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare dependencies, feature configuration examples, and contract artifacts that the implementation will follow

- [X] T001 Add auth dependencies and any required feature flags in `D:\Projects\open-source\nz\rust-microservice-template\Cargo.toml` and crate `Cargo.toml` files under `D:\Projects\open-source\nz\rust-microservice-template\src\`
- [X] T002 [P] Add baseline auth configuration examples for local development in `D:\Projects\open-source\nz\rust-microservice-template\config.app.toml`
- [X] T003 [P] Verify the planned API surface against `D:\Projects\open-source\nz\rust-microservice-template\specs\003-auth-access-control\contracts\auth-protected-api.yaml` and `D:\Projects\open-source\nz\rust-microservice-template\specs\003-auth-access-control\quickstart.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared auth configuration, principal model, and reusable enforcement path required by all stories

**⚠️ CRITICAL**: No user story work should begin until this phase is complete

- [X] T004 Extend auth settings models and environment loading for JWT config, bootstrap users, and service API keys in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\settings.rs`
- [X] T005 [P] Add settings coverage for auth configuration parsing and overrides in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\settings.rs`
- [X] T006 [P] Add shared auth request/response, claims, principal, and permission types in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\dtos.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\queries.rs`
- [X] T007 Implement shared auth services for password verification, JWT issue/verify, and service API key lookup in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs`
- [X] T008 [P] Add reusable header parsing and auth request validation helpers in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`
- [X] T009 Implement shared unauthorized/forbidden error mapping for auth flows in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\errors.rs`
- [X] T010 Implement reusable bearer-token and service-api-key enforcement helpers in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\config.rs`
- [X] T011 Wire auth settings and shared auth services into application startup in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\lib.rs`

**Checkpoint**: Shared auth plumbing is ready for story-level endpoint work

---

## Phase 3: User Story 1 - Access Protected Endpoints as an Authenticated User (Priority: P1) 🎯 MVP

**Goal**: Allow user-facing clients to obtain a JWT and access protected business endpoints with valid bearer authentication

**Independent Test**: Request a token from `POST /api/v1/auth/token`, call a protected `/api/v1/to-do-items` endpoint without credentials, with invalid credentials, and with a valid bearer token, and verify only the valid authenticated request succeeds.

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T012 [P] [US1] Add unit tests for login request validation, password verification orchestration, and JWT claim validation in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`
- [X] T013 [P] [US1] Add integration tests for token issuance plus unauthorized and authorized user access to `/api/v1/to-do-items` in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`

### Implementation for User Story 1

- [X] T014 [US1] Implement the login command/query flow and bearer token response mapping in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\handlers.rs`, `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs`, and `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\dtos.rs`
- [X] T015 [US1] Add `POST /api/v1/auth/token` request/response handling in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs`, `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`, and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\responses.rs`
- [X] T016 [US1] Protect `/api/v1/to-do-items` user-facing endpoints with bearer-token authentication in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs`
- [X] T017 [US1] Keep health and documentation routes open while wiring the auth token endpoint into routing in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\config.rs`

**Checkpoint**: User Story 1 should now be independently functional and testable as the MVP

---

## Phase 4: User Story 2 - Access Protected Endpoints from Another Service (Priority: P2)

**Goal**: Allow service callers to authenticate with header-based API keys and access service-protected endpoints such as audit reads

**Independent Test**: Call the audit endpoint without a service credential, with an invalid API key, and with a valid API key; verify only the valid service credential succeeds.

### Tests for User Story 2 ⚠️

- [X] T018 [P] [US2] Add unit tests for service API key header parsing and service principal lookup in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs`
- [X] T019 [P] [US2] Add integration tests for unauthorized and authorized service-key access to `/api/v1/audit/to-do-items/{id}` in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`

### Implementation for User Story 2

- [X] T020 [US2] Implement service API key principal resolution and service-auth permission checks in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs`
- [X] T021 [US2] Replace bespoke audit token handling with the shared service API key auth flow in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs`
- [X] T022 [US2] Align audit and startup configuration with service API key auth in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\settings.rs`, `D:\Projects\open-source\nz\rust-microservice-template\src\starter\src\lib.rs`, and `D:\Projects\open-source\nz\rust-microservice-template\config.app.toml`

**Checkpoint**: User Stories 1 and 2 should now both work independently for human and service callers

---

## Phase 5: User Story 3 - Enforce Authorization Rules on Protected Resources (Priority: P3)

**Goal**: Enforce permission-based authorization and return `403 Forbidden` when authenticated callers lack the required permission

**Independent Test**: Call the same protected endpoint with authenticated principals that do and do not have the required permission, and verify allowed versus forbidden behavior plus matching documentation.

### Tests for User Story 3 ⚠️

- [X] T023 [P] [US3] Add unit tests for permission-policy evaluation and forbidden-path behavior in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\errors.rs`
- [X] T024 [P] [US3] Add integration tests proving `403 Forbidden` for insufficient user and service permissions on protected endpoints in `D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs`

### Implementation for User Story 3

- [X] T025 [US3] Define route-level permission policies for to-do and audit use cases in `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\queries.rs`, `D:\Projects\open-source\nz\rust-microservice-template\src\application\src\dtos.rs`, and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs`
- [X] T026 [US3] Apply permission checks to protected endpoint handlers for both user and service principals in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs`
- [X] T027 [US3] Update OpenAPI security schemes, protected-route requirements, and token endpoint documentation in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\api_doc.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\api\app.rs`
- [X] T028 [US3] Update auth request/response schema metadata and security-header documentation in `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs` and `D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\responses.rs`

**Checkpoint**: All user stories should now be independently functional, protected, and permission-aware

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final documentation, verification, and feature hardening across all stories

- [X] T029 [P] Update repository-facing auth documentation and examples in `D:\Projects\open-source\nz\rust-microservice-template\README.md`
- [X] T030 [P] Align the generated feature walkthrough with final implementation details in `D:\Projects\open-source\nz\rust-microservice-template\specs\003-auth-access-control\quickstart.md`
- [X] T031 Run full verification commands for the feature (`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`, `cargo test --workspace`) from `D:\Projects\open-source\nz\rust-microservice-template`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - blocks all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational completion
- **User Story 2 (Phase 4)**: Depends on Foundational completion and reuses shared auth plumbing from US1
- **User Story 3 (Phase 5)**: Depends on Foundational completion and should build on the implemented user and service authentication paths
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - no dependency on later stories
- **User Story 2 (P2)**: Can start after Foundational - independent of US1 business behavior, but shares the common auth framework
- **User Story 3 (P3)**: Depends on the user and service authentication paths being in place so permission enforcement can be applied consistently

### Within Each User Story

- Tests should be written and observed failing before implementation
- Request/response and principal models should be in place before endpoint wiring
- Shared auth helpers should be reused rather than duplicated in handlers
- Documentation updates should follow stabilized runtime behavior

### Parallel Opportunities

- T002 and T003 can run in parallel with T001
- T005, T006, and T008 can run in parallel after T004 starts the shared auth model
- T012 and T013 can run in parallel
- T018 and T019 can run in parallel
- T023 and T024 can run in parallel
- T029 and T030 can run in parallel after implementation is complete

---

## Parallel Example: User Story 1

```text
Task: "Add unit tests for login request validation, password verification orchestration, and JWT claim validation in D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs and D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs"
Task: "Add integration tests for token issuance plus unauthorized and authorized user access to D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs"
```

---

## Parallel Example: User Story 2

```text
Task: "Add unit tests for service API key header parsing and service principal lookup in D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\requests.rs and D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs"
Task: "Add integration tests for unauthorized and authorized service-key access to D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs"
```

---

## Parallel Example: User Story 3

```text
Task: "Add unit tests for permission-policy evaluation and forbidden-path behavior in D:\Projects\open-source\nz\rust-microservice-template\src\application\src\services.rs and D:\Projects\open-source\nz\rust-microservice-template\src\presentation\src\errors.rs"
Task: "Add integration tests proving 403 Forbidden for insufficient user and service permissions in D:\Projects\open-source\nz\rust-microservice-template\src\starter\tests\integration.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Stop and validate token issuance plus bearer-protected business endpoints independently

### Incremental Delivery

1. Complete Setup + Foundational
2. Deliver User Story 1 for end-user authentication on protected business routes
3. Add User Story 2 for service-to-service authenticated access
4. Add User Story 3 for permission enforcement and final API contract parity
5. Finish with documentation alignment and full verification

### Parallel Team Strategy

1. One developer completes shared settings, principal models, and auth service plumbing in Phase 2
2. A second developer prepares US1 integration and unit coverage while shared plumbing stabilizes
3. After the common auth framework is ready, service-auth work (US2) and permission-policy work (US3) can split across different developers

---

## Notes

- [P] tasks target different files or can be completed without waiting on another unfinished task in the same phase
- [Story] labels map directly to the three user stories in `spec.md`
- The suggested MVP scope is **User Story 1 only**
- No database migration tasks are included because the approved plan keeps auth identities configuration-backed for this feature
