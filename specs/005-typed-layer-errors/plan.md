# Implementation Plan: Explicit Error Types by Application Layer

**Branch**: `feature/005-typed-layer-errors` | **Date**: 2026-03-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/005-typed-layer-errors/spec.md`

## Summary

Replace cross-layer `anyhow` usage in normal request flow with explicit error contracts aligned to the repository's DDD boundaries. The implementation introduces application-level typed errors, narrows infrastructure failures behind repository translations, updates presentation-layer problem-details mapping to consume stable application errors, keeps `anyhow` limited to outer process and startup boundaries, and verifies the new behavior through unit and integration tests.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Anyhow 1, Thiserror 2, `problem_details`, `uuid`, `async-trait`  
**Storage**: PostgreSQL via Diesel and r2d2 pool for business data  
**Testing**: `cargo test`, crate-level unit tests, HTTP integration tests in `src/starter/tests`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`  
**Target Platform**: Linux-hosted HTTP service  
**Project Type**: Layered Rust web service  
**Performance Goals**: Preserve existing request-path performance and response timing by limiting the change to error typing and mapping logic rather than adding new I/O or business processing  
**Constraints**: Maintain the fixed dependency direction across `domain`, `application`, `infrastructure`, `presentation`, and `starter`; keep `/api/v1` behavior stable on success paths; expose consistent problem-details responses without leaking internal error details; avoid schema changes; keep `anyhow` only at startup and outer composition boundaries  
**Scale/Scope**: One existing service, one repository contract family, several query handlers, HTTP error translation in `presentation`, OpenAPI/README updates for stable failure responses, and regression coverage for representative failure mappings

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- `PASS`: The plan reinforces the required DDD boundaries by moving error ownership to the appropriate layer instead of allowing lower-level failures to bleed upward.
- `PASS`: The presentation layer remains the sole owner of HTTP problem-details translation, preserving the API-contract rule in the constitution.
- `PASS`: No persistence schema or migration changes are required; repository contracts remain in `application` and implementations remain in `infrastructure`.
- `PASS`: The feature improves the constitution's explicit typed-error requirement and reduces accidental internal detail exposure to clients.
- `PASS`: The verification strategy covers unit, integration, and contract-level behavior for error responses, which matches the repository testing standard for public behavior changes.
- `PASS WITH NOTE`: The feature may add new error modules and conversion glue, but that complexity is directly tied to making architectural boundaries explicit rather than introducing speculative abstractions.

## Phase 0: Research

### Decisions

1. **Application owns the stable error contract**
   - Decision: Introduce explicit application-level error enums that model the failures the rest of the service is allowed to depend on.
   - Rationale: Repository traits and handlers currently return `anyhow::Result`, which prevents callers from distinguishing business, validation, and dependency failures in a stable way.
   - Alternatives considered:
     - Keep `anyhow` in application and rely on downcasting in presentation: rejected because it keeps the cross-layer leak and makes behavior fragile.
     - Expose infrastructure errors directly to presentation: rejected because it violates dependency direction and leaks implementation concerns into the transport layer.

2. **Infrastructure translates dependency failures, not HTTP behavior**
   - Decision: Keep infrastructure-specific error types inside `infrastructure`, but translate them into application errors at the repository boundary.
   - Rationale: Database and adapter failures must remain implementation details while still giving the application layer enough information to choose a stable outward category.
   - Alternatives considered:
     - Collapse all failures into one generic repository error: rejected because it loses useful distinctions such as not-found vs. concurrency conflict.
     - Move database-specific error typing into `application`: rejected because it couples use-case contracts to persistence details.

3. **Presentation maps only application errors to problem details**
   - Decision: Replace `From<anyhow::Error>` HTTP mapping with explicit conversion from application errors to `HttpError` problem-details responses.
   - Rationale: Stable API behavior requires the transport layer to depend on a stable service contract rather than on runtime downcasts against lower-layer errors.
   - Alternatives considered:
     - Continue downcasting boxed or generic errors in presentation: rejected because it is brittle and easy to regress when new failures are introduced.
     - Let handlers return `HttpError` directly: rejected because it would leak transport semantics into the application layer.

4. **Reserve `anyhow` for process boundaries**
   - Decision: Keep `anyhow` in `starter` and startup/configuration code where outer-boundary aggregation is appropriate, but remove it from normal application and presentation request flow.
   - Rationale: The constitution explicitly permits `anyhow` at composition boundaries while requiring typed errors in reusable and domain-facing paths.
   - Alternatives considered:
     - Remove `anyhow` everywhere immediately: rejected because startup wiring and process bootstrapping still benefit from aggregated outer-boundary context.
     - Leave `anyhow` in repository traits and handlers: rejected because it undermines the goal of explicit cross-layer contracts.

5. **Lock the outward contract through tests and docs**
   - Decision: Add targeted unit and integration coverage for representative error translations and update API-facing documentation to describe stable failure outcomes.
   - Rationale: Error handling regressions are easy to miss without explicit tests, and stable client behavior is part of the feature definition.
   - Alternatives considered:
     - Rely on code review alone: rejected because subtle mapping regressions can slip through.
     - Test only internal enum conversions: rejected because the client-visible HTTP contract also needs direct verification.

## Phase 1: Design & Contracts

### Data Model Changes

- No persistent schema changes are required.
- Add non-persistent error models that define boundary contracts:
  - application error categories for repository and handler results
  - infrastructure error categories for persistence and adapter failures
  - presentation-level HTTP problem response mapping rules derived from application errors
- Add or refine documentation-only error contract artifacts describing stable client-visible failure categories and HTTP outcomes.

### Domain Design

- No domain-entity or invariant changes are planned.
- Domain types remain free of HTTP, persistence, and generic error-aggregation concerns.

### Application Design

