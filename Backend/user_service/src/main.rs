

#[macro_use] extern crate rocket;
extern crate diesel;


use domain::models::{UserDisplay, UpdateUser, LogResponse, Response };
use user_service::handlers::*;
use utoipa_swagger_ui::SwaggerUi;
use common::db::establish_connection;
use user_service::routes;
use rocket::serde::json::Json;
use utoipa::OpenApi;
use user_service::guards::SecurityAddon;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_checker_handler,
        get_users,
        get_users_by_id,
        update_user,
        delete_user,
    ),
    components(schemas(Response, UserDisplay, UpdateUser, LogResponse)),
    tags(
        (name = "User Service API", description = "API pour la gestion des utilisateurs" ),
    ),
    info(
        title = "User Service API",
        version = "0.1.0",
        description = "API pour la gestion des utilisateurs",
        contact(name = "Bitbox Projet", email = "contact@level.ovh"),
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

#[get("/openapi.json")]
fn openapi_handler() -> Json<String> {
    Json(serde_json::to_string(&ApiDoc::openapi()).unwrap())
}
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
        .mount("/api", routes::routes())
        .mount("/api/doc/", routes![openapi_handler])
        .attach(rocket::fairing::AdHoc::on_ignite("OpenAPI Documentation", |rocket| {
            Box::pin(async move {
                let openapi = ApiDoc::openapi();
                rocket.mount("/", SwaggerUi::new("/api/docs/swagger-ui").url("/api/docs/openapi.json", openapi.clone()))
            })
        }))
}
// Compare this snippet from Bitbox/Backend/user_service/src/handlers.rs: