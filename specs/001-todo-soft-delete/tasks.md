# Tasks: Soft Delete and Audit Metadata for To-Do Items

**Input**: Implementation plan from `specs/001-todo-soft-delete/plan.md`  
**Status**: Completed

## Setup

- [x] T001 Create the feature specification, checklist, and planning artifacts under `specs/001-todo-soft-delete/`
- [x] T002 Confirm the design against the repository constitution and current DDD/CQRS boundaries

## Tests

- [x] T003 Add or update domain and settings tests for deletion-state behavior and audit configuration
- [x] T004 Extend integration tests for delete, hidden reads, blocked updates, and audit access behavior

## Core

- [x] T005 Update `src/domain` and `src/application` contracts for soft deletion and audit retrieval
- [x] T006 Update `src/infrastructure` persistence and Diesel schema for `deleted_at` and `deleted_by`
- [x] T007 Update `src/presentation` and `src/starter` for the restricted audit endpoint and configuration wiring

## Integration

- [x] T008 Add the forward-only migration for lifecycle fields and verify standard reads filter deleted rows
- [x] T009 Update OpenAPI and contributor-facing documentation for audit access behavior

## Polish

- [x] T010 Run formatting, linting, and test verification for the completed feature
- [x] T011 Capture the final spec artifacts so the implemented feature is traceable in-repo
