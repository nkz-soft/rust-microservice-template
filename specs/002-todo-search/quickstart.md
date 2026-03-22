# Quickstart: Search Support for To-Do Items

## Prerequisites

- Docker available for the integration-test database container
- Rust toolchain installed
- Service configuration present in `config.app.toml` or environment variables

## Validate the feature locally

1. Start the service dependencies using the existing deployment scripts, or run the service through the current local workflow.
2. Create a few to-do items with distinct titles and notes.
3. Query the list endpoint without search:

```bash
curl "http://localhost:8181/api/v1/to-do-items?page=1&page_size=10"
```

4. Query the list endpoint with a title or note term:

```bash
curl "http://localhost:8181/api/v1/to-do-items?page=1&page_size=10&search=milk&sort=title:asc"
```

5. Verify that:
   - matching items are returned
   - non-matching items are excluded
   - pagination metadata is still present
   - blank search values are rejected with `400 Bad Request`

## Verification commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
```

## Expected behaviors

- Omitting `search` preserves normal list behavior.
- Providing `search` filters on both title and note content.
- Search remains compatible with existing pagination and sort query parameters.
- Deleted items remain hidden from standard list responses.
