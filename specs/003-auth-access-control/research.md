# Research: Authentication and Authorization Support

## Decision 1: User-facing authentication bootstrap

- **Decision**: Add an open `POST /api/v1/auth/token` endpoint that validates configuration-backed user credentials and returns a signed JWT for subsequent bearer-authenticated requests.
- **Rationale**: The feature requires JWT support for user-facing APIs, but the specification explicitly excludes registration and broader user-management workflows. Configuration-backed bootstrap users plus a token issuance endpoint are the smallest design that still gives end users a real authentication path.
- **Alternatives considered**:
  - External identity provider integration: rejected because it introduces a new runtime dependency and operational complexity beyond the template’s current scope.
  - Database-backed users: rejected because it requires schema design, migrations, and account-management workflows that are out of scope for this feature.

## Decision 2: Service credential approach

- **Decision**: Use header-based API keys for service-to-service authentication and fold the existing audit-token access pattern into that shared model.
- **Rationale**: The current repository already has an established header-token flow for audit access, including configuration and integration tests. Formalizing service API keys extends an existing pattern instead of introducing a second machine-auth mechanism.
- **Alternatives considered**:
  - Bearer tokens for service identities: rejected because it blurs the line between human and service authentication paths.
  - Supporting both API keys and bearer tokens for services from the start: rejected because it widens the feature without solving an immediate requirement.

## Decision 3: Authorization model

- **Decision**: Use permission-based authorization for both user and service principals, with roles allowed only as a grouping mechanism in configuration.
- **Rationale**: Endpoint requirements stay explicit and scalable when tied to permissions. This also matches the clarified specification and keeps authorization checks uniform across caller types.
- **Alternatives considered**:
  - Role-only authorization: rejected because it becomes coarse and brittle as more endpoints are protected.
  - Endpoint allowlists without named permissions: rejected because it makes policy reuse and documentation harder.

## Decision 4: Auth identity storage

- **Decision**: Keep auth identities in configuration for this feature rather than introducing a new persistence model.
- **Rationale**: Configuration-backed identities are sufficient for a template starter feature, keep the implementation simple, and avoid schema churn for a capability that is primarily about request protection.
- **Alternatives considered**:
  - PostgreSQL-backed identity tables: rejected because they add migrations and admin workflows that the specification does not require.

## Decision 5: Enforcement location

- **Decision**: Perform credential parsing and HTTP challenge behavior in the presentation layer, then pass a normalized authenticated principal into application code for permission checks.
- **Rationale**: This preserves DDD boundaries, keeps HTTP concerns out of the application layer, and ensures unauthorized and forbidden responses remain transport concerns.
- **Alternatives considered**:
  - Per-handler inline auth checks: rejected because they duplicate logic and increase inconsistency risk.
  - Repository-level authorization: rejected because it mixes access control with persistence concerns.

## Decision 6: Documentation and verification strategy

- **Decision**: Treat OpenAPI security metadata, README auth instructions, and integration tests as first-class deliverables of the feature.
- **Rationale**: Authentication and authorization are public contract behavior. The feature is incomplete unless callers can discover how to authenticate and the runtime verifies both acceptance and rejection paths.
- **Alternatives considered**:
  - Code-only implementation with minimal documentation: rejected because it would violate the constitution’s API parity and testing requirements.
