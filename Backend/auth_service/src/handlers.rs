extern crate common;
extern crate domain;


use rocket::response::status::{Custom, NoContent};

use crate::models::{ClaimsType, ForgotPasswordRequest, LogoutRequest, ResetPasswordRequest, TotpSetupResponse, Verify2FARequest};
use domain::models::{User, NewUser, UserRole, NewAuthToken, AuthToken};
use domain::schema::*;
use diesel::prelude::*;
use rocket::http::Status;
use common::db::Pool;
use rocket::State;
use crate::models::{AuthenticatedUser, RegisterRequest, LoginRequest };
use crate::utils::{hash_password, generate_token, verify_password, generate_totp_secret, generate_totp_qr_code, validate_token, verify_totp_code, qr_to_svg_string, generate_reset_token, send_reset_email};
use chrono::Utc;
use diesel::delete;
use jsonwebtoken::{decode, DecodingKey, Validation};
use qrcodegen::{QrCode};
use rocket::response::status;
use rocket::serde::json::{Json, json};
use domain::schema::auth_tokens;
use serde_json::Value;
use base64::Engine;
use diesel::insert_into;
use uuid::Uuid;


/**
* connecter un utilisateur
* @param login_request : la demande de connexion
* @param pool : la connexion à la base de données
* @return l'utilisateur connecté
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
* $ curl -X POST http://localhost:8000/api/v1/register -H "Content-Type: application/json" -d '{"email": "test", "password": "test", "first_name": "test", "last_name": "test"}
*/
#[rocket::post("/register", format = "application/json", data = "<register_request>")]
pub fn register(register_request: Json<RegisterRequest>, pool: &State<Pool>) -> Result<Json<AuthenticatedUser>, Status> {

    if register_request.0.email.is_empty() || register_request.0.password.is_empty() || register_request.0.first_name.is_empty() || register_request.0.last_name.is_empty() {
        return Err(Status::BadRequest);
    }

    let (totp_secret, totp_uri) = match generate_totp_secret(&register_request.0.email, Uuid::new_v4()) {
        Ok(result) => result,
        Err(_) => {
            println!("Error generating TOTP secret");
            return Err(Status::InternalServerError);
        }
    };

    let totp_qr_code = match generate_totp_qr_code(&totp_uri) {
        Ok(result) => result,
        Err(_) => {
            println!("Error generating TOTP QR code");
            return Err(Status::InternalServerError);
        }
    };
    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            println!("Error getting database connection");
            return Err(Status::ServiceUnavailable);
        }
    };
    let hashed_password = hash_password(&register_request.0.password);

    let new_user = NewUser {
        email: &register_request.0.email,
        first_name: &register_request.0.first_name,
        last_name: &register_request.0.last_name,
        totp_secret: &totp_secret,
        password_hash: &hashed_password,
        role: UserRole::Student,
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn) {
        Ok(_) => {
            println!("New user created ");
            let user = users::table
                .filter(users::email.eq(&register_request.0.email))
                .first::<User>(&mut conn)
                .expect("Error finding user");

            let user_id = user.id;
            let token = generate_token(user.id);
            let expires_at = Utc::now().naive_utc() + chrono::Duration::days(2); // 2 JOUR avent espiration

            let new_auth_token = NewAuthToken {
                id: Uuid::new_v4(),
                user_id: Some(user_id),
                token: &token,
                created_at: Some(Utc::now().naive_utc()),
                expires_at,
            };
            match diesel::insert_into(auth_tokens::table)
                .values(&new_auth_token)
                .execute(&mut conn) {
                Ok(_) => println!("Auth token insert"),
                Err(e) => {
                    println!("Error saving auth token: {:?}", e);
                    return Err(Status::InternalServerError);
                }
            }

            Ok(Json(AuthenticatedUser {
                id: user_id,
                role: user.role,
                token,
                totp_qr_code: Some(totp_qr_code),
            }))
        },
        Err(e) => {
            println!("Error saving new user: {:?}", e);
            Err(Status::InternalServerError)
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
* $ curl -X POST http://localhost:8000/api/v1/login -H "Content-Type: application/json" -d '{"email": "test", "password": "test"}'
*/
#[rocket::post("/login", format = "application/json", data = "<login_request>")]
pub async fn login(login_request: Json<LoginRequest>, pool: &State<Pool>) -> Result<Json<AuthenticatedUser>, Status> {


    // Récupérer une connexion à la base de données depuis le pool
    let mut conn = pool.get().map_err(|_| Status::ServiceUnavailable)?;

    // Trouver l'utilisateur par email dans la base de données
    let user = users::table
        .filter(users::email.eq(&login_request.email))
        .first::<User>(&mut conn)
        .map_err(|_| Status::Unauthorized)?;

    // Si un token est fourni, le valider
    if let Some(ref token) = login_request.token {
        let valid_token = auth_tokens::table
            .filter(auth_tokens::token.eq(token))
            .filter(auth_tokens::user_id.eq(user.id))
            .first::<AuthToken>(&mut conn)
            .optional()
            .map_err(|_| Status::InternalServerError)?;

        if valid_token.is_some() {
            return Ok(Json(AuthenticatedUser {
                id: user.id,
                role: user.role,
                token: token.clone(),
                totp_qr_code: None,
            }));
        }
    }
//si  le token est expiere ou n'ex pas dem
    if let Some(password) = &login_request.password {
        if verify_password(password.to_string(), &user.password_hash) {

            let token = generate_token(user.id);
            let expires_at = Utc::now().naive_utc() + chrono::Duration::days(2); //date d'expiration du token
            let new_auth_token = NewAuthToken {
                id: Uuid::new_v4(),
                user_id: Some(user.id),
                token: &token,
                created_at: Some(Utc::now().naive_utc()),
                expires_at,
            };

            match diesel::insert_into(auth_tokens::table)
                .values(&new_auth_token)
                .execute(&mut conn) {
                Ok(_) => {
                    println!("Auth token inserted into database");
                },
                Err(e) => {
                    println!("Error saving auth token: {:?}", e);
                    return Err(Status::InternalServerError);
                }
            }

            return Ok(Json(AuthenticatedUser {
                id: user.id,
                role: user.role,
                token: token,
                totp_qr_code: None, // QR code nn requis lors de la connexion
            }));
        }
    }

    Err(Status::Unauthorized)
}


/**
* se deconnecter
* @param pool : la connexion à la base de données
* @return statut de la deconnexion
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
*$ curl -X POST http://localhost:8000/api/v1/logout -H "Content-Type: application/json" -d '{"token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkMzIwZjQzZi1mZjQwLTQwZjYtYjIwZi1mZjQwZjYtYjIwZi1mZjQwZjYtYjIwZiIsImV4cCI6MTYyNjQwNjYwMn0.1Z6Z9J
*/
#[post("/logout", format = "application/json", data = "<request>")]
pub fn logout(request: Json<LogoutRequest>, pool: &State<Pool>) -> Result<NoContent, Custom<Value>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Service unavailable"})))?;

    delete(auth_tokens::table.filter(auth_tokens::token.eq(&request.token)))
        .execute(&mut conn)
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!({"error": "Failed to delete token"})))
}

