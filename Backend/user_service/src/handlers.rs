

extern crate common;
extern crate domain;

//use utoipa::{OpenApi, ToSchema};
use rocket::{put, get, post};
use serde_json::json;
use domain::schema::users::dsl::users;
use diesel::prelude::*;
use rocket::State;
use sha2::{Sha512 , Digest};
use chrono::Utc;
use uuid::Uuid;
use common::db::Pool;
use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::{
    self,
    serde::{json::Json, json::Value as JsonValue} ,
 };
use domain::models::{User, NewUser , UserDisplay , UpdateUser };




/**
 * Hasherle mdp
 * @param password : le mot de passe à hasher
 * @return le mot de passe hashé
 */
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let hash_result = hasher.finalize();
    let hex_string = hash_result.iter()
        .map(|byte| format!("{:056x}", byte))
        .collect::<Vec<String>>()
        .join("");
    hex_string
}

/**
 * Créer un nouvel utilisateur
 * @param new_user : le nouvel utilisateur
    * @param pool : la connexion à la base de données
    * @return le nouvel utilisateur créé
    * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 */

#[post("/users", format = "application/json", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUser>, pool: &State<Pool>) -> Result<status::Custom<Json<User>>, status::Custom<JsonValue>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Database connection error"})))?;
    
    let hashed_password = hash_password(&new_user.password_hash);
    let new_user = NewUser {
        password_hash: &hashed_password,
        t
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
        ..new_user.into_inner()
    };
    match diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn) {
            Ok(user) => Ok(Custom(Status::Created, Json(user))),
            Err(err) => Err(Custom(
                Status::InternalServerError,
                json!({ "error": err.to_string() })

            )),
    }
}

/**
 * Récupérer tous les utilisateurs
 * @param pool : la connexion à la base de données
 * @return la liste des utilisateurs
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 * @see users
 */

#[get("/users")]
pub fn get_users(pool: &State<Pool>) -> Result<status::Custom<Json<Vec<UserDisplay>>>, status::Custom<JsonValue>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Database connection error"})))?;
    match users.load::<User>(&mut conn) {
        Ok(users_list) => { //
            let display_users: Vec<UserDisplay> = users_list.into_iter().map(|user| user.into()).collect(); // convertir les utilisateurs  en JSON pour les afficher dans l'API
            Ok(Custom(Status::Ok, Json(display_users))) // renvoyer la liste des utilisateurs sous forme de JSON
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
 * @param pool : la connexion à la base de données
 * @return l'utilisateur correspondant à l'identifiant
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 * @see users
 */

#[get("/users/<id>")]
pub fn get_users_by_id(id: Uuid, pool: &State<Pool>) -> Result<status::Custom<Json<UserDisplay>>, status::Custom<JsonValue>> {
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
 * @param id : l'identifiant de l'utilisateur
 * @param user : les informations de l'utilisateur à mettre à jour
 * @param pool : la connexion à la base de données
 * @return l'utilisateur mis à jour
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 * @see users
 */

#[put("/users/<id>", format = "application/json", data = "<user>")]
pub fn update_user(id: Uuid, user: Json<UpdateUser>, pool: &State<Pool>) -> Result<status::Custom<Json<User>>, status::Custom<JsonValue>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Database connection error"})))?;
    let user = user.into_inner();
    let user = diesel::update(users.find(id))
        .set(&user)
        .get_result::<User>(&mut conn)
        .map_err(|err| Custom(
            Status::InternalServerError,
            json!({ "error": err.to_string() })
        ))?;
    Ok(Custom(Status::Ok, Json(user)))
}

//

#[get("/user")]
pub fn index() -> &'static str {
    "Hello, world!"
}


