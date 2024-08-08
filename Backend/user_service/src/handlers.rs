

extern crate common;
extern crate domain;
extern crate auth_service;


use serde_json::json;
use domain::schema::users::dsl::users;
use diesel::prelude::*;
use rocket::State;
use uuid::Uuid;
use common::db::Pool;
use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::{
    self,
    serde::{json::Json, json::Value as JsonValue} ,
 };
use domain::models::{User, UserDisplay, UpdateUser, Response, LogResponse};
use crate::guards::TrainerGuard;




#[utoipa::path(
    get,
    path = "/api/health",
    tag = "Health Checker Endpoint",
    responses(
        (status = 200, description = "Authenticated User", body = Response),
        (status = 401, description = "Unauthorized"),
    )
)]
#[rocket::get("/health")]
pub fn health_checker_handler() -> Json<LogResponse> {
    Json(LogResponse {
        status: "success",
        message: "Complete Restful Bitbox API in Rust is working !!".to_string(),
    })
}

/**
 * Récupérer tous les utilisateurs
 * @param pool : la connexion à la base de données
 * Accessible uniquement par un Trainer
 * @return la liste des utilisateurs
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 * @see users
* $ curl -X GET"http://localhost:8000/users" \
-H "Authorization: Bearer <token>" \
-H "Content-Type: application/json"
 */
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Liste des utilisateurs récupérée avec succès", body = [UserDisplay]),
        (status = 401, description = "Non autorisé"),
        (status = 503, description = "Erreur de connexion à la base de données")
    ),
    security(("token" = []))
)]
#[rocket::get("/users")]
pub fn get_users(_trainer: TrainerGuard, pool: &State<Pool>) -> Result<status::Custom<Json<Vec<UserDisplay>>>, status::Custom<JsonValue>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Database connection error"})))?;
    match users.load::<User>(&mut conn) {
        Ok(users_list) => {
            let display_users: Vec<UserDisplay> = users_list.into_iter().map(|user| user.into()).collect();
            Ok(Custom(Status::Ok, Json(display_users)))
        },
        Err(err) => Err(Custom(
            Status::InternalServerError,
            json!({ "error": err.to_string() })
        )),
    }
}

/**
 * Récupérer un utilisateur par son identifiant
 * @param id : l'identifiant de l'utilisateur
 * Accessible uniquement par un Trainer
 * @param pool : la connexion à la base de données
 * @return l'utilisateur correspondant à l'identifiant
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 * @see users
* $ curl -X GET "http://localhost:8000/users/<id>" \
-H "Authorization: Bearer <token>" \
-H "Content-Type: application/json"
 */
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = Uuid, description = "Identifiant de l'utilisateur à récupérer")
    ),
    responses(
        (status = 200, description = "Utilisateur récupéré avec succès", body = UserDisplay),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Utilisateur non trouvé"),
        (status = 503, description = "Erreur de connexion à la base de données")
    ),
    security(("token" = []))
)]
#[rocket::get("/users/<id>")]
pub fn get_users_by_id(_trainer: TrainerGuard, id: Uuid, pool: &State<Pool>) -> Result<status::Custom<Json<UserDisplay>>, status::Custom<JsonValue>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Database connection error"})))?;
    match users.find(id).first::<User>(&mut conn) {
        Ok(user) => {
            let user_display: UserDisplay= user.into();
            Ok(Custom(Status::Ok, Json(user_display)))
        },
        Err(err) => Err(Custom(
            Status::InternalServerError,
            json!({ "error": err.to_string() })
        )),
    }
}

/**
 * Mettre à jour un utilisateur
 * Accessible uniquement par un Trainer
 * @param id : l'identifiant de l'utilisateur
 * @param user : les informations de l'utilisateur à mettre à jour
 * @param pool : la connexion à la base de données
 * @return l'utilisateur mis à jour
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 * @see users
* $ curl -X PUT "http://localhost:8000/users/<id>" \
-H "Authorization: Bearer <token>" \
-H "Content-Type: application/json" \
-d '{"email": "test"}'
 */
#[utoipa::path(
    put,
    path = "/users/{id}",
    request_body = UpdateUser,
    params(
    ("id" = Uuid, description = "Identifiant de l'utilisateur à mettre à jour")
    ),
    responses(
    (status = 200, description = "Utilisateur mis à jour avec succès", body = Response),
    (status = 401, description = "Non autorisé"),
    (status = 500, description = "Erreur de connexion à la base de données")
    ),
    security(("token" = []))
)]
#[rocket::put("/users/<id>", format = "application/json", data = "<user>")]
pub fn update_user(_trainer: TrainerGuard, id: Uuid, user: Json<UpdateUser>, pool: &State<Pool> ) -> Result<status::Custom<Json<Response>>, status::Custom<JsonValue>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Database connection error"})))?;
    let user = user.into_inner();
    let _user = diesel::update(users.find(id))
        .set(&user)
        .get_result::<User>(&mut conn)
        .map_err(|err| Custom(
            Status::InternalServerError,
            json!({ "error": err.to_string() })
        ))?;
    Ok(Custom(Status::Ok, Json(Response { message: "User updated successfully".into() })))
}



/**
 * Supprimer un utilisateur par son identifiant
 * Accessible uniquement par un Trainer
 * @param id : l'identifiant de l'utilisateur
 * @param pool : la connexion à la base de données
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* $ curl -X DELETE "http://localhost:8000/users/<id>" \
-H "Authorization: Bearer <token>" \
-H "Content-Type: application/json"
 */
#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = Uuid, description = "Identifiant de l'utilisateur à supprimer")
    ),
    responses(
        (status = 204, description = "Utilisateur supprimé avec succès"),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Utilisateur non trouvé"),
        (status = 500, description = "Erreur de connexion à la base de données")
    ),
    security(("token" = []))
)]
#[rocket::delete("/users/<id>") ]
pub fn delete_user(_trainer: TrainerGuard, id: Uuid, pool: &State<Pool>) -> Result<Json<Response>, status::Custom<JsonValue>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Database connection error"})))?;
    match diesel::delete(users.find(id)).execute(&mut conn) {
        Ok(0) => Err(Custom(Status::NotFound, json!({"error": "User not found"}))),
        Ok(_) => Ok(Json(Response { message: "User deleted successfully".into() })),
        Err(err) => Err(Custom(
            Status::InternalServerError,
            json!({ "error": err.to_string() })
        )),
    }
}