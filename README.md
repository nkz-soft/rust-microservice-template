# rust-microservice-template

![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/nkz-soft/rust-microservice-template?style=flat-square)
![license](https://img.shields.io/github/license/nkz-soft/rust-microservice-template?style=flat-square)
![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/nkz-soft/rust-microservice-template/build-by-tag.yaml)

Template for microservice based on Domain Driven Design principles with Rust

The purpose of this project is to provide a means for building microservices with the last version of Rust that follows basic Domain Driven Design principles

### ⭐ Give a star

If you're using this repository for your learning, samples or your project, please give a star. Thanks :+1:

## Table of Contents

- [Installation](#installation)
- [Architecture](#architecture)
- [Implementation Details](#implementation-details)
- [API Validation](#api-validation)
- [Configuration](#configuration)
- [Coverage](#coverage)
- [Deployment](#deployment)
- [Plan](#plan)
- [Technologies - Libraries](#technologies-used)

## Usage
### Prerequisites

Before you can install and configure the microservice, you will need to ensure that you have the following prerequisites installed:
- Docker and Docker Compose

### Installation
Once you have all the prerequisites installed, you can follow these steps to install and configure your microservice:

1. Clone the repository for your microservice using Git:
```bash
git clone https://github.com/nkz-soft/rust-microservice-template.git
```
2. Change into the directory for your microservice:
```bash
cd rust-microservice-template/deployment/docker
```

3. Use the deployment scripts in `deployment/docker` to start the required services.
   To start the full stack, run:
```bash
./docker-compose-all.sh up --build -d
```
   Available deployment files in this directory:
   - `docker-compose-app.yaml`
   - `docker-compose-infrastructure.yaml`
   - `docker-compose-all.sh`
   - `docker-compose-infrastructure.sh`
4. Verify that the microservice is running correctly by visiting the endpoint in your web browser or using a tool like curl:
```bash
curl -v  http://localhost:8181/api/v1/to-do-items
```

### Coverage

The workspace includes a `cargo` alias for coverage reporting.

1. Install the coverage tool once:
```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
```
2. Generate the local HTML coverage report:
```bash
cargo coverage
```

This writes the report to `target/coverage/html/index.html` and fails if total line coverage drops below `90%`.

Pull requests to `main` also run the `coverage` GitHub Actions workflow, generate a Cobertura report, publish a Markdown coverage summary in the job output, add the same summary as a sticky pull request comment, and upload the generated coverage artifacts.

## Architecture
The microservice follows a layered Domain Driven Design structure with a separate presentation boundary for the HTTP API.

### Domain Layer
The Domain Layer is the core of the system. It contains the entities and domain concepts that model the business problem.
In Rust, this layer is implemented with structs, traits, enums, and functions that stay independent from HTTP and storage concerns.

### Application Layer
The Application Layer coordinates use cases.
It translates incoming requests into commands and queries, invokes domain behavior through handlers and services, and defines repository contracts for persistence.

### Infrastructure Layer
The Infrastructure Layer provides concrete integrations such as PostgreSQL access, Diesel repositories, and migrations.
It is where external systems are wired to the application contracts.

### Presentation Layer
The Presentation Layer exposes the HTTP API.
It owns routing, request and query parsing, validation, response mapping, problem-details error responses, and OpenAPI documentation.

### CQRS
CQRS (Command Query Responsibility Segregation) is used to separate read and write flows into distinct commands, queries, and handlers.

## Implementation Details

This section covers how the template is put together at the API and runtime level.

### API Validation

The API validates request bodies and query parameters before they reach the application layer.

#### Create and update requests

`POST /api/v1/to-do-items` and `PUT /api/v1/to-do-items/{id}` enforce these rules:

- `title` is required, must not be blank, and must be at most 120 characters
- `note` is required, must not be blank, and must be at most 1000 characters
- JSON request bodies larger than 8 KB are rejected

Example valid create request:

```bash
curl -X POST http://localhost:8181/api/v1/to-do-items \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"Buy milk\",\"note\":\"2 liters\"}"
```

Example invalid create request:

```bash
curl -X POST http://localhost:8181/api/v1/to-do-items \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"   \",\"note\":\"2 liters\"}"
```

Invalid payloads return `400 Bad Request` with a problem-details JSON response.

#### List query parameters

`GET /api/v1/to-do-items` supports optional validated query parameters:

- `page`: one-based page number, default `1`
- `page_size`: number of items per page, default `20`, maximum `100`
- `search`: optional case-insensitive filter applied to title and note, must not be blank
- `sort`: deterministic sorting, supports `id:asc`, `id:desc`, `title:asc`, and `title:desc`

Example:

```bash
curl "http://localhost:8181/api/v1/to-do-items?page=1&page_size=10&search=milk&sort=title:asc"
```

Invalid query parameters also return `400 Bad Request`.

#### List response shape

`GET /api/v1/to-do-items` returns a paginated payload:

```json
{
  "items": [
    {
      "id": "6f8d9d10-4d9f-4b97-9cd2-53f4f4224f2e",
      "title": "Buy milk",
      "note": "2 liters"
    }
  ],
  "meta": {
    "page": 1,
    "page_size": 10,
    "total_items": 1,
    "total_pages": 1
  }
}
```

### Configuration

To configure the microservice, modify `config.app.toml`.

```toml
[service]
http_url = '127.0.0.1:8181'
service_name = 'rust_template_service'

[database]
database_url = 'postgres://postgres:postgres@localhost:5432/rust_template_db'
```

You can also configure the service via environment variables.

```bash
export MICROSERVICE__SERVICE__HTTP_URL="127.0.0.1:8181"
export MICROSERVICE__SERVICE__SERVICE_NAME="rust_template_service"
export MICROSERVICE__DATABASE__DATABASE_URL="postgres://postgres:postgres@localhost:5432/rust_template_db"
```

### OpenAPI and Error Handling

The template includes OpenAPI generation through `utoipa` and Swagger UI integration for API discovery.
Swagger UI is available at `GET /api/v1/swagger-ui/` and OpenAPI JSON at `GET /api/v1/api-docs/openapi.json`.

Validation and request parsing errors are returned as problem-details responses, giving clients a structured `400 Bad Request` payload instead of ad hoc text errors.

### API Versioning Strategy

The API is explicitly versioned under `/api/v1`.
All current endpoints, including health checks, live under this prefix, for example:

- `GET /api/v1/to-do-items`
- `POST /api/v1/to-do-items`
- `GET /api/v1/healthz/ready`

Future breaking API changes should be introduced under a new version prefix (for example `/api/v2`) while keeping previous versions available during migration windows.

## Deployment

## Plan

- [x] New blank microservice solution
- [x] Basic Github actions
- [x] Docker compose
- [x] REST API
- [x] CQRS (Command Query Responsibility Segregation)
- [x] PostgreSQL storage with diesel
    - [X] Add support for migrations
    - [X] CRUD Operation
- [X] Integration tests
- [ ] Automated code coverage reporting
- [X] Configuration
    - [X] Configuration file
    - [X] Environment variables
- [x] OpenAPI documentation
- [x] Advanced error handling
- [x] Health checks
- [x] Problem details
- [x] API Validation
- [ ] Coming soon :)

## Technologies used

- [Rust](https://github.com/rust-lang/rust): The Rust Programming Language.
- [Actix](https://github.com/actix/actix-web): Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
- [rust-postgres](https://github.com/sfackler/rust-postgres): PostgreSQL support for Rust.
- [testcontainers-rs](https://github.com/testcontainers/testcontainers-rs): Testcontainers-rs is the official Rust language fork of http://testcontainers.org.
- [utoipa](https://github.com/juhaku/utoipa): Code first and compile time generated OpenAPI documentation for Rust APIs.
- [diesel](https://github.com/diesel-rs/diesel): Diesel is a Rust ORM and query builder.
- [diesel_migrations](https://github.com/diesel-rs/diesel_migrations): Diesel migrations for Rust.
- [problem_details](https://github.com/frenetisch-applaudierend/problem-details-rs): Problem details for Rust.
- [validator](https://github.com/Keats/validator): Request and query validation for Rust structs.

