# Quickstart: Authentication and Authorization Support

## Prerequisites

- Docker available for the integration-test database container
- Rust toolchain installed
- Auth configuration present in `config.app.toml` or environment variables

## Example local configuration

Add auth settings alongside the existing service, database, and audit settings:

```toml
[auth]
jwt_issuer = "rust-template-service"
jwt_audience = "rust-template-clients"
jwt_signing_secret = "replace-for-local-dev-only"
jwt_ttl_seconds = 3600

[[auth.users]]
username = "demo-user"
password_hash = "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$PL01amPyeUuxG7H0vIr5X+qHkZvWnHmGBGXFYvh8z2E"
permissions = ["todo:read", "todo:write"]

[[auth.services]]
service_name = "audit-client"
header_name = "X-Service-Api-Key"
key = "local-service-key"
permissions = ["audit:read"]
```

## Validate the feature locally

1. Start the service dependencies using the existing deployment scripts, or run the service through the current local workflow.
2. Request a JWT for a configured user:

```bash
curl -X POST "http://localhost:8181/api/v1/auth/token" \
  -H "Content-Type: application/json" \
  -d '{"username":"demo-user","password":"password"}'
```

3. Use the returned bearer token on a protected to-do endpoint:

```bash
curl "http://localhost:8181/api/v1/to-do-items?page=1&page_size=10" \
  -H "Authorization: Bearer <access_token>"
```

4. Call the service-protected audit endpoint with an API key:

```bash
curl "http://localhost:8181/api/v1/audit/to-do-items/<id>" \
  -H "X-Service-Api-Key: local-service-key"
```

5. Verify that:
   - requests without credentials to protected routes return `401 Unauthorized`
   - requests with valid credentials but missing permissions return `403 Forbidden`
   - health and OpenAPI routes remain open
   - bearer tokens work for user-facing protected endpoints
   - service API keys work for service-protected endpoints such as audit access

## Verification commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
```

## Expected behaviors

- `/api/v1/auth/token` is open and returns a bearer token only for valid configured user credentials.
- `/api/v1/to-do-items` endpoints are protected by default and require a valid bearer token with the required permission.
- `/api/v1/audit/to-do-items/{id}` accepts the configured service API key and applies the same permission-based authorization model.
- `/api/v1/healthz/*` and API documentation endpoints remain open without credentials.
