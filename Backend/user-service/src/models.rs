use actix_web::{App, HttpServer, web};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::handlers::delete_user;
use crate::handlers::update_user;
use crate::handlers::get_user;
use crate::handlers::create_user;
mod handlers;
mod models;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::post().to(handlers::create_user))
            .route("/users/{id}", web::get().to(handlers::get_user))
            .route("/users/{id}", web::put().to(handlers::update_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
