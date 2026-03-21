# Rust Microservice Template Constitution

> Authoritative engineering standards for this repository and its Spec Kit workflow.

## Core Principles

### I. Code Quality
All source code MUST adhere to the following non-negotiable standards:

- Rust edition and language features: Use the workspace edition and modern Rust features such as `let-else`, pattern matching, the `?` operator, `impl Trait`, and `async/await` where they improve clarity. Do not modify workspace toolchain or dependency definitions without explicit approval.
- Compiler and lint discipline: Code should be written to a zero-warning standard. Prefer `#![deny(warnings)]` and `#![forbid(unsafe_code)]` for crates that can support them. `cargo clippy` is mandatory, and stricter lint sets such as `clippy::pedantic` should be enabled when the crate can sustain them without noise. Every `#[allow(...)]` must be narrow and justified.
- Formatting enforcement: `rustfmt` is authoritative. `cargo fmt --check` must pass in CI. Code that violates formatting must not be merged.
- Type safety first: Leverage `Option<T>` and `Result<T, E>` deliberately. Do not use `unwrap()` or `expect()` in reusable or production code paths without strong contextual justification. Prefer `thiserror` for library and domain-facing error types, and use `anyhow` only at application or composition boundaries where error aggregation is appropriate.
- Error handling: Errors must be explicit, typed, and contextual. Never swallow errors. Propagate them or log them with sufficient context. Panics are not acceptable in production paths.
- Async discipline: All async code must use the project-standard runtime and patterns. Every async call must be awaited or explicitly supervised. Fire-and-forget work is prohibited unless lifecycle, retries, and shutdown behavior are defined. Use `spawn_blocking` for blocking or CPU-heavy work. Introduce cancellation propagation when a feature actually requires cancellable workflows.
- Documentation: Public APIs should have `rustdoc` comments with description, arguments, returns, and errors where applicable. Add examples when usage is non-obvious. Public-facing crates should remain compatible with `cargo doc --no-deps`.
- Rust idioms over accidental complexity: Favor composition over inheritance-style designs, traits for meaningful abstraction, cohesive modules, and newtypes where they improve domain clarity. Avoid premature abstraction and framework-driven indirection.

Rationale: Consistent, idiomatic Rust reduces cognitive load, prevents entire classes of bugs at compile time, and keeps the codebase maintainable as complexity grows.

### II. Domain-Driven Design and Architectural Boundaries
This repository follows layered DDD with explicit transport and infrastructure boundaries.

- Layer boundaries are mandatory: `domain`, `application`, `infrastructure`, `presentation`, and `starter` each have a distinct responsibility and must remain separated.
- Domain purity: `domain` contains business concepts, invariants, and rules only. It must not depend on HTTP, database, framework, or runtime details.
- Application orchestration: `application` coordinates commands, queries, handlers, services, and repository contracts. It may depend on `domain`, but not on web or persistence frameworks.
- Infrastructure adapters: `infrastructure` implements PostgreSQL, Diesel, configuration, and other external integrations. It must not own business rules that belong in `domain` or use-case orchestration that belongs in `application`.
- Presentation adapters: `presentation` owns routing, request parsing, validation, response mapping, OpenAPI, and problem-details translation. Transport-specific behavior must stay here.
- Composition root: `starter` wires configuration, runtime, logging, dependency injection, and the selected HTTP adapter. It is the only place allowed to know the whole stack.
- Dependency direction is fixed: `domain <- application <- infrastructure/presentation <- starter`. No reverse dependencies are allowed.
- Ubiquitous language: Business terminology must stay consistent across code, documentation, API contracts, and schema names. Use modules, newtypes, and strongly typed DTOs to encode domain meaning.
- Aggregate and invariant ownership: Business invariants belong in domain entities, value objects, and domain services, not in handlers or repositories.
- CQRS clarity: Commands and queries must remain distinct in intent, inputs, and handlers. Application services must not collapse into generic pass-through wrappers.
- Frameworks are adapters: If multiple HTTP adapters exist, they must preserve the same application contracts and external API behavior unless a specification explicitly approves divergence.

Rationale: DDD aligns the software model with business reality, and Rust's type system makes architectural boundaries enforceable instead of aspirational.

### III. API Contract and Transport Standards
The HTTP API is a stable product surface and must not be an accidental byproduct of framework code.

- API versioning: Public endpoints remain versioned under `/api/v1` until a deliberate versioning decision approves a breaking change.
- Contract-first behavior: Externally visible routes, request rules, response schemas, and error semantics must stay explicit and documented.
- Validation at the boundary: Request bodies, query strings, headers, and path parameters must be validated in the presentation layer before application handlers execute.
- Problem details: HTTP errors must use a consistent problem-details response instead of ad hoc text.
- OpenAPI parity: OpenAPI documentation must be updated alongside code changes and must describe the actual runtime behavior.
- Concurrency and conditional update rules: Headers such as `If-Match`, version fields, and related optimistic concurrency semantics must be enforced consistently when the API exposes them.
- Response stability: Persistence internals must not leak directly into external response contracts.

Rationale: Stable contracts reduce breaking changes, improve client trust, and make framework changes possible without changing business behavior.

### IV. Testing and Verification Standards
Every feature must be accompanied by tests that match the risk and architectural level of the change.

