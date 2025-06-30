use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

// Alias tipe pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(80) // Atur sesuai kapasitas server (contoh: 15 koneksi aktif)
        .build(manager)
        .expect("Failed to create DB connection pool")
}



