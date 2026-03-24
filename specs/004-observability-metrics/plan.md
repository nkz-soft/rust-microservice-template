# Implementation Plan: Observability for Service Operations

**Branch**: `feature/004-observability-metrics` | **Date**: 2026-03-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/004-observability-metrics/spec.md`

## Summary

Add a tracing-native observability layer to the existing Actix service by replacing the current `env_logger` plus `Logger` setup with structured request tracing, correlation-ID propagation, and Prometheus-compatible metrics exposure through `/metrics`. The implementation stays in the composition and presentation boundaries, adds configuration for observability behavior, documents the operational surface, and verifies the feature through HTTP integration tests plus targeted configuration and OpenAPI checks.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: Actix Web 4, Tokio 1, Utoipa 5, Config 0.15, Anyhow 1, Thiserror 2, `tracing`, `tracing-subscriber`, `tracing-actix-web`, `metrics`, `metrics-exporter-prometheus`, `uuid`  
**Storage**: PostgreSQL via Diesel and r2d2 pool for business data; in-process Prometheus exporter state for runtime metrics  
**Testing**: `cargo test`, integration tests in `src/starter/tests`, crate-level unit tests, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`  
**Target Platform**: Linux-hosted HTTP service  
**Project Type**: Layered Rust web service  
**Performance Goals**: Keep observability overhead low enough that normal request handling remains responsive, expose `/metrics` on-demand, and avoid self-scrape distortion in service traffic metrics  
**Constraints**: Preserve `/api/v1` API behavior, keep DDD layer boundaries intact, apply the same access model as the existing API to `/metrics`, reuse incoming request IDs when present, return the request ID in responses, exclude `/metrics` traffic from normal request metrics, and avoid exposing implementation internals or secrets in logs  
**Scale/Scope**: One service process, all existing HTTP endpoints, one new `/metrics` endpoint, startup/runtime configuration changes, README/OpenAPI updates, and end-to-end verification of tracing and metrics behavior

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- `PASS`: The plan keeps observability as cross-cutting transport and composition behavior in `starter` and `presentation`, without pushing framework details into `application` or `domain`.
- `PASS`: Public API behavior remains versioned under `/api/v1`; the only new external surface is the operational `/metrics` endpoint plus response headers on existing endpoints.
- `PASS`: Configuration continues through the existing settings model and environment-variable overrides; no hardcoded deployment-only values are introduced.
- `PASS`: The plan expands structured tracing and metrics in line with the constitution's observability and operational-safety requirements while explicitly avoiding secret leakage.
- `PASS`: Verification covers integration behavior, documentation parity, and configuration wiring, which matches the repository testing standard for transport and runtime changes.
- `PASS WITH NOTE`: This feature adds new workspace dependencies, but they are narrowly scoped to observability and replace an existing weaker logging path rather than adding speculative abstraction.

## Phase 0: Research

### Decisions

1. **Logging and tracing stack**
   - Decision: Standardize runtime instrumentation on `tracing` with `tracing-subscriber`, and replace `env_logger` plus Actix `Logger` in the composition root.
   - Rationale: The workspace already includes `tracing`, and a single tracing-native stack avoids split formatting, duplicate request logs, and ad hoc context propagation.
   - Alternatives considered:
     - Keep `env_logger` and add more text logs: rejected because it does not satisfy structured tracing or correlation needs cleanly.
     - Run both `env_logger` and `tracing` independently: rejected because it increases duplication and configuration drift.

2. **Request correlation strategy**
   - Decision: Accept an inbound request ID when present, otherwise generate one, attach it to tracing spans, and return it in the response headers.
   - Rationale: This matches the clarified spec, preserves upstream trace continuity, and gives clients a stable identifier for incident reporting.
   - Alternatives considered:
     - Always generate a new ID: rejected because it breaks upstream correlation.
     - Require clients to send an ID: rejected because it would be a breaking operational expectation.

3. **Metrics exposure model**
   - Decision: Expose Prometheus-formatted metrics from a dedicated `/metrics` endpoint using an in-process exporter registered during startup.
   - Rationale: This aligns with the issue, is a natural fit for service monitoring, and keeps metrics retrieval independent from business endpoints.
   - Alternatives considered:
     - Push metrics only to logs: rejected because it is harder to integrate with standard monitoring systems.
     - Emit metrics through a versioned business API path: rejected because metrics are an operational surface, not business data.

4. **Request metrics scope**
   - Decision: Track request count, latency, and error metrics for normal API traffic while explicitly excluding `/metrics` scrapes from those measurements.
   - Rationale: This matches the clarified spec and prevents self-observation traffic from polluting service latency and throughput signals.
   - Alternatives considered:
     - Count `/metrics` with all other endpoints: rejected because it distorts monitoring data.
     - Build a separate metrics namespace for every operational path immediately: rejected as unnecessary scope for this feature.

5. **Implementation boundary**
   - Decision: Implement middleware, exporter wiring, and settings changes in `starter` and presentation-oriented support modules without changing domain or application contracts.
   - Rationale: Observability is infrastructure and transport behavior in this repository's DDD structure, so the simplest correct design keeps business layers untouched.
   - Alternatives considered:
     - Add observability concepts to application handlers: rejected because it would leak transport/runtime concerns into the use-case layer.

## Phase 1: Design & Contracts

### Data Model Changes

- No persistent schema changes are required.
- Add configuration models for observability runtime settings under the existing `Settings` structure.
- Add request-scoped observability context carrying:
  - effective request correlation ID
  - route or endpoint label used for metrics attribution
  - outcome classification for success vs. error metrics
- Add a metrics endpoint response model at the contract level only; it is generated text rather than a domain DTO.

### Application Design