- Introduce one or more application error enums under `src/application/src/` to represent stable service-level failures such as:
  - entity not found
  - optimistic concurrency conflict
  - invalid request precondition or business validation failure where applicable
  - infrastructure unavailable or unexpected internal dependency failure
- Update `ToDoItemRepository` to return typed application results instead of `anyhow::Result`.
- Update query handlers and `ToDoItemService` helpers to propagate typed application errors without converting to transport-specific types.
- Re-export the new error types from `application::lib` so `presentation` can depend on them directly.

### Infrastructure Design

- Keep infrastructure-specific failures in `src/infrastructure/src/errors.rs`, but sharpen them around repository implementation needs.
- Update `PostgresToDoItemRepository` to:
  - stop returning `anyhow::Result` for normal repository methods
  - map Diesel and pool failures into infrastructure error variants
  - translate or expose repository results in a way that satisfies the typed application contract
- Keep database migration and startup configuration code unchanged except where startup-only `anyhow` use remains appropriate.

### Presentation Design

- Refactor `src/presentation/src/errors.rs` so `HttpError` converts from application error types rather than `anyhow::Error`.
- Define stable mapping rules from application failure categories to HTTP problem details, including:
  - `404` for missing resources
  - `412` for optimistic concurrency conflicts
  - `500` for internal dependency or unexpected failures
  - existing `400`, `401`, and `428` transport validation/precondition errors remain owned by presentation
- Ensure client-visible problem details contain stable titles and details without leaking database messages, stack traces, or internal driver text.
- Update OpenAPI annotations in HTTP handlers if failure descriptions or response coverage need to become more explicit.

### Documentation Design

- Update README or equivalent contributor-facing documentation with the new error-handling boundary rules:
  - typed errors in normal layer interactions
  - `anyhow` limited to process boundaries
  - stable HTTP problem-details outcomes for representative failures
- Keep documentation focused on contracts and maintenance guidance, not internal implementation trivia.

## Phase 2: Implementation Plan

1. **Define application error contracts**
   - Add application-level error modules and result aliases where useful.
   - Rework repository traits and query handlers to use typed errors instead of `anyhow`.
   - Re-export the new error types for downstream layers.

2. **Refactor infrastructure repository failures**
   - Update repository implementations to return typed errors.
   - Preserve distinctions for not-found, version conflict, and unexpected storage failures.
   - Remove normal-path `anyhow` usage from repository methods.

3. **Refactor presentation HTTP mapping**
   - Replace `anyhow` downcasting in `HttpError` with direct application-error mapping.
   - Ensure stable problem-details output and preserve existing validation/auth/precondition handling.
   - Align OpenAPI error descriptions with the new mapping rules.

4. **Document and harden the contract**
   - Add or update README guidance and a contract artifact for representative HTTP failure behavior.
   - Keep wording aligned with the spec and avoid implementation leakage.

5. **Verify behavior**
   - Add unit tests for application-to-HTTP mapping and any conversion helpers.
   - Add integration tests for representative `404`, `412`, and generic internal failure cases.
   - Run formatting, clippy, and workspace tests.

## Project Structure

### Documentation (this feature)

```text
specs/
└── 005-typed-layer-errors/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/
    │   └── http-error-contract.yaml
    └── checklists/
        └── requirements.md
```

### Source Code (repository root)

```text
src/
├── application/
│   └── src/
│       ├── handlers.rs
│       ├── lib.rs
│       ├── repositories.rs
│       ├── services.rs
│       └── errors.rs
├── infrastructure/
│   └── src/
│       ├── errors.rs
│       ├── lib.rs
│       └── postgres_repositories.rs
├── presentation/
│   └── src/
│       ├── api/
│       │   └── app.rs
│       ├── errors.rs
│       ├── lib.rs
│       └── responses.rs
└── starter/
    ├── src/
    │   └── lib.rs
    └── tests/
        ├── integration.rs
        └── memory_management_tests.rs
```

**Structure Decision**: Keep the existing layered workspace and add narrowly scoped error modules and conversions at each existing boundary. No new crates or architectural layers are required.

## Test Strategy

- Unit tests:
  - application error conversion and propagation helpers
  - presentation `HttpError` mapping for representative application failures
  - infrastructure error translation where behavior is isolated enough to verify without full HTTP flow
- Integration tests:
  - `GET /api/v1/to-do-items/{id}` returns `404` problem details for missing items
  - `PUT /api/v1/to-do-items/{id}` returns `412` problem details for version conflicts
  - a representative unexpected repository failure returns a sanitized `500` problem-details response
  - existing success-path endpoint behavior remains unchanged after the error refactor
- Documentation and contract verification:
  - OpenAPI annotations remain aligned with the actual runtime responses
  - README and `contracts/http-error-contract.yaml` reflect the shipped outward error contract
- Verification commands:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets --all-features`
  - `cargo test --workspace`

## Risks and Mitigations

- **Risk**: Error enums could drift between layers and recreate ambiguity under different names.
  - Mitigation: Keep one stable application-facing contract and require explicit conversions at each boundary.
- **Risk**: Sanitizing internal failures could accidentally erase distinctions clients currently rely on.
  - Mitigation: Preserve documented `404` and `412` semantics, verify mappings with integration tests, and treat all other unexpected failures as intentionally generic `500` responses.
- **Risk**: Refactoring repository trait signatures can affect many tests and service constructors.
  - Mitigation: Change the trait once, update call sites systematically, and use compiler guidance to close all typed-error gaps before final verification.
- **Risk**: Existing tests that fabricate `anyhow` errors may become invalid or too generic.
  - Mitigation: Update test doubles to emit the new explicit application error categories so tests verify the intended contracts rather than generic error plumbing.

## Complexity Tracking

No constitution violations currently require exception handling.
