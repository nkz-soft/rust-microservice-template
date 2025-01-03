use crate::errors::Error::ItemNotFound;
use crate::DbPool;
use actix_web::web::Data;
use anyhow::anyhow;
use application::repositories::ToDoItemRepository;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use domain::entities::ToDoItem;
use domain::schema::to_do_items::dsl::to_do_items;
use domain::schema::to_do_items::id;
use uuid::Uuid;

pub struct PostgresToDoItemRepository {
    pool: Data<DbPool>,
}

impl PostgresToDoItemRepository {
    pub fn new(pool: &Data<DbPool>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl ToDoItemRepository for PostgresToDoItemRepository {
    async fn get_all(&self) -> anyhow::Result<Vec<ToDoItem>> {
        let mut connection = self.pool.get()?;
        let items = to_do_items.load(&mut connection)?;
        Ok(items)
    }

    async fn get_by_id(&self, item_id: Uuid) -> anyhow::Result<ToDoItem> {
        let mut connection = self.pool.get()?;
        to_do_items
            .filter(id.eq(&item_id))
            .first::<ToDoItem>(&mut connection)
            .optional()?
            .ok_or(anyhow!(ItemNotFound { id: item_id }))
    }

    async fn save(&self, entity: ToDoItem) -> anyhow::Result<Uuid> {
        let mut connection = self.pool.get()?;
        diesel::insert_into(to_do_items)
            .values(&entity)
            .on_conflict(id)
            .do_update()
            .set(&entity)
            .execute(&mut connection)?;
        Ok(entity.id)
    }

    async fn delete(&self, item_id: Uuid) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;
        diesel::delete(to_do_items.filter(id.eq(&item_id))).execute(&mut connection)?;
        Ok(())
    }
}
