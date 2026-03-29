# Quickstart: Explicit Error Types by Application Layer

## Prerequisites

- Rust toolchain installed
- PostgreSQL available through the project's existing local or test workflow
- Service configuration present in `config.app.toml` or environment variables

## Implement the feature

1. Add a typed error module in `src/application/src/` and export it from `src/application/src/lib.rs`.
2. Update `ToDoItemRepository` and the query handlers to return typed application errors instead of `anyhow::Result`.
3. Refactor infrastructure repository implementations to translate persistence failures into the typed application contract.
4. Refactor `src/presentation/src/errors.rs` so HTTP problem details map from application errors directly.
5. Update OpenAPI descriptions and README guidance for stable client-visible failure behavior.

## Validate the contract locally

1. Start the service using the project's normal local workflow.
2. Request a non-existent to-do item:

```bash
curl -i "http://localhost:8181/api/v1/to-do-items/00000000-0000-0000-0000-000000000001"
```

3. Confirm the response is a `404` problem-details payload without database-specific internals.
4. Create a to-do item, then update it twice using a stale `If-Match` value:

```bash
curl -i -X PUT \
  -H "Content-Type: application/json" \
  -H "If-Match: \"1\"" \
  -d '{"title":"updated","status":"new"}' \
  "http://localhost:8181/api/v1/to-do-items/<id>"
```

5. Confirm the stale update returns `412` with the stable conflict problem response.
6. Exercise a representative unexpected repository failure through tests or a controlled test double and confirm the HTTP response is a sanitized `500`.

## Verification commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
```

`cargo test --workspace` requires Docker for the existing `src/starter/tests/integration.rs` testcontainers suite. When Docker is unavailable, run `cargo test --workspace --lib` to verify the typed-error refactor and rerun the full workspace suite in a Docker-enabled environment.

## Expected behaviors

- Normal repository and handler flows use explicit typed errors instead of generic cross-layer aggregation.
- Presentation maps application errors to stable problem-details responses.
- Known missing-resource and concurrency failures keep their existing outward HTTP semantics.
- Unexpected internal failures return sanitized generic `500` responses.
- Startup and outer composition boundaries may still use `anyhow` where aggregated context is appropriate.
- Problem-details responses for typed application failures include stable `title`, `status`, and `detail` fields.
