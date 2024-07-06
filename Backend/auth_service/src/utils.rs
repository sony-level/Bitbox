use sha2::{Digest, Sha512};
use jsonwebtoken::{encode, Header, EncodingKey, Validation, TokenData, DecodingKey, decode};
use uuid::Uuid;
use chrono::Utc;
use crate::models:: ClaimsType;
use std::str;
use std::fmt::Write;
use base64::Engine;
use base64::engine::general_purpose;
//use dotenv::dotenv;
//use std::env;
//use std::time::{SystemTime, UNIX_EPOCH};
//use otpauth::HOTP;
//use rocket::futures::TryFutureExt;
use qrcodegen::{QrCode, QrCodeEcc};
//use serde::{Deserialize, Serialize};
use jsonwebtoken::errors::Error as JwtError;
use google_authenticator::GoogleAuthenticator;
use once_cell::sync::{ OnceCell};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
//use domain::models::auth_tokens::user_id;


static GA_AUTH: OnceCell<GoogleAuthenticator> = OnceCell::new();

/**
 * Hasher un mot de passe
 * @param password : le mot de passe à hasher
 * @return le mot de passe hashé
 */
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

/**
 * Verifier un mot de passe
    * @param password : le mot de passe à vérifier
    * @param hash : le mot de passe hashé
    * @return le mot de passe hashé
 */
pub fn verify_password(password: String, hash: &str) -> bool {
    hash_password(&password) == hash
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

/**
    * Générer un token d'authentification
    * @param user_id : l'identifiant de l'utilisateur
    * @return le token d'authentification
    */
pub fn generate_token(id: Uuid )-> String {
    let expiration = Utc::now().timestamp() + 60 * 60 * 24; // 24 heures d'expiration
    let void = ClaimsType {
        sub: id,
        exp: expiration as usize,
    };
    let user_secret = generate_user_secret(id);
    let token = encode(
        &Header::default(),
        &void,
        &EncodingKey::from_secret(user_secret.as_ref()), // TDO: changer le secret , le mettre dans un fichier de configuration ou une foinction pour generer un secret
    )
    .unwrap();
    token
}

/*
    * verifier le token d'authentification
    * @param token : le token d'authentification
    * @return l'identifiant de l'utilisateur
 */
pub fn validate_token(token: &str , id: Uuid) -> Result<TokenData<ClaimsType>, JwtError> {
    let user_secret = generate_user_secret(id);
    let decoding_key = DecodingKey::from_secret(user_secret.as_ref()); // Assurez-vous que ce secret correspond à celui utilisé pour générer les tokens
    decode::<ClaimsType>(token, &decoding_key, &Validation::default())
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
pub fn generate_totp_secret(email: &str, _id: Uuid) -> Result<(String, String), String> {
    let secret = go_auth().create_secret(32);
    let account_name = utf8_percent_encode(email, NON_ALPHANUMERIC).to_string();
    let issuer_name = utf8_percent_encode("Bitbox", NON_ALPHANUMERIC).to_string();
    let uri = format!("oauth://totp/{}?secret={}&issuer={}", account_name, secret, issuer_name);

    Ok((secret, uri))
}

/**
 * Générer un QR code pour le 2FA
 * @param uri : l'URI TOTP
 * @return l'image du QR code en base64
 */
pub fn generate_totp_qr_code(uri: &str) -> Result<String, String> {
    let qr = QrCode::encode_text(uri, QrCodeEcc::High).map_err(|e| e.to_string())?;
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
fn qr_to_svg_string(qr: &QrCode, border: i32) -> String {
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

