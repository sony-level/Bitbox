extern crate common;
extern crate domain;

use diesel::{ Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use domain::models::UserRole;


/**
 * Représente un utilisateur
 */
#[derive(Deserialize, Serialize, Debug , ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: Uuid,
}
/**
    * Représente une demande de connexion
    */
#[derive(Deserialize, Serialize , ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password:  String,

}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    token: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClaimsType {
    pub iat: usize,
    pub sub: Uuid,
    pub exp: usize,
    pub email: String,
    pub role: UserRole,
}

#[derive(Queryable, Debug, Serialize, Deserialize ,ToSchema)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub role: UserRole,
    pub token: String,
    pub totp_qr_code: Option<String>,

}
#[derive(Deserialize , Serialize, ToSchema)]
pub struct Verify2FARequest {
    pub user_id: Option<Uuid>,
    pub totp_code: String,
}

#[derive(Deserialize, Serialize , ToSchema)]
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

#[derive(Deserialize, Serialize , ToSchema)]
pub struct LogoutRequest {
    pub  user_id : String,
}

#[derive(Deserialize, Serialize , ToSchema)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize, Serialize , ToSchema)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateStudentRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwk {
    pub kty: String,
    pub r#use: String,
    pub kid: String,
    pub x5c: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,

}