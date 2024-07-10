extern crate common;
extern crate domain;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
//use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use domain::models::UserRole;
//use domain::schema::auth_tokens::user_id;

/**
 * Représente un utilisateur
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
   // pub totp_secret: String,
   // pub role: UserRole,
}

/**
    * Représente une demande de connexion
    */
#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password:  Option<String>,
    pub token: Option<String>
}


/**
    *
    */
#[derive(Deserialize, Serialize)]
pub struct ClaimsType {
    pub sub: Uuid,
    pub exp: usize,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub role: UserRole,
    pub token: String,
    pub totp_qr_code: Option<String>,

}
#[derive(Deserialize , Serialize)]
pub struct Verify2FARequest {
    pub user_id: Option<Uuid>,
    pub totp_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Deserialize, Serialize)]
pub struct TotpSetupResponse {
    pub secret: String,
    pub uri: String,
    pub qr_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct LogoutRequest {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
    pub user_id: Uuid,
}