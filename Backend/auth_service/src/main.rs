#[macro_use] extern crate rocket;
extern crate diesel;

mod handlers;
mod utils;
mod models;
mod guard;
use std::env;
use rocket::fairing::AdHoc;
use crate::models::AuthConfig;
use rocket::{Build, Rocket};
use crate::models::{ LoginRequest, LoginResponse, RegisterRequest};
use domain::models::{Error, Response};




use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::register,
        handlers::login,
        handlers::token_confirm,
        handlers::logout,
        handlers::protected_route
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            LoginResponse,
        )
    ),
    tags(
        (name = "Bitbox  API", description = "Bitbox API endpoints.")
    )
)]
struct ApiDoc;



/**
* Lancer le serveur Rocket
* @return le serveur Rocket
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
*/
#[launch]
fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = common::db::establish_connection();
    rocket::build()
        .manage(pool)
        .manage(AuthConfig { jwt_secret })
        .mount(
            "/api/v1",
            routes![
                handlers::register,
                handlers::token_confirm,
                handlers::login,
                handlers::logout,
                handlers::protected_route,
            ],
        )
        .attach(AdHoc::try_on_ignite("OpenApi", |rocket| async {
            Ok(rocket.manage(ApiDoc::openapi()))
        }))


}


