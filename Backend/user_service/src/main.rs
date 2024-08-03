#[macro_use] extern crate rocket;
extern crate diesel;

mod handlers;
mod routes;
mod guards;

use common::db::establish_connection;

/**
 * Lancer le serveur Rocket
 * @return le serveur Rocket
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 */
#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    let pool = establish_connection();
    rocket::build()
        .manage(pool)
        .mount("/api/v1", routes::routes())
}
// Compare this snippet from Bitbox/Backend/user_service/src/handlers.rs: