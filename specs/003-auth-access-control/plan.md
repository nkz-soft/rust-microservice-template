# Implementation Plan: Authentication and Authorization Support

**Branch**: `feature/003-auth-access-control` | **Date**: 2026-03-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/003-auth-access-control/spec.md`

## Summary

Add access control to the HTTP API by protecting business endpoints by default, introducing a small authentication surface for user-facing clients and service-to-service callers, and enforcing permission-based authorization consistently across both identity types. The simplest design that fits the current template is configuration-backed auth identities, an open token issuance endpoint for user JWT acquisition, header-based API keys for service identities, presentation-layer guards that map credentials into an authenticated principal, and OpenAPI/README/test updates that prove unauthenticated, authenticated, and forbidden flows.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1, Validator 0.20, `jsonwebtoken`, `actix-web-httpauth`, `argon2`  
**Storage**: PostgreSQL via Diesel and r2d2 pool for existing to-do data; configuration file and environment variables for bootstrap auth identities and secrets  
**Testing**: `cargo test`, integration tests in `src/starter/tests`, crate-level unit tests, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`  
**Target Platform**: Linux-hosted HTTP service  
**Project Type**: Layered Rust web service  
**Performance Goals**: Keep auth checks on the request path lightweight enough that protected CRUD and list requests remain comparable to current behavior for typical to-do workloads, with credential parsing and permission evaluation adding only bounded in-process overhead  
**Constraints**: Preserve `/api/v1` API stability, keep health and API documentation open, keep auth enforcement in presentation/application boundaries, avoid introducing a new user-management subsystem or database schema for this feature, preserve existing audit behavior by folding it into the new service-credential model, and ensure OpenAPI matches runtime auth requirements  
**Scale/Scope**: One new auth endpoint, protection for existing business endpoints, one configuration-backed auth model for users and services, no new persistence schema, OpenAPI/README updates, and unit plus integration verification for success and failure flows

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- `PASS`: Layer boundaries remain explicit. Credential parsing and HTTP challenge behavior stay in `presentation`, principal and permission modeling stay in `application` and shared domain-facing types, and infrastructure remains limited to configuration loading plus existing PostgreSQL access.
- `PASS`: Public API remains versioned under `/api/v1`; the design adds an auth endpoint and auth requirements without breaking the versioning contract.
- `PASS`: Validation remains at the boundary for headers and request bodies, and error responses will continue to use problem-details semantics.
- `PASS`: The plan avoids unnecessary persistence churn by using configuration-backed auth identities instead of introducing user tables and migrations for this feature.
- `PASS`: Verification spans unit, integration, and contract documentation coverage for unauthenticated, authenticated, and forbidden flows.
- `PASS WITH NOTE`: New security logic increases operational sensitivity. The design therefore requires explicit configuration, secret-safe logging, and OpenAPI documentation parity before implementation is complete.

## Phase 0: Research

### Decisions

1. **User-facing authentication bootstrap**
   - Decision: Use a small open `POST /api/v1/auth/token` endpoint that validates a configured username and Argon2-hashed password, then issues a signed JWT containing subject and permission claims.
   - Rationale: The feature requires JWT support for user-facing APIs but the spec excludes registration and user-management workflows. Configuration-backed bootstrap identities plus a token endpoint are the smallest design that provides end-user JWT access without a new persistence model.
   - Alternatives considered:
     - External identity provider integration: rejected because it adds a new external dependency and operational surface not implied by the current template.
     - Database-backed users: rejected because it requires schema design, migrations, and management workflows beyond the requested feature scope.

2. **Service-to-service authentication path**
   - Decision: Reuse the repository’s existing header-token pattern by formalizing service API keys in request headers, including the current audit access path.
   - Rationale: The current codebase already has `X-Audit-Token` parsing, tests, configuration, and OpenAPI metadata for a service-style credential. Extending that pattern is the least disruptive path and aligns with the clarification decision.
   - Alternatives considered:
     - Bearer tokens for services: rejected because it collapses user and machine flows into one path and complicates documentation and policy separation.
     - Supporting both API keys and bearer tokens for services immediately: rejected because it expands the feature surface without adding essential value.

