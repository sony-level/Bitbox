use sha2::{Digest, Sha512};
use jsonwebtoken::{encode, Header, EncodingKey, Validation, TokenData, DecodingKey, decode};
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::models::{Claims, ClaimsType};
use std::{env, str};
use std::fmt::Write;
use base64::Engine;
use base64::engine::general_purpose;
use qrcodegen::{QrCode as QrCodeGen, QrCodeEcc};
use jsonwebtoken::errors::Error as JwtError;
use google_authenticator::GoogleAuthenticator;
use once_cell::sync::{ OnceCell};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use std::result::Result;
use lettre::{Message, SmtpTransport, Transport };
use lettre::transport::smtp::authentication::Credentials;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use lettre::message::{header, Mailbox, SinglePart};
use lettre::message::header::{ContentType};
use lettre::transport::smtp::client::{Tls, TlsParameters};
use native_tls::{TlsConnector, Protocol};
use domain::models::{NewUser, User};
use argon2::password_hash::Error as PasswordHashError;

static GA_AUTH: OnceCell<GoogleAuthenticator> = OnceCell::new();

/**
 * Hasher un mot de passe
 * @param password : le mot de passe à hasher
 * @return le mot de passe hashé
 */
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    password_hash
}

/**
 * Verifier un mot de passe
    * @param password : le mot de passe à vérifier
    * @param hash : le mot de passe hashé
    * @return le mot de passe hashé
 */
pub fn verify_password(password: &Option<String>, hash: &str) -> Result<bool, PasswordHashError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;

    if let Some(ref actual_password) = password {
        match argon2.verify_password(actual_password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    } else {
        Err(PasswordHashError::Password)
    }
}
/**
 * Générer un secret utilisateur
 * @param user_id : l'identifiant de l'utilisateur
 * @return le secret utilisateur
 */
fn generate_user_secret(id: Uuid) -> String {
    let mut hasher = Sha512::new();
    hasher.update(id.to_string().as_bytes());
    format!("{:x}", hasher.finalize())
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

fn go_auth() -> &'static GoogleAuthenticator { //
    GA_AUTH.get_or_init(|| {
        GoogleAuthenticator::new()
    })
}

/**
    * Générer un code OTP
    * @param user_id : l'identifiant de l'utilisateur
    * @return le code OTP
    */
pub fn generate_totp_secret(email: &str, id: Uuid) -> Result<(String, String ), String> {
    let sec = generate_user_secret(id);
    let secret = go_auth().create_secret(32);
    let account_name = utf8_percent_encode(email, NON_ALPHANUMERIC).to_string();
    let issuer_name = utf8_percent_encode(sec.as_str(), NON_ALPHANUMERIC).to_string(); // TDO: changer le nom de l'émetteur
    let uri = format!("otpauth://totp/{}?secret={}&issuer={}", account_name, secret, issuer_name);

    Ok((secret, uri))
}

/**
 * Générer un QR code pour le 2FA
 * @param uri : l'URI TOTP
 * @return l'image du QR code en base64
 */
pub fn generate_totp_qr_code(uri: &str) -> Result<String, String> {
    let qr = QrCodeGen::encode_text(uri, QrCodeEcc::High).map_err(|e| e.to_string())?;
    let svg = qr_to_svg_string(&qr, 4);
    let encoded = general_purpose::STANDARD.encode(svg);
    let result = format!("data:image/svg+xml;base64,{}", encoded);
    Ok(result)
}

/**
 * Convertir un QR code en une chaîne SVG
 * @param qr : le QR code à convertir
 * @param border : la taille de la bordure
 * @return la chaîne SVG
 */
pub fn qr_to_svg_string(qr: &QrCodeGen, border: i32) -> String {
    let mut result = String::new();
    write!(result, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n").unwrap();
    write!(result, "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {} {}\">\n",
           qr.size() + border * 2, qr.size() + border * 2).unwrap();
    write!(result, "<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n").unwrap();
    write!(result, "<path d=\"").unwrap();
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    write!(result, " ").unwrap();
                }
                write!(result, "M{},{}h1v1h-1z", x + border, y + border).unwrap();
            }
        }
    }
    write!(result, "\" fill=\"#000000\"/>\n").unwrap();
    write!(result, "</svg>\n").unwrap();
    result
}
/**
    * Vérifier un code OTP
    * @param secret : le secret
    * @param code : le code à vérifier
    * @return true si le code est valide, false sinon
    */
pub fn verify_totp_code(secret: &str, code: &str) -> bool {
    go_auth().verify_code(secret, code, 3, 0)
}

pub fn send_confirmation_email(email: &str, token: &str) -> Result<(), String> {
    dotenv::dotenv().ok();

    let smtp_username = env::var("SMTP_USERNAME").map_err(|e| e.to_string())?;
    let smtp_password = env::var("SMTP_PASSWORD").map_err(|e| e.to_string())?;
    let smtp_server = env::var("SMTP_SERVER").map_err(|e| e.to_string())?;
    let smtp_from_email = env::var("SMTP_FROM_EMAIL").map_err(|e| e.to_string())?;
    let smtp_port = env::var("SMTP_PORT").map_err(|e| e.to_string())?;

    let email_body = format!("Pour confirmer votre inscription, cliquez sur le lien suivant : https://localhost:3000/api/v1/confirm_registration?token={}", token);
    let message_id = format!("<{}@{}>", Uuid::new_v4(), smtp_server);
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

    let tls_connector = TlsConnector::builder()
        .min_protocol_version(Some(Protocol::Tlsv12))
        .build()
        .map_err(|e| e.to_string())?;

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

/**
 * Hasher un token avec Argon2
 * @param token : le token à hasher
 * @return le token hashé
 */
pub fn hash_token(token: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let token_hash = argon2.hash_password(token.as_bytes(), &salt).unwrap().to_string();
    token_hash
}
