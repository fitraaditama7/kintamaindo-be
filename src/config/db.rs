use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use lazy_static::lazy_static;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
    pub static ref DB_POOL: DbPool = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder().build(manager).expect("Failed to create database pool.")
    };
}