3. **Authorization model**
   - Decision: Make endpoint access decisions permission-based for both user and service identities, while allowing roles only as a configuration-time grouping mechanism.
   - Rationale: Permission-based checks align with the clarified spec, keep endpoint policy explicit, and avoid over-coupling behavior to a small fixed role set.
   - Alternatives considered:
     - Role-only authorization: rejected because it is coarser and less stable as endpoint coverage grows.
     - Endpoint-specific allowlists only: rejected because they do not scale cleanly across both user and service identities.

4. **Enforcement location**
   - Decision: Enforce credential parsing and auth challenges in the presentation layer, then pass an authenticated principal into application handlers where endpoint-specific permission requirements can be checked before business execution.
   - Rationale: This matches the repository’s DDD constitution, keeps HTTP-specific semantics out of application handlers, and preserves a clear boundary between transport errors and business logic.
   - Alternatives considered:
     - Embedding auth logic in each endpoint handler: rejected because it duplicates validation and weakens consistency.
     - Moving auth checks into infrastructure or repositories: rejected because authorization is not a persistence concern.

5. **Protected surface rollout**
   - Decision: Protect all business endpoints under `/api/v1/to-do-items` and `/api/v1/audit/to-do-items`, keep `/api/v1/healthz/*` and documentation routes open, and treat `/api/v1/auth/token` as the dedicated open authentication bootstrap route.
   - Rationale: This preserves the clarified default-protection rule while still allowing clients to obtain tokens without preexisting authentication.
   - Alternatives considered:
     - Protecting only mutating endpoints: rejected because it would leave the service partially open and fail the core access-control goal.

## Phase 1: Design & Contracts

### Data Model Changes

- No database schema changes are required for this feature.
- Add configuration-backed auth models for JWT signing settings, bootstrap user credentials, and service API keys.
- Add an authenticated principal model shared across protected request handling that carries subject identifier, principal type, and granted permissions.
- Add a token claims model for issued JWTs with stable claim names for subject, expiry, issued-at, and permissions.

### Application Design

- Introduce explicit auth-facing DTOs and services in `src/application/src` for login input, token output, permission modeling, authenticated principals, and token issuance orchestration.
- Keep repository contracts unchanged for to-do persistence; auth identity storage remains configuration-backed in this feature.
- Define permission requirements per use case so endpoint handlers can map routes to required permissions without embedding raw strings everywhere.
- Align existing audit access behavior with the service-principal model instead of maintaining it as a separate bespoke check.

### Infrastructure Design

- Extend settings loading in `src/application/src/settings.rs` to read auth signing config, bootstrap users, and service API keys from `config.app.toml` and environment overrides.
- Implement any required adapter modules for password hash verification, JWT signing and verification, and configuration-backed principal lookup.
- Avoid Diesel migrations or new repository implementations unless implementation reveals an unavoidable gap; the planned design assumes config-backed auth only.

### Presentation Design

- Add `POST /api/v1/auth/token` as an open route that accepts user credentials and returns a bearer token response.
- Protect business endpoints by default through reusable auth extractors or middleware for bearer token validation, API key header validation, `401 Unauthorized` mapping, and `403 Forbidden` mapping.
- Replace the bespoke audit-token check with the same service API key flow and permission policy used elsewhere.
- Update OpenAPI to describe bearer auth, API key auth, route-specific security requirements, and token issuance payloads.

### Documentation Design

- Update README configuration examples to show bootstrap auth settings, service API keys, and the token acquisition flow.
- Document which endpoints are open versus protected and which credential type each protected route accepts.
- Keep user-facing docs focused on request behavior and credential requirements, not implementation details such as claim-signing internals.

## Phase 2: Implementation Plan

