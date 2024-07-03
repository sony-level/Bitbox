
extern crate diesel;
extern crate dotenv;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

// Alias pour le pool de connexions
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Initialiser le pool de connexions
pub fn init_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// Établir la connexion à la base de données
pub fn establish_connection() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
