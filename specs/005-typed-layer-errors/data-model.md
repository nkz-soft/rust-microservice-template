# Data Model: Explicit Error Types by Application Layer

## Overview

This feature does not add or change persistent business entities. It introduces non-persistent boundary models for failure handling so each layer can communicate errors explicitly without leaking lower-level details.

## Error Contract Entities

### ApplicationError

- **Purpose**: Represents the stable failure categories that can cross normal service-layer boundaries.
- **Likely fields or variants**:
  - `NotFound`
  - `Conflict`
  - `Validation`
  - `Internal`
- **Rules**:
  - Must remain independent from HTTP and persistence frameworks.
  - Must be expressive enough for presentation to choose a stable HTTP problem response.
  - Must not expose raw adapter, driver, or stack-trace details.

### RepositoryError or InfrastructureError

- **Purpose**: Represents persistence and adapter failures inside the infrastructure layer.
- **Likely fields or variants**:
  - resource missing
  - optimistic concurrency conflict
  - storage or connection failure
  - unexpected persistence failure
- **Rules**:
  - May carry internal context needed for logging and diagnosis.
  - Must be translated before becoming part of an application or client-visible contract.
  - Must not be used directly by the presentation layer.

### HttpError Mapping

- **Purpose**: Represents presentation-layer translation from application failures to HTTP problem-details responses.
- **Derived outcomes**:
  - `ApplicationError::NotFound` -> `404`
  - `ApplicationError::Conflict` -> `412`
  - `ApplicationError::Validation` -> `400` when the failure originates from application-level business validation
  - `ApplicationError::Internal` -> `500`
- **Rules**:
  - Must produce stable, documented problem-details responses.
  - Must sanitize internal messages before returning them to clients.
  - Must coexist with existing presentation-owned failures such as malformed payloads, missing auth headers, and missing precondition headers.

## Relationships

- Infrastructure repository operations produce infrastructure-specific failures.
- Infrastructure converts or maps those failures into `ApplicationError` for the repository trait contract.
- Application handlers propagate `ApplicationError` without transport-specific knowledge.
- Presentation maps `ApplicationError` into `HttpError` and then into HTTP problem-details responses.

## State Transitions

### Repository Failure Lifecycle

1. A repository operation interacts with PostgreSQL or another adapter boundary.
2. The infrastructure layer detects a concrete failure condition such as missing data, stale version, or storage failure.
3. The repository implementation maps that condition into the stable application failure contract.
4. The application layer propagates the failure without converting it to transport-specific types.
5. The presentation layer maps the stable application failure category into a problem-details HTTP response.

### Unexpected Internal Failure Lifecycle

1. An unexpected adapter or service failure occurs.
2. Internal context is retained for logging or diagnosis inside the service boundary.
3. The failure is surfaced to higher layers as a stable internal application failure category.
4. The client receives a sanitized generic problem-details response without internal implementation details.
