pub mod config;

pub use self::config::configure;
use diesel::{r2d2, PgConnection};

pub mod postgres_repositories;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

mod errors;