1. **Establish configuration-backed auth models**
   - Extend settings structs and tests for auth signing config, bootstrap users, and service API keys.
   - Add sample configuration entries that keep local development usable.
   - Preserve the current environment override pattern.

2. **Implement auth domain/application flow**
   - Add login request/response DTOs, authenticated principal types, and permission modeling.
   - Implement password verification and JWT issuance services for user-facing login.
   - Implement service API key lookup and principal construction.

3. **Wire presentation-layer enforcement**
   - Add the open token issuance endpoint.
   - Introduce reusable auth guards or extractors for bearer tokens and service API keys.
   - Apply permission checks to existing to-do and audit endpoints with route-appropriate policies.
   - Convert audit access from bespoke token checking to the shared service-auth path.

4. **Update API contracts and docs**
   - Add OpenAPI security schemes and token endpoint documentation.
   - Document protected-route requirements in README and request examples.
   - Ensure problem-details responses describe unauthorized and forbidden flows consistently.

5. **Harden verification**
   - Add unit tests for settings parsing, password verification orchestration, JWT claim validation, API key parsing, and permission mapping.
   - Add integration tests for open routes, unauthorized responses, forbidden responses, successful bearer-auth requests, successful service-key requests, and audit access through the shared service-auth path.
   - Re-run formatting, clippy, and workspace tests.

## Project Structure

### Documentation (this feature)

```text
specs/
└── 003-auth-access-control/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/
    │   └── auth-protected-api.yaml
    └── checklists/
        └── requirements.md
```

### Source Code (repository root)

```text
src/
├── application/
│   └── src/
│       ├── dtos.rs
│       ├── handlers.rs
│       ├── queries.rs
│       ├── services.rs
│       └── settings.rs
├── presentation/
│   └── src/
│       ├── api/
│       │   ├── api_doc.rs
│       │   └── app.rs
│       ├── config.rs
│       ├── errors.rs
│       ├── requests.rs
│       └── responses.rs
├── starter/
│   ├── src/
│   │   └── lib.rs
│   └── tests/
│       └── integration.rs
└── infrastructure/
    └── src/
        └── lib.rs
```

**Structure Decision**: Keep the existing layered workspace and implement access control through narrow additions to current application, presentation, and starter wiring. No new crate and no new persistence schema are required for the planned feature.

## Test Strategy

- Unit tests:
  - settings parsing covers auth signing keys, bootstrap users, and service API keys
  - login request validation rejects blank credentials
  - JWT verification rejects malformed, expired, or wrongly signed tokens
  - API key parsing trims values and rejects missing or blank required headers
  - permission checks distinguish authenticated-but-forbidden from unauthenticated flows
- Integration tests:
  - `POST /api/v1/auth/token` issues a token for valid bootstrap credentials
  - protected to-do endpoints reject missing or invalid bearer tokens
  - protected to-do endpoints allow valid bearer tokens with required permissions
  - protected endpoints return `403 Forbidden` for principals lacking required permissions
  - service-authenticated audit access succeeds with the correct API key and fails otherwise
  - health and documentation endpoints remain open without credentials
- Verification commands:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets --all-features`
  - `cargo test --workspace`

## Risks and Mitigations

- **Risk**: Auth configuration becomes difficult to operate or easy to misconfigure.
  - Mitigation: Keep configuration shape explicit, add settings tests, and document minimal local-development examples in README and quickstart.
- **Risk**: Duplicated auth logic emerges across routes, leading to inconsistent `401` and `403` behavior.
  - Mitigation: Centralize credential parsing and authorization checks in reusable presentation-layer guards or extractors.
- **Risk**: Introducing JWT support leaks secrets or implementation detail through logs and error payloads.
  - Mitigation: Restrict error responses to problem-details summaries and keep logs structured and secret-free.
- **Risk**: Shared auth rollout breaks the existing audit endpoint semantics.
  - Mitigation: Add regression integration tests around audit access and treat audit as the first service-authenticated route migrated to the common path.

## Complexity Tracking

No constitution violations currently require exception handling.
