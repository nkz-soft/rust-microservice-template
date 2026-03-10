use crate::errors::Error::{ItemNotFound, VersionConflict};
use crate::DbPool;
use actix_web::web::Data;
use anyhow::{anyhow, Context, Result};
use application::{
    GetAllToDoItemsQuery, PaginatedResult, SortDirection, ToDoItemRepository, ToDoItemSortField,
};
use async_trait::async_trait;
use diesel::dsl::count_star;
use diesel::pg::Pg;
use diesel::prelude::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::PgTextExpressionMethods;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use domain::to_do_items::dsl::{
    id as item_id, note as item_note, title as item_title, to_do_items, version as item_version,
};
use domain::ToDoItem;
use tokio::task;
use uuid::Uuid;

pub struct PostgresToDoItemRepository {
    pool: Data<DbPool>,
}

impl PostgresToDoItemRepository {
    pub fn new(pool: &Data<DbPool>) -> Self {
        Self { pool: pool.clone() }
    }

    async fn run_db<T, F>(&self, operation: F) -> Result<T>
    where
        T: Send + 'static,
        F: FnOnce(&mut PgConnection) -> Result<T> + Send + 'static,
    {
        let pool = self.pool.clone();

        task::spawn_blocking(move || {
            let mut connection = pool
                .get()
                .context("failed to acquire database connection")?;
            operation(&mut connection)
        })
        .await
        .context("database task join failure")?
    }
}

#[async_trait]
impl ToDoItemRepository for PostgresToDoItemRepository {
    async fn get_all(
        &self,
        query: GetAllToDoItemsQuery,
    ) -> anyhow::Result<PaginatedResult<ToDoItem>> {
        self.run_db(move |connection| {
            let total_items = build_filtered_query(query.search.as_deref())
                .select(count_star())
                .first::<i64>(connection)?;

            let items = apply_sort(build_filtered_query(query.search.as_deref()), &query)
                .offset(query.offset())
                .limit(query.limit())
                .load(connection)?;

            Ok(PaginatedResult::new(
                items,
                query.page,
                query.page_size,
                total_items,
            ))
        })
        .await
    }

    async fn get_by_id(&self, todo_item_id: Uuid) -> anyhow::Result<ToDoItem> {
        self.run_db(move |connection| {
            to_do_items
                .filter(item_id.eq(&todo_item_id))
                .first::<ToDoItem>(connection)
                .optional()?
                .ok_or(anyhow!(ItemNotFound { id: todo_item_id }))
        })
        .await
    }

    async fn create(&self, entity: ToDoItem) -> anyhow::Result<Uuid> {
        self.run_db(move |connection| {
            diesel::insert_into(to_do_items)
                .values(&entity)
                .execute(connection)?;
            Ok(entity.id)
        })
        .await
    }

    async fn update(&self, entity: ToDoItem) -> anyhow::Result<Uuid> {
        self.run_db(move |connection| {
            let affected_rows = diesel::update(
                to_do_items.filter(item_id.eq(entity.id).and(item_version.eq(entity.version))),
            )
            .set((
                item_title.eq(entity.title.clone()),
                item_note.eq(entity.note.clone()),
                item_version.eq(entity.version + 1),
            ))
            .execute(connection)?;

            if affected_rows == 1 {
                return Ok(entity.id);
            }

            let actual_version = to_do_items
                .filter(item_id.eq(entity.id))
                .select(item_version)
                .first::<i32>(connection)
                .optional()?;

            match actual_version {
                Some(actual_version) => Err(anyhow!(VersionConflict {
                    id: entity.id,
                    expected_version: entity.version,
                    actual_version,
                })),
                None => Err(anyhow!(ItemNotFound { id: entity.id })),
            }
        })
        .await
    }

    async fn delete(&self, todo_item_id: Uuid) -> anyhow::Result<()> {
        self.run_db(move |connection| {
            diesel::delete(to_do_items.filter(item_id.eq(&todo_item_id))).execute(connection)?;
            Ok(())
        })
        .await
    }
}

fn build_filtered_query<'a>(search: Option<&str>) -> domain::to_do_items::BoxedQuery<'a, Pg> {
    let mut query = to_do_items.into_boxed::<Pg>();

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
