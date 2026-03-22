# Research: Search Support for To-Do Items

## Decision 1: Matching strategy

- **Decision**: Use case-insensitive substring matching across both title and note content.
- **Rationale**: This satisfies the feature request, is already compatible with the current list-query model, and keeps the design simple enough for the current scale of a to-do service.
- **Alternatives considered**:
  - PostgreSQL full-text search: rejected because the user-visible requirement does not require ranking, stemming, or advanced query syntax.
  - Exact-match filtering only: rejected because it is less useful for the described discovery workflow.

## Decision 2: API shape

- **Decision**: Keep search on `GET /api/v1/to-do-items` as an optional `search` query parameter.
- **Rationale**: This preserves the existing list workflow and matches the specification requirement that search be exposed through query parameters.
- **Alternatives considered**:
  - Dedicated search endpoint: rejected because it duplicates the list API and expands surface area without adding user value.

## Decision 3: Blank input handling

- **Decision**: Reject whitespace-only search input as invalid and treat omitted search as normal listing.
- **Rationale**: This keeps query validation explicit and avoids ambiguous behavior for clients sending accidental blank values.
- **Alternatives considered**:
  - Treat blank search as no-op: rejected because it hides client input mistakes and weakens contract clarity.

## Decision 4: Verification approach

- **Decision**: Verify search primarily through HTTP integration tests, with unit tests for request validation and mapping.
- **Rationale**: Search is externally visible behavior whose correctness depends on the full path from query parsing to repository filtering and paginated response metadata.
- **Alternatives considered**:
  - Repository-only tests: rejected because they do not prove the public HTTP contract.
  - OpenAPI-only verification: rejected because documentation parity alone does not prove runtime behavior.
