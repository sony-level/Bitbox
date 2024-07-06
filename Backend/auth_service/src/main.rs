#[macro_use] extern crate rocket;
extern crate diesel;

mod handlers;
mod routes;
mod utils;
mod models;

//use common::db::Pool;
//use common::db::establish_connection;


use rocket::figment::Figment;
use rocket::config::{Config};
use rocket::{ Build, Rocket};
use rocket::http::Method;
//use rocket::form::FromForm;
//use rocket::time::format_description::well_known::iso8601::Config;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
//use rocket_okapi::{openapi_get_routes};
//use crate::handlers::register;

/**
 * Lancer le serveur Rocket
 * @return le serveur Rocket
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 */
#[launch]
fn rocket() -> Rocket<Build> {

    let figment = Figment::from(Config::default())
        .merge(("openapi.title", "Auth Service"));

        //.merge(("tls", TlsConfig::from_paths("/path/to/your/cert.pem", "/path/to/your/key.pem")));

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect())
        //.allowed_headers(vec![rocket::http::Header::authorization(), rocket::http::Header::CONTENT_TYPE])
        .allow_credentials(true)
        .to_cors()
        .expect("error creating CORS fairing");

    dotenv::dotenv().ok();
    let pool = common::db::establish_connection();
    rocket::custom(figment)
        .manage(pool)
        .mount(
            "/api/v1",
            routes![
                handlers::register, //handlers::register,
                handlers::login,
                handlers::verify_2fa,
                //handlers::admin_route,
                //handlers::get_user,
            ],
        )
        .mount("/swagger", make_swagger_ui(&SwaggerUIConfig {
            url: "/openapi.json".to_string(),
            ..Default::default()
        }))
        .attach(cors)

}


// Compare this snippet from Bitbox/Backend/auth_service/src/handlers.rs: