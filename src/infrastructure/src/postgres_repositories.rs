use crate::errors::Error::ItemNotFound;
use crate::DbPool;
use actix_web::web::Data;
use anyhow::{anyhow, Context, Result};
use application::ToDoItemRepository;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use domain::to_do_items::dsl::to_do_items;
use domain::to_do_items::id;
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
    async fn get_all(&self) -> anyhow::Result<Vec<ToDoItem>> {
        self.run_db(|connection| {
            let items = to_do_items.load(connection)?;
            Ok(items)
        })
        .await
    }

    async fn get_by_id(&self, item_id: Uuid) -> anyhow::Result<ToDoItem> {
        self.run_db(move |connection| {
            to_do_items
                .filter(id.eq(&item_id))
                .first::<ToDoItem>(connection)
                .optional()?
                .ok_or(anyhow!(ItemNotFound { id: item_id }))
        })
        .await
    }

    async fn save(&self, entity: ToDoItem) -> anyhow::Result<Uuid> {
        self.run_db(move |connection| {
            diesel::insert_into(to_do_items)
                .values(&entity)
                .on_conflict(id)
                .do_update()
                .set(&entity)
                .execute(connection)?;
            Ok(entity.id)
        })
        .await
    }

    async fn delete(&self, item_id: Uuid) -> anyhow::Result<()> {
        self.run_db(move |connection| {
            diesel::delete(to_do_items.filter(id.eq(&item_id))).execute(connection)?;
            Ok(())
        })
        .await
    }
}
