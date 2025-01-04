mod config;
mod errors;
mod postgres_repositories;

use diesel::{r2d2, PgConnection};
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub use config::configure;
pub use errors::Error;
pub use postgres_repositories::PostgresToDoItemRepository;
