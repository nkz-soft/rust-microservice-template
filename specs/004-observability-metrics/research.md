# Research: Observability for Service Operations

## Decision 1: Adopt a tracing-native runtime stack

- **Decision**: Use `tracing`, `tracing-subscriber`, and Actix-compatible tracing middleware as the single runtime instrumentation stack.
- **Rationale**: The workspace already includes `tracing`, and consolidating on a tracing-native stack gives structured request spans, consistent context propagation, and cleaner operational output than the current `env_logger` plus `Logger` setup.
- **Alternatives considered**:
  - Keep `env_logger` only: rejected because plain text logs do not satisfy the structured tracing requirement.
  - Run `env_logger` beside `tracing`: rejected because it risks duplicate request logs and split configuration.

## Decision 2: Preserve or generate request IDs at the HTTP boundary

- **Decision**: Reuse an inbound request correlation identifier when present; otherwise generate one, attach it to tracing context, and return it in the HTTP response.
- **Rationale**: This matches the clarified spec, supports upstream request correlation, and gives clients and operators a shared identifier for troubleshooting.
- **Alternatives considered**:
  - Always generate a new request ID: rejected because it breaks correlation with upstream callers.
  - Require every client to provide a request ID: rejected because it would create an unnecessary adoption hurdle and a brittle operational dependency.

## Decision 3: Expose metrics through an in-process Prometheus exporter

- **Decision**: Use `metrics` with `metrics-exporter-prometheus` to expose runtime metrics through a dedicated `/metrics` endpoint.
- **Rationale**: The issue explicitly asks for Prometheus metrics, and an in-process exporter is the simplest fit for the current single-service runtime model.
- **Alternatives considered**:
  - Write metrics-like information only to logs: rejected because standard monitoring systems cannot scrape or query it effectively.
  - Push metrics to an external collector only: rejected because the current requirement is pull-based endpoint exposure.

## Decision 4: Keep observability concerns out of business layers

- **Decision**: Implement observability in `starter` and HTTP-facing support code without changing domain or application contracts.
- **Rationale**: Observability is a transport/runtime concern in this repository's DDD model, and keeping it at the boundary preserves layer purity.
- **Alternatives considered**:
  - Add observability fields to commands or handlers: rejected because it leaks framework and runtime concerns into the use-case layer.
  - Create a dedicated observability service in application: rejected because the feature does not require business orchestration.

## Decision 5: Exclude `/metrics` traffic from normal request metrics

- **Decision**: Do not include `/metrics` scrapes in request count, latency, or error metrics for the service's normal API traffic.
- **Rationale**: This prevents self-observation traffic from distorting throughput and latency signals and matches the clarified specification.
- **Alternatives considered**:
  - Include `/metrics` in the same metrics as all other endpoints: rejected because it pollutes normal traffic measurements.
  - Introduce a second parallel metrics taxonomy immediately: rejected as unnecessary complexity for the current scope.

## Decision 6: Document observability for operators, not just developers

- **Decision**: Update README and the feature contract with request ID behavior, metrics usage, and example operational queries or dashboard ideas.
- **Rationale**: The spec requires operational guidance, and user-facing observability is incomplete if operators cannot interpret the exposed signals.
- **Alternatives considered**:
  - Document only code-level implementation details: rejected because it does not satisfy operator usability.
  - Leave documentation until after implementation: rejected because documentation parity is part of the feature's acceptance criteria.