- Unit tests are required for domain logic, validation rules, mappers, and application handlers where behavior can be isolated.
- Integration tests are required for HTTP flows, repository implementations, migrations, and application wiring where correctness depends on real integration points.
- Contract verification is required when API shape, serialization rules, or problem-details responses change. This can be achieved through integration tests plus OpenAPI verification; an additional dedicated contract-testing framework is optional, not mandatory.
- Architectural verification must protect layer boundaries and dependency direction. This may be enforced through crate structure, code review, static checks, and future automation.
- Test independence is mandatory. Tests must not rely on execution order or shared mutable global state. Serialization of tests is allowed only when isolation is otherwise impossible and the reason is explicit.
- Coverage is a guardrail, not a vanity metric. Public behavior must be covered at the right level, and CI coverage must not regress below the enforced repository threshold without an explicit documented exception.
- Naming and readability matter: test names should describe behavior clearly enough that a failure is understandable without opening the implementation.

Rationale: A layered testing strategy catches defects early, validates contracts, and allows confident change in a multi-layer architecture.

### V. Observability, Security, and Operational Safety
Operational quality is part of feature completeness.

- Configuration: Runtime behavior must be configurable through the existing settings model and environment-variable overrides. Hardcoded deployment values are not acceptable.
- Logging and tracing: New work should favor structured, `tracing`-compatible instrumentation over ad hoc logs. Logs must contain enough context to debug failures without leaking secrets or personal data.
- Input safety: All untrusted input must be validated and mapped at the system boundary. Invalid inputs must fail predictably.
- Error exposure: Internal implementation details, secrets, stack traces, and raw database errors must not be exposed to external clients.
- Health endpoints: Liveness and readiness checks must remain meaningful and reflect dependency health where applicable.
- Persistence safety: Schema changes must be delivered with forward-only Diesel migrations and validation that proves they apply cleanly.
- Async and concurrency safety: Shared state in server contexts must be thread-safe. Prefer `Arc` and owned data where concurrency requires them.
- External dependency resilience: Timeouts, retries, idempotency, and failure modes must be considered whenever new external integrations are introduced.

Rationale: Reliable systems are easier to operate, safer to evolve, and cheaper to debug.

### VI. Performance and Simplicity
The project should remain fast, understandable, and cheap to evolve.

- Simplicity first: Prefer the simplest design that satisfies the current requirement. Do not add abstractions, layers, or crates for speculative future needs.
- Request-path discipline: Blocking work must not occur on async request paths. CPU-heavy or blocking operations must be isolated appropriately.
- Database efficiency: Query behavior must be deliberate. Avoid unnecessary round trips, broad scans, and N+1 patterns. Indexing and pagination decisions should be explicit when queries grow.
- Measurable behavior: Performance-sensitive features should define practical latency, throughput, or resource expectations in the plan when performance is a requirement.
- Memory and allocation awareness: Hot paths should avoid unnecessary allocations and copies when a clearer, equally safe alternative exists.

Rationale: Performance matters, but not at the expense of clarity. The correct default is simple, explicit, and measurable.

## Implementation Standards

### Dependency Direction
- `domain` must remain the most stable layer.
- `application` may depend on `domain`.
- `infrastructure` may depend on `application` and `domain`.
- `presentation` may depend on `application` and shared DTO or error contracts, but must not reach into persistence internals.
- `starter` may depend on every layer because it is the composition root.

### API and Data Rules
- Request and query models must validate user input explicitly.
- Response models must be deliberate and stable.
- Breaking API changes require a specification update, migration plan, and versioning decision.
- OpenAPI examples and schema metadata should be updated when request or response behavior changes.

### Persistence Rules
- Database schema evolution must be delivered through Diesel migrations checked into the repository.
- Repository contracts belong in `application`; repository implementations belong in `infrastructure`.
- Transaction boundaries must be explicit in application or infrastructure code and must respect aggregate consistency requirements.

### Delivery Rules
- A specification is not complete until it defines testable acceptance criteria.
- New endpoint behavior must include both success-path and failure-path verification.
- CI is a minimum quality bar, not a substitute for local verification.
- New code must match the style and conventions of existing files under `src/`.
- Existing functions should not be changed unless the task requires it. Prefer adding new modules or narrowly scoped extensions over opportunistic refactoring.
- Tests should cover newly introduced functionality without breaking existing tests. Expanding old test suites is acceptable when required by the change, but unrelated test churn should be avoided.

## Development Workflow

Spec Kit artifacts are part of the delivery process, not optional documentation.

1. Start by creating or updating the feature specification.
2. Produce a plan that maps requirements to layer boundaries, API changes, persistence impact, operational concerns, and testing strategy.
3. Generate or refine tasks so each task maps to a concrete, reviewable outcome.
4. Implement only after the specification and plan are coherent enough to evaluate tradeoffs.
5. Update tests, OpenAPI, README, migrations, and related operational documentation as part of the same change when behavior changes.

All contributors must follow these workflow rules:

- Branching: Work must happen on feature or fix branches. Direct commits to `main` are prohibited.
- Pull requests: Every PR must pass repository CI, including build, formatting, linting, tests, and coverage checks relevant to the change.
- Review: Architectural changes must be reviewed against this constitution, not only for correctness but for boundary compliance.
- Incremental delivery: Changes should be small, focused, and independently reviewable. Monolithic PRs are discouraged.
- Documentation: New public behavior must be documented where users and contributors expect to find it.

## Governance

This constitution supersedes informal habits and short-term implementation convenience.

- Compliance: Every specification, plan, task list, and pull request must be evaluated against these principles.
- Exceptions: Violations must be corrected or explicitly documented as bounded exceptions with rationale and expiry criteria.
- Amendments: Changes require a written rationale, review and approval, and updates to affected templates or guidance.
- Precedence: When a generated plan conflicts with this constitution, the constitution wins until it is formally amended.
- Versioning: Use semantic versioning for this document. Major versions remove or fundamentally change principles, minor versions add new principles or sections, and patch versions clarify wording.

**Version**: 1.1.0 | **Ratified**: 2026-03-21 | **Last Amended**: 2026-03-21
