#[macro_use] extern crate rocket;
extern crate diesel;

mod handlers;
//mod routes;
mod utils;
mod models;




use rocket::{ Build, Rocket};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
//use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

/**
 * Lancer le serveur Rocket
 * @return le serveur Rocket
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 */
#[launch]
fn rocket() -> Rocket<Build> {

    //let figment = Figment::from(Config::default())
        //.merge(("openapi.title", "Auth Service"));

        //.merge(("tls", TlsConfig::from_paths("cert/cert.pem", "cert/key.pem")));

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect())
        //.allowed_headers(vec![rocket::http::Header::authorization(), rocket::http::Header::CONTENT_TYPE])
        .allow_credentials(true)
        .to_cors()
        .expect("error creating CORS fairing");

    dotenv::dotenv().ok();
    let pool = common::db::establish_connection();
    rocket::build()
        .manage(pool)
        .attach(cors)
        .mount(
            "/api/v1",
            routes![
                handlers::register,
                handlers::login,
                handlers::logout,
                handlers::verify_2fa,
                handlers::totp_setup,
                handlers::reset_password,
                handlers::forgot_password,

            ],
        )

}



// Compare this snippet from Bitbox/Backend/auth_service/src/handlers.rs: