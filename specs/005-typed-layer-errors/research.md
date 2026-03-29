# Research: Explicit Error Types by Application Layer

## Decision 1: Put the stable error contract in `application`

- **Decision**: Define explicit application-level error enums as the only error contract used across normal service-layer boundaries.
- **Rationale**: `application` is the boundary shared by infrastructure and presentation, so it is the correct place to express stable service failures without leaking persistence or transport details.
- **Alternatives considered**:
  - Keep `anyhow` in repository and handler signatures: rejected because it preserves opaque cross-layer flow.
  - Let `presentation` depend on infrastructure errors: rejected because it violates the repository's fixed dependency direction.

## Decision 2: Keep infrastructure errors implementation-specific

- **Decision**: Maintain infrastructure-specific error variants for repository implementation concerns, but convert them before those failures become part of a higher-layer public contract.
- **Rationale**: Persistence adapters need detail for diagnosis and internal branching, but those details are not part of the service contract seen by application or HTTP clients.
- **Alternatives considered**:
  - Expose Diesel or adapter-specific failures directly: rejected because it leaks implementation details.
  - Collapse all storage failures into a single opaque internal error immediately: rejected because repository code still needs distinctions like not-found and version conflict.

## Decision 3: Map HTTP problem details from application errors, not runtime downcasts

- **Decision**: Replace `From<anyhow::Error>` in `presentation::HttpError` with direct conversions from application-defined error categories.
- **Rationale**: Downcasting generic errors is brittle and encourages hidden coupling to lower layers. Direct mapping from application errors makes the outward contract explicit and compiler-checked.
- **Alternatives considered**:
  - Continue using `anyhow` plus downcasts: rejected because the issue explicitly targets cross-layer generic error flow.
  - Return `HttpError` from handlers: rejected because it would leak transport concerns into the application layer.

## Decision 4: Keep `anyhow` only at outer process boundaries

- **Decision**: Retain `anyhow` in startup, configuration, and composition-root code where aggregated context is useful, while removing it from repository, handler, and presentation request flow.
- **Rationale**: This matches the constitution's explicit guidance and limits generic error aggregation to the narrow place where it is justified.
- **Alternatives considered**:
  - Remove `anyhow` from every crate in one pass: rejected because startup and outer-boundary orchestration still benefit from contextual aggregation.
  - Leave `anyhow` in application but not presentation: rejected because the application boundary would remain opaque to callers.

## Decision 5: Preserve current client-visible semantics where they already exist

- **Decision**: Carry forward existing outward `404` and `412` semantics and introduce a generic sanitized `500` mapping for unexpected internal failures.
- **Rationale**: The feature is about making contracts explicit, not changing the API arbitrarily. Existing known error cases should remain stable while undocumented internals become safely generic.
- **Alternatives considered**:
  - Redesign every error response shape and status code at once: rejected because it expands scope and increases client-facing risk.
  - Expose richer internal failure categories directly to clients: rejected because it conflicts with the requirement to avoid leaking internals.

## Decision 6: Use tests to protect both internal and outward mappings

- **Decision**: Add tests for layer-boundary error propagation and separate HTTP integration tests for client-visible problem-details behavior.
- **Rationale**: The risk in this feature is not only compile-time wiring but also behavioral drift in outward mappings, so both levels need direct coverage.
- **Alternatives considered**:
  - Test only enum conversions: rejected because clients care about actual HTTP behavior.
  - Test only HTTP endpoints: rejected because conversion gaps inside the service become harder to diagnose without lower-level checks.
