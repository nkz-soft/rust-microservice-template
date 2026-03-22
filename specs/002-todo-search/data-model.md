# Data Model: Search Support for To-Do Items

## Overview

This feature does not introduce new persisted entities. It extends the retrieval contract for existing to-do items by carrying an optional search term from the HTTP boundary to the repository query path.

## Entities

### ToDoItem

- **Purpose**: Existing user-managed work item returned from list and item-by-id flows.
- **Relevant fields for this feature**:
  - `id`
  - `title`
  - `note`
  - `status`
  - `deleted_at`
- **Feature impact**:
  - `title` and `note` are the searchable fields.
  - `deleted_at` continues to exclude soft-deleted records from standard list search results.

## Query Models

### GetAllToDoItemsQuery

- **Purpose**: Application-layer representation of list retrieval intent.
- **Fields**:
  - `page: u32`
  - `page_size: u32`
  - `search: Option<String>`
  - `sort: ToDoItemSort`
- **Validation/invariants**:
  - `page` and `page_size` must be valid positive values before entering the application layer.
  - `search` is either absent or a non-blank normalized string.
  - `sort` must map to one of the supported list sort options.

### GetAllToDoItemsQueryRequest

- **Purpose**: Presentation-layer request model for list query parameters.
- **Fields**:
  - `page`
  - `page_size`
  - `search`
  - `sort`
- **Validation rules**:
  - `page` is one-based.
  - `page_size` is bounded.
  - `search`, when present, must not be blank and must fit the maximum allowed length.
  - `sort`, when present, must parse to a supported field and direction pair.

## Derived Results

### Search Result Set

- **Purpose**: Paginated list response after applying optional search and sort rules.
- **Composition**:
  - `items: Vec<ToDoItem>`
  - `meta.page`
  - `meta.page_size`
  - `meta.total_items`
  - `meta.total_pages`
- **Rules**:
  - Only items whose title or note match the search term are included when `search` is present.
  - Non-matching items are excluded.
  - Omitted search preserves existing list behavior.
  - Soft-deleted items remain excluded from standard results even if they match the search term.

## State Transitions

- No domain-state transitions are introduced by this feature.
- The feature only changes retrieval filtering and documentation behavior.
