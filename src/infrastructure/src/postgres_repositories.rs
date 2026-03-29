use crate::errors::Error::{InternalError, ItemNotFound, VersionConflict};
use crate::DbPool;
use actix_web::web::Data;
use application::{
    ApplicationError, ApplicationResult, GetAllToDoItemsQuery, PaginatedResult, SortDirection,
    ToDoItemRepository, ToDoItemSortField,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::dsl::count_star;
use diesel::pg::Pg;
use diesel::prelude::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::PgTextExpressionMethods;
use diesel::{Insertable, OptionalExtension, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use domain::to_do_items::dsl::{
    deleted_at as item_deleted_at, deleted_by as item_deleted_by, due_at as item_due_at,
    id as item_id, note as item_note, status as item_status, title as item_title, to_do_items,
    updated_at as item_updated_at, version as item_version,
};
use domain::ToDoItem;
use tokio::task;
use uuid::Uuid;

pub struct PostgresToDoItemRepository {
    pool: Data<DbPool>,
}

#[derive(Queryable)]
struct DbToDoItem {
    id: Uuid,
    title: Option<String>,
    note: Option<String>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    due_at: Option<DateTime<Utc>>,
    version: i32,
    deleted_at: Option<DateTime<Utc>>,
    deleted_by: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = domain::to_do_items)]
struct NewDbToDoItem {
    id: Uuid,
    title: Option<String>,
    note: Option<String>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    due_at: Option<DateTime<Utc>>,
    version: i32,
    deleted_at: Option<DateTime<Utc>>,
    deleted_by: Option<Uuid>,
}

impl From<DbToDoItem> for ToDoItem {
    fn from(item: DbToDoItem) -> Self {
        ToDoItem {
            id: item.id,
            title: item.title,
            note: item.note,
            status: item.status,
            created_at: item.created_at,
            updated_at: item.updated_at,
            due_at: item.due_at,
            version: item.version,
            deleted_at: item.deleted_at,
            deleted_by: item.deleted_by,
        }
    }
}

impl From<&ToDoItem> for NewDbToDoItem {
    fn from(item: &ToDoItem) -> Self {
        Self {
            id: item.id,
            title: item.title.clone(),
            note: item.note.clone(),
            status: item.status.clone(),
            created_at: item.created_at,
            updated_at: item.updated_at,
            due_at: item.due_at,
            version: item.version,
            deleted_at: item.deleted_at,
            deleted_by: item.deleted_by,
        }
    }
}

impl PostgresToDoItemRepository {
    pub fn new(pool: &Data<DbPool>) -> Self {
        Self { pool: pool.clone() }
    }

    async fn run_db<T, F>(&self, operation: F) -> ApplicationResult<T>
    where
        T: Send + 'static,
        F: FnOnce(&mut PgConnection) -> std::result::Result<T, crate::Error> + Send + 'static,
    {
        let pool = self.pool.clone();

        task::spawn_blocking(move || {
            let mut connection = pool.get().map_err(|err| {
                ApplicationError::internal(format!("failed to acquire database connection: {err}"))
            })?;
            operation(&mut connection).map_err(ApplicationError::from)
        })
        .await
        .map_err(|err| ApplicationError::internal(format!("database task join failure: {err}")))?
    }
}

#[async_trait]
impl ToDoItemRepository for PostgresToDoItemRepository {
    async fn get_all(
        &self,
        query: GetAllToDoItemsQuery,
    ) -> ApplicationResult<PaginatedResult<ToDoItem>> {
        self.run_db(move |connection| {
            let total_items = build_filtered_query(query.search.as_deref())
                .select(count_star())
                .first::<i64>(connection)
                .map_err(map_diesel_error)?;

            let items = apply_sort(build_filtered_query(query.search.as_deref()), &query)
                .offset(query.offset())
                .limit(query.limit())
                .load::<DbToDoItem>(connection)
                .map_err(map_diesel_error)?
                .into_iter()
                .map(ToDoItem::from)
                .collect();

            Ok(PaginatedResult::new(
                items,
                query.page,
                query.page_size,
                total_items,
            ))
        })
        .await
    }

    async fn get_by_id(&self, todo_item_id: Uuid) -> ApplicationResult<ToDoItem> {
        self.run_db(move |connection| {
            to_do_items
                .filter(item_id.eq(&todo_item_id).and(item_deleted_at.is_null()))
                .first::<DbToDoItem>(connection)
                .optional()
                .map_err(map_diesel_error)?
                .map(ToDoItem::from)
                .ok_or(ItemNotFound { id: todo_item_id })
        })
        .await
    }

    async fn get_deleted_by_id_for_audit(&self, todo_item_id: Uuid) -> ApplicationResult<ToDoItem> {
        self.run_db(move |connection| {
            to_do_items
                .filter(item_id.eq(&todo_item_id).and(item_deleted_at.is_not_null()))
                .first::<DbToDoItem>(connection)
                .optional()
                .map_err(map_diesel_error)?
                .map(ToDoItem::from)
                .ok_or(ItemNotFound { id: todo_item_id })
        })
        .await
    }

    async fn create(&self, entity: ToDoItem) -> ApplicationResult<Uuid> {
        self.run_db(move |connection| {
            let new_entity = NewDbToDoItem::from(&entity);
            diesel::insert_into(to_do_items)
                .values(&new_entity)
                .execute(connection)
                .map_err(map_diesel_error)?;
            Ok(entity.id)
        })
        .await
    }

    async fn update(&self, entity: ToDoItem) -> ApplicationResult<Uuid> {
        self.run_db(move |connection| {
            let next_updated_at = Utc::now();
            let affected_rows = diesel::update(
                to_do_items.filter(
                    item_id
                        .eq(entity.id)
                        .and(item_version.eq(entity.version))
                        .and(item_deleted_at.is_null()),
                ),
            )
            .set((
                item_title.eq(entity.title.clone()),
                item_note.eq(entity.note.clone()),
                item_status.eq(entity.status.clone()),
                item_due_at.eq(entity.due_at),
                item_updated_at.eq(next_updated_at),
                item_version.eq(entity.version + 1),
            ))
            .execute(connection)
            .map_err(map_diesel_error)?;

            if affected_rows == 1 {
                return Ok(entity.id);
            }

            let actual_version = to_do_items
                .filter(item_id.eq(entity.id).and(item_deleted_at.is_null()))
                .select(item_version)
                .first::<i32>(connection)
                .optional()
                .map_err(map_diesel_error)?;

            match actual_version {
                Some(actual_version) => Err(VersionConflict {
                    id: entity.id,
                    expected_version: entity.version,
                    actual_version,
                }),
                None => Err(ItemNotFound { id: entity.id }),
            }
        })
        .await
    }

    async fn delete(&self, todo_item_id: Uuid, deleted_by: Option<Uuid>) -> ApplicationResult<()> {
        self.run_db(move |connection| {
            let deleted_at = Utc::now();
            diesel::update(
                to_do_items.filter(item_id.eq(&todo_item_id).and(item_deleted_at.is_null())),
            )
            .set((
                item_deleted_at.eq(Some(deleted_at)),
                item_deleted_by.eq(deleted_by),
            ))
            .execute(connection)
            .map_err(map_diesel_error)?;
            Ok(())
        })
        .await
    }
}

fn build_filtered_query<'a>(search: Option<&str>) -> domain::to_do_items::BoxedQuery<'a, Pg> {
    let mut query = to_do_items
        .filter(item_deleted_at.is_null())
        .into_boxed::<Pg>();

    if let Some(search) = search {
        let pattern = format!("%{}%", search.trim());
        query = query.filter(
            item_title
                .ilike(pattern.clone())
                .or(item_note.ilike(pattern)),
        );
    }

    query
}

fn apply_sort<'a>(
    query: domain::to_do_items::BoxedQuery<'a, Pg>,
    params: &GetAllToDoItemsQuery,
) -> domain::to_do_items::BoxedQuery<'a, Pg> {
    match (&params.sort.field, &params.sort.direction) {
        (ToDoItemSortField::Id, SortDirection::Asc) => query.order(item_id.asc()),
        (ToDoItemSortField::Id, SortDirection::Desc) => query.order(item_id.desc()),
        (ToDoItemSortField::Title, SortDirection::Asc) => {
            query.order((item_title.asc(), item_id.asc()))
        }
        (ToDoItemSortField::Title, SortDirection::Desc) => {
            query.order((item_title.desc(), item_id.desc()))
        }
    }
}

fn map_diesel_error(err: diesel::result::Error) -> crate::Error {
    InternalError(format!("database operation failed: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error as InfrastructureError;

    #[test]
    fn infrastructure_not_found_maps_to_application_not_found() {
        let id = Uuid::new_v4();

        let error = ApplicationError::from(InfrastructureError::ItemNotFound { id });

        assert_eq!(error, ApplicationError::NotFound { id });
    }

    #[test]
    fn infrastructure_version_conflict_maps_to_application_conflict() {
        let id = Uuid::new_v4();

        let error = ApplicationError::from(InfrastructureError::VersionConflict {
            id,
            expected_version: 2,
            actual_version: 3,
        });

        assert_eq!(
            error,
            ApplicationError::Conflict {
                id,
                expected_version: 2,
                actual_version: 3,
            }
        );
    }

    #[test]
    fn diesel_errors_are_sanitized_to_internal_infrastructure_errors() {
        let error = map_diesel_error(diesel::result::Error::NotFound);

        match error {
            InfrastructureError::InternalError(message) => {
                assert!(message.contains("database operation failed"));
            }
            other => panic!("expected internal error, got {other:?}"),
        }
    }
}
