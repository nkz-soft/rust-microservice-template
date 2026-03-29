# rust-microservice-template Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-29

## Active Technologies
- Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1, Validator 0.20 (feature/002-todo-search)
- PostgreSQL via Diesel and r2d2 pool (feature/002-todo-search)
- Rust 2021 + Actix Web 4, Tokio 1, Utoipa 5, Config 0.15, Anyhow 1, Thiserror 2, `tracing`, `tracing-subscriber`, `tracing-actix-web`, `metrics`, `metrics-exporter-prometheus`, `uuid` (feature/004-observability-metrics)
- PostgreSQL via Diesel and r2d2 pool for business data; in-process Prometheus exporter state for runtime metrics (feature/004-observability-metrics)
- Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Anyhow 1, Thiserror 2, `problem_details`, `uuid`, `async-trait` (feature/005-typed-layer-errors)

- Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1 (001-todo-soft-delete)

## Project Structure

```text
/
```

## Commands

cargo test; cargo clippy

## Code Style

Rust 2021: Follow standard conventions

## Recent Changes
- feature/005-typed-layer-errors: Added Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Anyhow 1, Thiserror 2, `problem_details`, `uuid`, `async-trait`
- feature/004-observability-metrics: Added Rust 2021 + Actix Web 4, Tokio 1, Utoipa 5, Config 0.15, Anyhow 1, Thiserror 2, `tracing`, `tracing-subscriber`, `tracing-actix-web`, `metrics`, `metrics-exporter-prometheus`, `uuid`
- feature/002-todo-search: Added Rust 2021 + Actix Web 4, Diesel 2, Tokio 1, Utoipa 5, Config 0.15, Thiserror 2, Anyhow 1, Validator 0.20


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
