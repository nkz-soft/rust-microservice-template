# Quickstart: Observability for Service Operations

## Prerequisites

- Docker available for the integration-test database container
- Rust toolchain installed
- Service configuration present in `config.app.toml` or environment variables

## Configure observability locally

Add or override observability settings in `config.app.toml` or via environment variables.

Example environment overrides:

```bash
export MICROSERVICE__OBSERVABILITY__LOG_LEVEL="info"
export MICROSERVICE__OBSERVABILITY__REQUEST_ID_HEADER="x-request-id"
export MICROSERVICE__OBSERVABILITY__METRICS_ENABLED="true"
export MICROSERVICE__OBSERVABILITY__METRICS_PATH="/metrics"
```

## Validate the feature locally

1. Start the service dependencies using the existing deployment scripts, or run the service through the current local workflow.
2. Call a normal API endpoint without a request ID:

```bash
curl -i "http://localhost:8181/api/v1/to-do-items?page=1&page_size=5"
```

3. Confirm the response includes the configured request ID header.
4. Call a normal API endpoint with an inbound request ID:

```bash
curl -i \
  -H "X-Request-Id: client-trace-123" \
  "http://localhost:8181/api/v1/to-do-items?page=1&page_size=5"
```

5. Confirm the response returns the same request ID value.
6. Retrieve Prometheus metrics:

```bash
curl "http://localhost:8181/metrics"
```

7. Confirm the metrics output includes request traffic, latency, and error series for normal API traffic.
8. Re-fetch `/metrics` and confirm scraping it does not inflate the normal request metrics for business endpoints.

## Example operational checks

- Request troubleshooting: filter logs or traces by the request ID returned in the client response.
- Traffic check: inspect the request count metric for `GET /api/v1/to-do-items`.
- Error check: trigger a known `400` or `404` response and verify the error metric series increases.
- Latency check: compare latency series before and after a burst of test requests.

## Verification commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
```

## Expected behaviors

- Every request receives an effective request correlation ID.
- Incoming request IDs are preserved when present.
- The response includes the same request ID used in structured operational records.
- `/metrics` returns Prometheus-formatted output.
- `/metrics` traffic is excluded from normal request count, latency, and error metrics.
