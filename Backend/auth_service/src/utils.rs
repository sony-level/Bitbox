use std::{env, str};
use std::error::Error;
use std::result::Result;
use argon2::{self , Config};
use base64::Engine;
//use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use uuid::Uuid;
use rand::rngs::OsRng;
use rand::{ RngCore};
use domain::models::{NewUser, User};
use reqwest::Client;
use crate::models::{Claims, ClaimsType, Jwk, Jwks};


/**
 * Hasher un mot de passe
 * @param password : le mot de passe à hasher
 * @return le mot de passe hashé
 */
pub fn hash_password(password: &str) -> String {
    let mut salt = [0u8; 32];
    OsRng.try_fill_bytes(&mut salt).expect("Failed to generate salt");
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}
/**
 * Verifier un mot de passe
    * @param password : le mot de passe à vérifier
    * @param hash : le mot de passe hashé
    * @return le mot de passe hashé
 */
pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, Box<dyn Error>> {
    let is_valid = argon2::verify_encoded(hashed_password, password.as_bytes())?;
    Ok(is_valid)
}
pub fn generate_jwt(user_id: Uuid, secret: &str, expiration_minutes: i64) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(expiration_minutes))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}

/**
    * Générer un token d'authentification
    * @param user_id : l'identifiant de l'utilisateur
    * @return le token d'authentification
    */
pub fn generate_token(user: &NewUser) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    let claims = ClaimsType {
        iat: Utc::now().timestamp() as usize,
        sub: Some(user.id).unwrap(),
        exp: expiration as usize,
        email: user.email.parse().unwrap(),
        role: user.role.clone(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(user.jwt_secret.as_ref())).unwrap()
}

pub fn generat_token(user: &User) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    let claims = ClaimsType {
        iat: Utc::now().timestamp() as usize,
        sub: Some(user.id).unwrap(),
        exp: expiration as usize,
        email: user.email.clone().parse().unwrap(),
        role: user.role.clone(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(user.jwt_secret.as_ref())).unwrap()
}

/**
    * verifier le token d'authentification
    * @param token : le token d'authentification
    * @return l'identifiant de l'utilisateur
 */
pub fn decode_token(token: &str, secret: &str) -> Result<TokenData<ClaimsType>, JwtError> {
    decode::<ClaimsType>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )
}

pub fn decode_jwt_no_secret(token: &str) -> Result<ClaimsType, jsonwebtoken::errors::Error> {
    let mut validation = jsonwebtoken::Validation::default();
    validation.insecure_disable_signature_validation();
    let token_data = jsonwebtoken::decode::<ClaimsType>(&token, &jsonwebtoken::DecodingKey::from_secret(b""), &validation)?;
    Ok(token_data.claims)
}


pub fn send_confirmation_email(email: &str, token: &str) -> Result<(), String> {
    dotenv::dotenv().ok();

    let smtp_username = env::var("SMTP_USERNAME").map_err(|e| e.to_string())?;
    let smtp_password = env::var("SMTP_PASSWORD").map_err(|e| e.to_string())?;
    let smtp_server = env::var("SMTP_SERVER").map_err(|e| e.to_string())?;
    let smtp_from_email = env::var("SMTP_FROM_EMAIL").map_err(|e| e.to_string())?;
    let smtp_port = env::var("SMTP_PORT").map_err(|e| e.to_string())?;

    let email_body = format!("Pour confirmer votre inscription, cliquez sur le lien suivant : https://localhost:8000/auth/confirm_registration?token={}", token);
    //let message_id = format!("<{}@{}>", Uuid::new_v4(), smtp_server);
    //let message_id_header = lettre::message::header::MessageId::new(message_id);

    let  email = Message::builder()
        .from(smtp_from_email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Confirmation d'inscription")
        .header(ContentType::TEXT_PLAIN)
        //.header(MessageId::new(message_id))
        .singlepart(SinglePart::plain(email_body))
        .map_err(|e| e.to_string())?;

    let creds = Credentials::new(smtp_username.clone(), smtp_password.clone());

    let tls_parameters = TlsParameters::builder(smtp_server.clone())
        .build_native()
        .map_err(|e| e.to_string())?;

    let mailer = SmtpTransport::relay(&smtp_server)
        .map_err(|e| e.to_string())?
        .port(smtp_port.parse().unwrap())
        .tls(Tls::Required(tls_parameters))
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(|e| e.to_string())?;
    Ok(())
}


pub async fn fetch_jwks(jwks_url: &str) -> Result<Jwks, reqwest::Error> {
    let client = Client::new();
    let res = client.get(jwks_url).send().await?;
    let jwks = res.json::<Jwks>().await?;
    Ok(jwks)
}


pub fn find_jwk<'a>(jwks: &'a Jwks, kid: &str) -> Option<&'a Jwk> {
    jwks.keys.iter().find(|key| key.kid == kid)
}


pub fn decod_jwt(token: &str, jwks: &Jwks) -> Result<TokenData<ClaimsType>, JwtError> {
    let header = jsonwebtoken::decode_header(token).unwrap();
    let kid = match header.kid {
        Some(k) => k,
        None => return Err(JwtError::from(ErrorKind::InvalidToken))
    };

    let jwk = find_jwk(jwks, &kid).ok_or(JwtError::from(ErrorKind::InvalidToken))?;

    let x5c = &jwk.x5c[0];
    let der = base64::engine::general_purpose::STANDARD.decode(&x5c).map_err(|_| JwtError::from(ErrorKind::InvalidToken))?;
    let pem = format!(
        "-----BEGIN CERTIFICATE-----\n{:?}\n-----END CERTIFICATE-----",
        base64::engine::general_purpose::STANDARD.encode(&der)
    );
    let decoding_key = DecodingKey::from_rsa_pem(pem.as_bytes()).unwrap();
    decode::<ClaimsType>(token, &decoding_key, &Validation::new(jsonwebtoken::Algorithm::RS256))
}

pub fn is_password_strong(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    has_uppercase && has_lowercase && has_digit && has_special
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "mysecretpassword";
        let hashed = hash_password(password);
        assert!(!hashed.is_empty(), "Hashed password should not be empty");
    }

    #[test]
    fn test_verify_password() {
        let password = "mysecretpassword";
        let hashed = hash_password(password);
        let is_valid = verify_password(password, &hashed);
        assert!(is_valid, "Password verification should succeed");

        let wrong_password = "wrongpassword";
        let is_valid = verify_password(wrong_password, &hashed);
        assert!(!is_valid, "Password verification should fail for wrong password");
    }
}