- No changes to domain entities, handlers, commands, queries, or repository contracts are planned.
- Existing application services remain unaware of observability implementation details.
- Any error classification used for metrics is derived at the HTTP boundary from response status or transport errors, not from new application-layer abstractions.

### Infrastructure and Composition Design

- Replace startup logging initialization in `src/starter/src/main.rs` with tracing-subscriber initialization configured from settings and environment overrides.
- Replace `Logger::default()` usage in `src/starter/src/lib.rs` with tracing-aware request middleware and custom middleware/helpers for:
  - request ID extraction or generation
  - response header injection for the effective request ID
  - request timing and status classification
  - metrics recording that excludes `/metrics`
- Register the Prometheus exporter once during startup and expose a handler that renders current metrics output.
- Keep observability setup isolated to the composition root and supporting modules rather than threading it through feature-specific handlers.

### Presentation Design

- Add a presentation-level handler for `/metrics` because it is an HTTP-facing operational endpoint.
- Document the response header that carries the request correlation identifier for normal API responses.
- Keep the metrics endpoint on the same access model as the rest of the API, but outside `/api/v1` because it is operational infrastructure rather than business API versioning.
- Ensure health, Swagger, OpenAPI, and business endpoints continue to coexist cleanly with the added middleware chain.

### Configuration Design

- Extend `Settings` with observability configuration values for:
  - log or trace filter level
  - request ID header name or canonical header usage
  - metrics enablement and/or endpoint path if needed by the current settings model
- Preserve current environment override behavior through `MICROSERVICE__...` variables.
- Default configuration should be safe for local development while remaining overrideable in deployment.

### Documentation Design

- Update README with:
  - request ID response-header behavior
  - metrics endpoint usage
  - sample Prometheus queries or dashboard-oriented checks
  - any new observability-related environment variables
- Update OpenAPI only for user-visible request ID response headers on versioned endpoints if supported by the current tooling; `/metrics` may remain documented via contract artifact and README if not part of the business OpenAPI surface.
- Keep documentation focused on observable behavior, not crate internals.

## Phase 2: Implementation Plan

1. **Establish runtime observability foundations**
   - Add the required workspace and crate dependencies for tracing subscribers, Actix tracing integration, and Prometheus metrics export.
   - Extend settings and configuration loading for observability controls.
   - Replace legacy logger initialization with tracing subscriber setup at startup.

2. **Add request correlation and structured tracing**
   - Introduce middleware/helpers that reuse or generate request IDs.
   - Attach the effective request ID to request spans and response headers.
   - Ensure successful and failed requests are distinguishable in structured output without exposing secrets.

3. **Add Prometheus metrics exposure**
   - Initialize the exporter once.
   - Record request count, latency, and error metrics for normal API traffic.
   - Exclude `/metrics` requests from normal request metrics and expose the metrics handler.

4. **Document the operational surface**
   - Update README and any relevant API docs with request ID behavior, metrics retrieval, and example operational queries.
   - Keep wording aligned with the clarified spec and contract artifact.

5. **Verify behavior end to end**
   - Add integration tests for request ID propagation, metrics availability, and `/metrics` exclusion behavior.
   - Add unit tests for new settings/defaults and any header-parsing helpers.
   - Run formatting, clippy, and workspace tests.

## Project Structure

### Documentation (this feature)

```text
specs/
└── 004-observability-metrics/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/
    │   └── observability-http.yaml
    └── checklists/
        └── requirements.md
```

### Source Code (repository root)

```text
src/
├── application/
│   └── src/
│       └── settings.rs
├── presentation/
│   └── src/
│       ├── api/
│       │   ├── api_doc.rs
│       │   ├── api_health_check.rs
│       │   ├── app.rs
│       │   └── mod.rs
│       └── lib.rs
└── starter/
    ├── src/
    │   ├── lib.rs
    │   ├── main.rs
    │   └── observability.rs
    └── tests/
        └── integration.rs
```

**Structure Decision**: Keep the existing layered workspace and implement observability as composition-root and HTTP-boundary behavior. Add a small dedicated support module under `starter` for runtime wiring rather than spreading middleware setup through feature-specific files.

## Test Strategy

- Unit tests:
  - settings defaults and environment overrides for observability config
  - request ID extraction/generation helpers
  - response header propagation helpers if split into reusable functions
- Integration tests:
  - successful requests return a request ID header and keep it stable through the response
  - incoming request ID values are preserved in responses
  - failed requests still return a request ID and contribute to error observability behavior
  - `GET /metrics` returns Prometheus-formatted output
  - `/metrics` traffic does not inflate normal request count metrics
  - existing business endpoints remain functional with the new middleware stack
- Documentation and contract verification:
  - OpenAPI tests remain green where versioned API docs change
  - README examples and contract artifact match shipped behavior
- Verification commands:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets --all-features`
  - `cargo test --workspace`

## Risks and Mitigations

- **Risk**: Double-logging or conflicting subscriber initialization could produce noisy or broken runtime output.
  - Mitigation: Remove or replace the legacy `env_logger` and Actix `Logger` path instead of layering new instrumentation on top.
- **Risk**: Middleware ordering could cause missing request IDs in responses or spans.
  - Mitigation: Centralize observability middleware composition in `starter` and verify both inbound-preserved and generated-ID scenarios through integration tests.
- **Risk**: Metrics labels could create unstable or high-cardinality series.
  - Mitigation: Keep metric dimensions limited to stable route/outcome labels and explicitly avoid using raw request IDs as metric labels.
- **Risk**: `/metrics` could accidentally bypass the intended access model or contaminate request metrics.
  - Mitigation: Cover endpoint accessibility and metric exclusion behavior in integration tests and keep the exclusion rule explicit in middleware code.

## Complexity Tracking

No constitution violations currently require exception handling.
