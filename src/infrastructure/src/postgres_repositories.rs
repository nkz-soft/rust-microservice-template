use actix_web::web::Data;
use application::mappers::ToDoItemMapper;
use application::repositories::ToDoItemRepository;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use uuid::Uuid;

use domain::entities::ToDoItem;

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
    async fn get_all(&self) -> Result<Vec<ToDoItem>, String> {
        let client = self.pool.get().await.unwrap();

        let rows = client
            .query(
                r#"
                SELECT *  FROM to_do_items;
                "#,
                &[],
            )
            .await
            .unwrap();

        Ok(ToDoItemMapper::from_vec(rows))
    }

    async fn get_by_id(&self, _id: Uuid) -> Result<Option<ToDoItem>, String> {
        let client = self.pool.get().await.unwrap();

        let rows = client
            .query(
                r#"
            SELECT *  FROM to_do_items WHERE id = $1;
            "#,
                &[&_id],
            )
            .await
            .unwrap();

        let row = rows.iter().next();
        match row {
            Some(row) => {
                let to_do_item = ToDoItem {
                    id: row.get(0),
                    title: row.get(1),
                    note: row.get(2),
                };
                Ok(Some(to_do_item.try_into().unwrap()))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, _entity: ToDoItem) -> Result<Uuid, String> {
        let client = self.pool.get().await.unwrap();

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
            .await
            .unwrap();

        let row = rows.iter().next();
        match row {
            Some(row) => {
                let id = row.get(0);
                Ok(id)
            }
            None => Err(String::from("Could not save item.")),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        let client = self.pool.get().await.unwrap();

        client
            .execute(
                r#"
            DELETE FROM to_do_items WHERE id = $1;
            "#,
                &[&id],
            )
            .await
            .unwrap();

        Ok(())
    }
}
