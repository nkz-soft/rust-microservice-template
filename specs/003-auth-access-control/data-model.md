# Data Model: Authentication and Authorization Support

## Overview

This feature does not introduce new persisted database entities. It adds a configuration-backed identity model, a token-issuance request and response model, and a normalized authenticated principal used across protected request handling.

## Configuration Models

### AuthSettings

- **Purpose**: Root auth configuration loaded from `config.app.toml` and environment variables.
- **Fields**:
  - `jwt_issuer`
  - `jwt_audience`
  - `jwt_signing_secret`
  - `jwt_ttl_seconds`
  - `users`
  - `services`
- **Validation rules**:
  - signing secret must be present and non-blank when user token issuance is enabled
  - token lifetime must be positive
  - each configured user and service entry must have a unique identifier

### AuthUser

- **Purpose**: Bootstrap human-operated identity that can exchange credentials for a JWT.
- **Fields**:
  - `username`
  - `password_hash`
  - `permissions`
  - `roles` (optional grouping)
- **Validation rules**:
  - `username` must be non-blank and unique
  - `password_hash` must be non-blank
  - `permissions` must contain supported permission names

### ServiceApiKey

- **Purpose**: Machine identity authenticated through a request header.
- **Fields**:
  - `service_name`
  - `header_name`
  - `key`
  - `permissions`
- **Validation rules**:
  - `service_name` must be non-blank and unique
  - `header_name` must be non-blank
  - `key` must be non-blank
  - `permissions` must contain supported permission names

## Request and Response Models

### TokenRequest

- **Purpose**: Open authentication request used to obtain a JWT.
- **Fields**:
  - `username`
  - `password`
- **Validation rules**:
  - both fields are required
  - blank values are rejected at the HTTP boundary

### TokenResponse

- **Purpose**: Response returned after successful user authentication.
- **Fields**:
  - `access_token`
  - `token_type`
  - `expires_in`
  - `permissions`
- **Rules**:
  - `token_type` is `Bearer`
  - `expires_in` reflects the configured token lifetime

## Runtime Models

### AuthenticatedPrincipal

- **Purpose**: Normalized caller context passed into protected request handling.
- **Fields**:
  - `subject`
  - `principal_type` (`user` or `service`)
  - `permissions`
- **Rules**:
  - every protected request must resolve to exactly one authenticated principal
  - authorization decisions are based on `permissions`, not directly on roles

### TokenClaims

- **Purpose**: Signed claims embedded in a user JWT.
- **Fields**:
  - `sub`
  - `iat`
  - `exp`
  - `iss`
  - `aud`
  - `permissions`
- **Rules**:
  - `sub` maps to the configured username
  - `permissions` contains only supported permission names
  - expired or malformed claims invalidate the token

### ProtectedEndpointPolicy

- **Purpose**: Route-level declaration of the permission needed to execute a protected action.
- **Fields**:
  - `required_permission`
  - `accepted_principal_types`
- **Rules**:
  - all business endpoints must have a policy
  - health and documentation routes do not require a protected policy

## Relationships

- `AuthSettings` contains many `AuthUser` entries.
- `AuthSettings` contains many `ServiceApiKey` entries.
- `AuthUser` authenticates through `TokenRequest` and yields `TokenResponse`.
- `TokenResponse.access_token` encodes `TokenClaims`.
- `AuthenticatedPrincipal` is derived either from `TokenClaims` or a matching `ServiceApiKey`.
- `ProtectedEndpointPolicy` evaluates an `AuthenticatedPrincipal` against a required permission.

## State Transitions

### User authentication flow

1. Client submits `TokenRequest`.
2. Credentials are validated against a configured `AuthUser`.
3. On success, the system returns `TokenResponse` with a signed JWT.
4. The client uses that token on subsequent protected requests.

### Service authentication flow

1. Service client sends the configured API key header.
2. The system resolves a matching `ServiceApiKey`.
3. The request becomes an `AuthenticatedPrincipal` of type `service`.
4. Route policy checks the principal permissions before business execution.

### Authorization outcomes

1. Missing or invalid credentials result in no `AuthenticatedPrincipal` and a `401 Unauthorized` response.
2. Valid credentials with insufficient permissions produce an `AuthenticatedPrincipal` but return `403 Forbidden`.
3. Valid credentials with the required permission allow the request to proceed.
