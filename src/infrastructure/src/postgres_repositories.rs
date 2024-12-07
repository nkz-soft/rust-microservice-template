use actix_web::web::Data;
use application::mappers::ToDoItemMapper;
use application::repositories::ToDoItemRepository;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use uuid::Uuid;

use domain::entities::ToDoItem;
use crate::errors::Error::ItemNotFound;

pub struct PostgresToDoItemRepository {
    pool: Data<Pool>,
}

impl PostgresToDoItemRepository {
    pub fn new(pool: &Data<Pool>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl ToDoItemRepository for PostgresToDoItemRepository {
    async fn get_all(&self) -> anyhow::Result<Vec<ToDoItem>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                r#"
                SELECT *  FROM to_do_items;
                "#,
                &[],
            )
            .await?;

        Ok(ToDoItemMapper::from_vec(rows))
    }

    async fn get_by_id(&self, _id: Uuid) -> anyhow::Result<Option<ToDoItem>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                r#"
            SELECT *  FROM to_do_items WHERE id = $1;
            "#,
                &[&_id],
            )
            .await?;

        let row = rows.first();
        match row {
            Some(row) => {
                let to_do_item = ToDoItem {
                    id: row.get(0),
                    title: row.get(1),
                    note: row.get(2),
                };
                Ok(Some(to_do_item))
            }
            None => Err(ItemNotFound { id: _id }.into()),
        }
    }

    async fn save(&self, _entity: ToDoItem) -> anyhow::Result<Uuid> {
        let client = self.pool.get().await?;

        let id = _entity.id;
        let title = _entity.title;
        let note = _entity.note;
        let rows = client
            .query(
                r#"
            INSERT INTO to_do_items (id, title, note)
            VALUES ($1, $2, $3)
            ON CONFLICT(id)
            DO UPDATE SET title = $2, note = $3
            RETURNING id;
            "#,
                &[&id, &title, &note],
            )
            .await?;

        let row = rows.first();
        match row {
            Some(row) => {
                let id = row.get(0);
                Ok(id)
            }
            None => Err(ItemNotFound { id: _entity.id }.into()),
        }
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        let client = self.pool.get().await?;

        client
            .execute(
                r#"
            DELETE FROM to_do_items WHERE id = $1;
            "#,
                &[&id],
            )
            .await?;

        Ok(())
    }
}
