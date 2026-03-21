# Data Model: Soft Delete and Audit Metadata for To-Do Items

## Entity: ToDoItem

### Fields

- `id`: stable item identifier
- `title`: optional title text
- `note`: optional note text
- `status`: lifecycle status for active work
- `created_at`: creation timestamp
- `updated_at`: last non-delete update timestamp
- `due_at`: optional due timestamp
- `version`: optimistic concurrency version
- `deleted_at`: optional deletion timestamp; `null` means the item is active
- `deleted_by`: optional actor identifier for the delete operation

### Validation Rules

- `deleted_at` must be set exactly once when an active item is soft-deleted.
- `deleted_by` may be empty when actor identity is unavailable.
- An item with `deleted_at != null` is considered deleted and must be excluded from standard reads.
- Standard updates must not apply to deleted items.

### State Transitions

- `Active -> Deleted`
  - Trigger: delete command
  - Effects: set `deleted_at`, optionally set `deleted_by`
- `Deleted -> Deleted`
  - Trigger: repeated delete command
  - Effects: no state change; original deletion metadata remains intact

## Entity: Audit Consumer

### Purpose

A caller allowed to inspect a deleted to-do item by ID through the restricted audit path.

### Access Rule

- Must provide the configured audit token header accepted by the service.

## Query Model Impact

- Standard list queries filter on active items only.
- Standard item-by-id queries resolve only active items.
- Audit item-by-id queries resolve deleted items only.
