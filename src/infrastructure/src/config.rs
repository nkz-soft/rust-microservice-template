use anyhow::{anyhow, Context};
use application::Settings;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{prelude::*, r2d2};
use diesel_migrations::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/migrations");

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub async fn configure(settings: &Settings) -> anyhow::Result<DbPool> {
    let database_url = settings.database.database_url.as_str();
    let mut connection =
        PgConnection::establish(database_url).context("failed to establish database connection")?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|err| anyhow!("failed to run database migrations: {err}"))?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .context("failed to create database connection pool")
}
