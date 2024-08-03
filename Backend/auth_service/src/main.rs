#[macro_use] extern crate rocket;
extern crate diesel;

mod handlers;
mod utils;
mod models;
mod guard;
use std::env;
use rocket::fairing::AdHoc;
use crate::models::{AuthConfig, Jwks};
use rocket::{Build, Rocket};
use crate::models::{ LoginRequest, LoginResponse, RegisterRequest};





use utoipa::OpenApi;
use crate::utils::fetch_jwks;

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
 async fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_audience = env::var("JWT_AUDIENCE").expect("JWT_AUDIENCE must be set");
    let jwt_issuer = env::var("JWT_ISSUER").expect("JWT_ISSUER must be set");
    let jwks_url = env::var("JWKS_URL").expect("JWKS_URL must be set");

    let jwks = fetch_jwks(&jwks_url).await.expect("Failed to fetch JWKS");

    let pool = common::db::establish_connection();
    rocket::build()
        .manage(pool)
        .manage(AuthConfig {jwt_secret})
        //.manage(AuthConfi::new(jwks))
        .mount(
            "/auth",
            routes![
                handlers::register,
                handlers::token_confirm,
                handlers::login,
                handlers::logout,
                handlers::protected_route,
                handlers::create_student,
            ],
        )
     


}


