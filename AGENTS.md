# rust-microservice-template Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-23

## Active Technologies
- Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1, Validator 0.20 (feature/002-todo-search)
- PostgreSQL via Diesel and r2d2 pool (feature/002-todo-search)
- Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1, Validator 0.20, `jsonwebtoken`, `actix-web-httpauth`, `argon2` (feature/003-auth-access-control)
- PostgreSQL via Diesel and r2d2 pool for existing to-do data; configuration file and environment variables for bootstrap auth identities and secrets (feature/003-auth-access-control)

- Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1 (001-todo-soft-delete)

## Project Structure

```text
backend/
frontend/
tests/
```

## Commands

cargo test; cargo clippy

## Code Style

Rust 2021: Follow standard conventions

## Recent Changes
- feature/003-auth-access-control: Added Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1, Validator 0.20, `jsonwebtoken`, `actix-web-httpauth`, `argon2`
- feature/002-todo-search: Added Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1, Validator 0.20

- 001-todo-soft-delete: Added Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
