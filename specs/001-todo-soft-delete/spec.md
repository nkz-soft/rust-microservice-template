# Feature Specification: Soft Delete and Audit Metadata for To-Do Items

**Feature Branch**: `001-todo-soft-delete`  
**Created**: 2026-03-22  
**Status**: Draft  
**Input**: User description: "Issue #163 - Add soft delete and audit metadata for to-do items"

## Clarifications

### Session 2026-03-22

- Q: What audit access scope should this feature support for deleted records? → A: Audit access is limited to fetching a single deleted to-do item by ID.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Hide Deleted Items from Standard Use (Priority: P1)

As an API consumer managing to-do items, I want deleting an item to remove it from normal views without destroying its history, so that accidental or business-relevant deletions do not erase the record completely.

**Why this priority**: This is the core user-facing behavior change requested in the issue. Without it, the feature does not deliver soft delete value.

**Independent Test**: Create a to-do item, delete it, then verify standard item retrieval and listing flows no longer return it while the delete operation still reports success.

**Acceptance Scenarios**:

1. **Given** an active to-do item exists, **When** a client deletes that item, **Then** the system marks it as deleted instead of permanently removing it.
2. **Given** a to-do item has been marked as deleted, **When** a client requests the standard list of to-do items, **Then** the deleted item is excluded from the response.
3. **Given** a to-do item has been marked as deleted, **When** a client requests that item through the standard item-by-id flow, **Then** the system responds as if the item is no longer available through normal access.

---

### User Story 2 - Preserve Deletion Audit Details (Priority: P2)

As a maintainer or operator, I want deletion metadata to be stored with the to-do item so that the service preserves when deletion happened and who performed it when actor identity is available.

**Why this priority**: Auditability is the primary business reason for replacing hard delete and directly addresses the issue statement.

**Independent Test**: Delete an item once with actor identity present and once without actor identity present, then verify the deletion timestamp is recorded in both cases and actor information is recorded only when available.

**Acceptance Scenarios**:

1. **Given** an active to-do item exists, **When** it is deleted, **Then** the system records the time the deletion occurred.
2. **Given** an active to-do item is deleted in a context where actor identity is available, **When** the deletion is recorded, **Then** the system stores the actor identity with the deleted item.
3. **Given** an active to-do item is deleted in a context where actor identity is not available, **When** the deletion is recorded, **Then** the system completes the delete operation without requiring actor metadata.

---

### User Story 3 - Review Deleted Item by ID for Audit Purposes (Priority: P3)

As an authorized audit consumer, I want a restricted way to inspect a deleted to-do item by ID so that soft-deleted records remain available for operational review without reappearing in normal client workflows.

**Why this priority**: The issue explicitly calls for optional admin or audit access, but it is secondary to the default delete and hide behavior.

**Independent Test**: Mark an item as deleted, access the audit-oriented item-by-id retrieval flow using authorized access, and verify the deleted item and its deletion metadata can be reviewed without affecting standard endpoints.

**Acceptance Scenarios**:

1. **Given** a to-do item has been soft-deleted, **When** an authorized audit consumer retrieves that item by ID through the audit access path, **Then** the deleted item can be retrieved together with its deletion metadata.
2. **Given** deleted items exist, **When** a normal client uses standard read endpoints, **Then** deleted items remain hidden.

### Edge Cases

- Deleting an already deleted to-do item must not create duplicate deletion records or inconsistent metadata.
- A soft-deleted item must not reappear in paginated, filtered, or sorted standard list responses.
- Standard update flows must not allow a deleted item to be modified as though it were still active.
- If actor identity is unavailable at delete time, the deletion must still succeed with empty actor metadata rather than failing.
- Audit-oriented access must not broaden visibility of deleted items to standard clients.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST replace permanent deletion of to-do items with soft deletion.
- **FR-002**: The system MUST record deletion timestamp metadata for every soft-deleted to-do item.
- **FR-003**: The system MUST record deletion actor metadata when actor identity is available at the time of deletion.
- **FR-004**: The system MUST allow deletion to succeed even when actor identity is unavailable, leaving deletion actor metadata empty in that case.
- **FR-005**: The system MUST exclude soft-deleted to-do items from all standard read operations, including item-by-id retrieval and list retrieval.
- **FR-006**: The system MUST ensure pagination, filtering, and sorting behavior on standard list operations is calculated only from non-deleted to-do items.
- **FR-007**: The system MUST treat soft-deleted to-do items as unavailable to standard update operations.
- **FR-008**: The system MUST provide a restricted audit-oriented access path that allows authorized consumers to retrieve a deleted to-do item by ID together with its deletion metadata.
- **FR-009**: The system MUST preserve existing API versioning and standard endpoint behavior for non-deleted to-do items.
- **FR-010**: The system MUST keep audit metadata associated with a deleted to-do item for as long as the soft-deleted record remains stored.
- **FR-011**: The system MUST include automated verification for default read behavior, deletion metadata capture, and restricted retrieval of deleted items.
- **FR-012**: The system MUST introduce any required persistence changes through repository-managed forward-only migrations.

### Key Entities *(include if feature involves data)*

- **To-Do Item**: A user-managed work item identified by stable item identity and lifecycle metadata, including active or deleted state.
- **Deletion Metadata**: Audit information attached to a soft-deleted to-do item that captures when deletion happened and, when available, who performed it.
- **Audit Consumer**: A privileged actor or system path allowed to review a deleted to-do item by ID without changing standard client visibility rules.

### Assumptions

- The existing standard to-do endpoints remain the primary client-facing API surface and continue to hide deleted items by default.
- Actor identity may not always be present in the current service, so deletion actor tracking is optional per request context rather than mandatory for all deletes.
- The exact authorization mechanism for audit-oriented access will be defined during planning, but the specification requires that the access path be restricted and not public by default.
- Restoring deleted items is out of scope for this feature because it is not requested in issue `#163`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of delete operations on to-do items retain the deleted record with a recorded deletion timestamp instead of permanently removing it.
- **SC-002**: 100% of standard read scenarios exercised by automated tests exclude soft-deleted to-do items from direct retrieval and paginated listing results.
- **SC-003**: 100% of deletion scenarios exercised by automated tests capture deletion actor metadata when actor identity is available and leave it empty when identity is unavailable.
- **SC-004**: Authorized audit retrieval scenarios can access deleted items and their deletion metadata without causing deleted items to appear in any standard client read scenario.