/**
* Mot de passse oublié
* @param forgot_password_request : la demande de mot de passse oublié
* @param pool : la connexion à la base de données
* @return statut de la demande
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
*$ curl -X POST http://localhost:8000/api/v1/forgot_password -H "Content-Type: application/json" -d '{"email": "test"}'

*/
#[post("/forgot_password", format = "application/json", data = "<request>")]
pub fn forgot_password(request: Json<ForgotPasswordRequest>, pool: &State<Pool>) -> Result<status::NoContent, Custom<Value>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, json!({"error": "Service unavailable"})))?;

    let user = users::table
        .filter(users::email.eq(&request.email))
        .first::<User>(&mut conn)
        .map_err(|_| Custom(rocket::http::Status::NotFound, json!({"error": "Email not found"})))?;

    let reset_token = generate_reset_token(user.id);

    send_reset_email(&user.email, &reset_token)
        .map_err(|e| Custom(rocket::http::Status::InternalServerError, json!({"error": e})))?;

    Ok(status::NoContent)
}

/**
 * Vérifier le code TOTP
 * @param verify_2fa_request : la demande de vérification 2FA
 * @param pool : la connexion à la base de données
 * @return statut de la vérification
 * @throws Unauthorized si le code TOTP est invalide
 * @throws BadRequest si l'identifiant de l'utilisateur est manquant
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* $ curl -X POST http://localhost:8000/api/v1/2fa/verify -H "Content-Type: application/json" -d '{"user_id": "d320f43f-ff40-40f6-b20f-ff40f6b20f6f", "totp_code": "123456"}'
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

/**
 * Setup TOTP
 * @param authenticated_user : l'utilisateur authentifier
 * @param pool : la connexion à la base de données
 * @return statut de la setup
 * @throws Unauthorized si l'utilisateur n'est pas authentifier
 * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
 * @see establish_connection
 * $ curl -X GET http://localhost:8000/api/v1/2fa/setup
*/
#[post("/2FA/setup", format = "application/json", data = "<authenticated_user>")]
pub fn totp_setup(authenticated_user: Json<AuthenticatedUser>, pool: &rocket::State<Pool>) -> Result<Json<TotpSetupResponse>, Status> {
    let mut conn = pool.get().map_err(|_| Status::ServiceUnavailable)?;

    let user = users::table
        .find(authenticated_user.id)
        .first::<User>(&mut conn)
        .map_err(|_| rocket::http::Status::NotFound)?;

    let email = &user.email;
    let user_id = user.id;

    let (totp_secret, uri) = generate_totp_secret(email, user_id)
        .map_err(|_| rocket::http::Status::InternalServerError)?;

    let qr_code_result = QrCode::encode_text(&uri, qrcodegen::QrCodeEcc::High)
        .map_err(|_| rocket::http::Status::InternalServerError)?;

    let qr_svg = qr_to_svg_string(&qr_code_result, 4);

    let engine = base64::engine::general_purpose::STANDARD;
    let qr_base64 = engine.encode(&qr_svg);

    Ok(Json(TotpSetupResponse {
        secret: totp_secret,
        uri,
        qr_code: qr_base64,
    }))
}


