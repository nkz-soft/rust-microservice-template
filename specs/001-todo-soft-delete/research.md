# Research: Soft Delete and Audit Metadata for To-Do Items

## Decision 1: Soft delete in the existing table

- **Decision**: Add nullable `deleted_at` and `deleted_by` columns to `to_do_items`.
- **Rationale**: This preserves row identity, versioning, and current repository shape while making hidden-vs-deleted filtering straightforward.
- **Alternatives considered**:
  - Separate archive table: rejected because it adds copy/move semantics and broader migration risk.
  - Event log only: rejected because standard reads still need a simple active/deleted filter.

## Decision 2: Standard access to deleted items

- **Decision**: Standard item reads and updates treat deleted records as `404 Not Found`.
- **Rationale**: This keeps deleted state hidden from normal clients and matches the existing missing-item contract.
- **Alternatives considered**:
  - `410 Gone`: rejected because it leaks deletion state through standard endpoints.
  - `409 Conflict` for update: rejected because the item is functionally unavailable, not merely in conflict.

## Decision 3: Audit access restriction model

- **Decision**: Protect the new audit endpoint with a configured shared audit token header.
- **Rationale**: The current service has no user/session auth model. A configuration-backed token is the smallest change that still creates a restricted path.
- **Alternatives considered**:
  - Full RBAC/authn system: rejected as out of scope for issue `#163`.
  - Unrestricted audit endpoint: rejected because it violates the specification.

## Decision 4: Repeated delete semantics

- **Decision**: Repeated delete calls are idempotent and preserve the original deletion metadata.
- **Rationale**: This makes retries safe and prevents audit noise.
- **Alternatives considered**:
  - Rewriting deletion metadata on every delete: rejected because it destroys the original audit event.
  - Failing repeated deletes: rejected because retrying a delete should stay predictable.
