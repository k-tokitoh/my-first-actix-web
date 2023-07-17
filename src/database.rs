use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn get_pool() -> DbPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
