use application::Settings;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{prelude::*, r2d2};
use diesel_migrations::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/migrations");

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub async fn configure(settings: &Settings) -> anyhow::Result<DbPool> {
    let mut connection = PgConnection::establish(settings.database.database_url.as_str())?;
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let manager = ConnectionManager::<PgConnection>::new(settings.database.database_url.as_str());
    Ok(Pool::builder().build(manager)?)
}