/**
* renitialiser le mot de passe
* @param request : la demande de renitialisation du mot de passe
* @param pool : la connexion à la base de données
* @return statut de la renitialisation
* @throws Unauthorized si le code TOTP est invalide
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* $ curl -X POST http://localhost:8000/api/v1/reset_password -H "Content-Type: application/json" -d '{"user_id": "d320f43f-ff40-40f6-b20f-ff40f6b20f6f", "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkMzIwZjQzZi1mZjQwLTQwZjYtYjIwZi1mZjQwZjYtYjIwZi1mZjQwZjYtYjIwZiIsImV4cCI6MTYyNjQwNjYwMn0.1Z6Z9J", "new_password": "test"}'

*/

#[post("/reset_password", format = "application/json", data = "<request>")]
pub fn reset_password(request: Json<ResetPasswordRequest>, pool: &State<Pool>) -> Result<NoContent, Status> {
    let mut conn= pool.get().map_err(|_| Status::ServiceUnavailable)?;

    let reset_token = generate_reset_token(request.user_id);

    let token_data = decode::<ClaimsType>(&request.token,
                                          &DecodingKey::from_secret(reset_token.as_ref()),
                                          &Validation::default())
        .map_err(|_| Status::Unauthorized)?;

    let new_password_hash = hash_password(&request.new_password);

    diesel::update(users::table.filter(users::id.eq(token_data.claims.sub)))
        .set(users::password_hash.eq(new_password_hash))
        .execute(&mut conn)
        .map(|_| NoContent)
        .map_err(|_| Status::InternalServerError)
}