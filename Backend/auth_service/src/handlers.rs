extern crate common;
extern crate domain;

use crate::models::Verify2FARequest;
use domain::models::{User, NewUser};
use domain::schema::*;
use diesel::prelude::*;
use rocket::http::Status;
use common::db::Pool;
use rocket::State;
use crate::models::{AuthenticatedUser, RegisterRequest, LoginRequest };
use crate::utils::{hash_password, generate_token, verify_password, generate_totp_secret, generate_totp_qr_code, validate_token, verify_totp_code};
use chrono::Utc;
use rocket::serde::json::Json;
use uuid::Uuid;



/**
* connecter un utilisateur
* @param login_request : la demande de connexion
* @param pool : la connexion à la base de données
* @return l'utilisateur connecté
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
*/
#[rocket::post("/register", format = "application/json", data = "<register_request>")]
pub fn register(register_request: Json<RegisterRequest>, pool: &State<Pool>) -> Result<Json<AuthenticatedUser>, Status> {
    //let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable.to_string()));
    let (totp_secret, totp_uri) = generate_totp_secret(&register_request.0.email, Uuid::new_v4()).map_err(|_| Status::InternalServerError)?;
    let totp_qr_code = generate_totp_qr_code(&totp_uri).map_err(|_| Status::InternalServerError)?;
    let mut conn = pool.get().map_err(|_| Status::ServiceUnavailable)?;
    let hashed_password = hash_password(&register_request.0.password);

    let new_user = NewUser {
        email: &register_request.0.email,
        first_name: &register_request.0.first_name,
        last_name: &register_request.0.last_name,
        totp_secret: &totp_secret,
        password_hash: &hashed_password,
        role: register_request.0.role,
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };
    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn) {
            Ok(_) =>  { let user = users::table
                .filter(users::email.eq(&register_request.0.email))
                .first::<User>(&mut conn)
                .expect("Error finding user");
                let user_id = user.id;
                let token = generate_token(user.id);
                Ok(Json(AuthenticatedUser {
                    id: user_id,
                    role: user.role,
                    token : token,
                    totp_qr_code: Some(totp_qr_code),
                }))
            },
            Err(e) => {
                println!("Error saving new user: {:?}", e);
                return Err(Status::InternalServerError);
            }
    }
}

/**
* connecter un utilisateur
* @param login_request : la demande de connexion
* @param pool : la connexion à la base de données
* @return l'utilisateur connecté
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
*/
#[rocket::post("/login", format = "application/json", data = "<login_request>")]
pub async fn login(login_request: Json<LoginRequest>, pool: &State<Pool>) -> Result<Json<AuthenticatedUser>, Status> {
    let mut conn = pool.get().map_err(|_| Status::ServiceUnavailable)?;

    let user = users::table
        .filter(users::email.eq(&login_request.email))
        .first::<User>(&mut conn)
        .map_err(|_| Status::Unauthorized)?;

    if let Some(ref token) = login_request.token {
        if validate_token(token, user.id).is_ok() {
            return Ok(Json(AuthenticatedUser {
                id: user.id,
                role: user.role,
                token: token.clone(),
                totp_qr_code: None, // Pas de QR code lors de la connexion
            }));
        }
    }
    if let Some(password) = &login_request.password {
        if verify_password(password.to_string(), &user.password_hash) {
            let token = generate_token(user.id);
            return Ok(Json(AuthenticatedUser {
                id: user.id,
                role: user.role,
                token: token,
                totp_qr_code: None, // Pas de QR code lors de la connexion
            }));
        }
    }
    Err(Status::Unauthorized)
}

/**
 * Vérifier le code TOTP
 * @param verify_2fa_request : la demande de vérification 2FA
 * @param pool : la connexion à la base de données
 * @return statut de la vérification
 * @throws Unauthorized si le code TOTP est invalide
 */
#[rocket::post("/2fa/verify", format = "application/json", data = "<verify_2fa_request>")]
pub async fn verify_2fa(verify_2fa_request: Json<Verify2FARequest>, pool: &State<Pool>) -> Result<Status, Status> {
    let mut conn = pool.get().map_err(|_| Status::ServiceUnavailable)?;
    if let Some(user_id) = verify_2fa_request.user_id {
        let user = users::table
            .filter(users::id.eq(user_id))
            .first::<User>(&mut conn)
            .map_err(|_| Status::Unauthorized)?;
        if verify_totp_code(&user.totp_secret, &verify_2fa_request.totp_code) {
            Ok(Status::Ok)
        } else {
            Err(Status::Unauthorized)
        }
    } else {
        Err(Status::BadRequest)
    }
}


