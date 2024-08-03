extern crate common;
extern crate domain;



use crate::utils::{decode_token, is_password_strong};
use rocket::response::status::{Custom};

use crate::models::{AuthConfig, CreateStudentRequest};
use domain::models::{User, NewUser, UserRole, NewAuthToken, Response, Error, EmailConfirmation, NewEmailConfirmation, AuthToken};
use domain::schema::*;
use diesel::prelude::*;
use rocket::http::{Cookie, CookieJar, Status};
use common::db::Pool;
use rocket::State;
use crate::models::{ RegisterRequest, LoginRequest};
use crate::utils::{hash_password, generate_token, verify_password, send_confirmation_email, generat_token, generate_jwt, decode_jwt};
use chrono::Utc;
use rocket::serde::json::{Json};
use domain::schema::auth_tokens;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use uuid::Uuid;
use crate::guard::AuthenticatedUser;



/**
* connecter un utilisateur
* @param login_request : la demande de connexion
* @param pool : la connexion à la base de données
* @return l'utilisateur connecté
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
* $ curl -X POST http://localhost:8000/api/v1/register -H "Content-Type: application/json" -d '{"email": "test", "password": "test", "first_name": "test", "last_name": "test"}
*/
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Authentication",
    request_body(content = RegisterRequest),
    responses(
        (status = 200, description = "Confirmation email sent", body = Response),
        (status = 400, description = "Invalid input", body = Error),
        (status = 500, description = "Internal server error", body = Error),
        (status = 409, description = "User already exists", body = Error),
        (status = 503, description = "Service unavailable", body = Error) ,
    )
)]
#[post("/register", format = "application/json", data = "<register_request>")]
pub fn register(register_request: Json<RegisterRequest>, pool: &State<Pool>,  config: &State<AuthConfig>) -> Result<Json<Response>, Custom<Json<Error>>> {
    if register_request.0.email.is_empty() || register_request.0.password.is_empty() || register_request.0.first_name.is_empty() || register_request.0.last_name.is_empty() {
        return Err(Custom(Status::BadRequest, Json(Error { error: "Invalid input".to_string() })))
    }
    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            println!("Error getting database connection");
            return Err(Custom(Status::ServiceUnavailable, Json(Error { error: "Service unavailable".to_string() })));
        }
    };
    let existing_user_count = users::table
        .filter(users::email.eq(&register_request.0.email))
        .count()
        .get_result::<i64>(&mut conn)
        .expect("Error counting users");
    if existing_user_count > 0 {
        return Err(Custom(Status::Conflict, Json(Error { error: "Un utilisateur existe déja avec cet Email".to_string() })));
    }
    if !is_password_strong(&register_request.0.password) {
        return Err(Custom(Status::BadRequest, Json(Error { error: "Password is not strong enough".to_string() })));
    }
    let user_id = Uuid::new_v4();
    let email_confirmation_token = generate_jwt(user_id, &config.jwt_secret, 30);

    let hashed_password = hash_password(&register_request.0.password);



    match send_confirmation_email(&register_request.0.email, &email_confirmation_token) {
        Ok(_) => println!("Confirmation email sent"),
        Err(e) => {
            println!("Error sending confirmation email: {:?}", e);
            return Err(Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() })));
        }
    }
    let new_email_confirmation = NewEmailConfirmation {
        id: user_id,
        email: &register_request.0.email,
        token: &email_confirmation_token,
        first_name: &register_request.0.first_name,
        last_name: &register_request.0.last_name,
        password_hash: &hashed_password,
        created_at: Some(Utc::now().naive_utc()),
        expires_at : Utc::now().naive_utc() + chrono::Duration::minutes(30),
    };
    match diesel::insert_into(email_confirmations::table)
        .values(&new_email_confirmation)
        .execute(&mut conn) {
        Ok(_) => println!("Email confirmation token inserted"),
        Err(e) => {
            println!("Error saving email confirmation token: {:?}", e);
            return Err(Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() })));
        }
    }

    Ok(Json(Response { message: "User registered successfully, please check your email to confirm registration .".to_string() }))
}

/**
* confirmer l'inscription
* @param token : le token de confirmation
* @param pool : la connexion à la base de données
* @return statut de la confirmation
* @throws Unauthorized si le token est invalide ou expiré
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
* $ curl -X GET http://localhost:8000/api/v1/confirm_registration?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkMzIwZjQzZi1mZjQwLTQwZjYtYjIwZi1mZjQwZjYtYjIwZi1mZjQwZjYtYjIwZiIsImV4cCI6MTYyNjQwNjYwMn0.1Z6Z9J
*/
#[utoipa::path(
    get,
    path = "/auth/confirm_registration",
    tag = "Authentification",
    responses(
        (status = 200, description = "User confirmed", body = Response),
        (status = 401, description = "Invalid or expired token", body = Error),
        (status = 500, description = "Internal server error", body = Error),
        (status = 503, description = "Service unavailable", body = Error) ,
    )
)]
#[rocket::get("/confirm_registration?<token>")]
pub fn token_confirm(token: String, pool: &State<Pool>, config: &State<AuthConfig> ) -> Result<Json<Response>, Custom<Json<Error>>> {
    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            println!("Error getting database connection");
            return Err(Custom(Status::ServiceUnavailable, Json(Error { error: "Service unavailable".to_string() })));
        }
    };
    let claims = match decode_jwt(
        &token,
        &config.jwt_secret,
    ) {
        Ok(data) => data.claims,
        Err(_) => {
            println!("Invalid or expired token");
            return Err(Custom(Status::Unauthorized, Json(Error { error: "Invalid or expired token".to_string() })));
        }
    };
    let email_confirmation = match email_confirmations::table
        .filter(email_confirmations::token.eq(&token))
        .select(EmailConfirmation::as_select())
        .first::<EmailConfirmation>(&mut conn) {
        Ok(ec) => ec,
        Err(_) => {
            println!("Invalid or expired token");
            return Err(Custom(Status::Unauthorized, Json(Error { error: "Invalid or expired token".to_string() })));
        }
    };
    if Utc::now().naive_utc() > email_confirmation.expires_at {
        println!("Token has expired");
        return Err(Custom(Status::Unauthorized, Json(Error { error: "Token has expired".to_string() })));
    }

    let email = email_confirmation.email.clone();
    let first_name = email_confirmation.first_name.clone();
    let last_name = email_confirmation.last_name.clone();
    let pass = email_confirmation.password_hash.clone();

    diesel::delete(email_confirmations::table.filter(email_confirmations::id.eq(email_confirmation.id)))
        .execute(&mut conn)
        .expect("Error deleting email confirmation token");

    let jwt_secret = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect::<String>();

    let new_user = NewUser {
        id :  claims.sub,
        email: &email,
        first_name: &first_name,
        last_name: &last_name,
        password: &pass,
        jwt_secret: &jwt_secret,
        role: UserRole::Trainer,
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };
    let user_id = match diesel::insert_into(users::table)
        .values(&new_user)
        .returning(users::id)
        .get_result::<Uuid>(&mut conn) {
        Ok(id) => id,
        Err(e) => {
            println!("Error saving new user: {:?}", e);
            return Err(Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() })));
        }
    };
    let auth_token = generate_token(&new_user);
    let new_auth_token = NewAuthToken {
        user_id: Some(user_id),
        token: &auth_token,
        created_at: Some(Utc::now().naive_utc()),
        expires_at: Utc::now().naive_utc() + chrono::Duration::hours(24),
    };

    match diesel::insert_into(auth_tokens::table)
        .values(&new_auth_token)
        .execute(&mut conn) {
        Ok(_) => println!("Auth token inserted"),
        Err(e) => {
            println!("Error saving auth token: {:?}", e);
            return Err(Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() })));
        }
    }
    Ok(Json(Response { message: "User confirmed.".to_string() }))
}


/**
* connecter un utilisateur
* @param login_request : la demande de connexion
* @param pool : la connexion à la base de données
* @return l'utilisateur connecté
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
* $ curl -X POST http://localhost:8000/api/v1/login -H "Content-Type: application/json" -d '{"email": "test", "password": "test"}' , "Cookie: token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.TJVA95OrM7E2cBab30RMHrHDcEfxjoYZgeFONFh7HgQ"
*/
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Authentification",
    request_body(content = LoginRequest),
    responses(
        (status = 200, description = "Login successful", body = Response),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/login", format = "application/json", data = "<login_request>")]
pub fn login(login_request: Json<LoginRequest>, pool: &State<Pool>, cookies: &CookieJar<'_> , config: &State<AuthConfig>) -> Result<Json<Response>, Custom<Json<Error>>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, Json(Error { error: "Service Unavailable".to_string() })))?;

    let user = users::table
        .filter(users::email.eq(&login_request.email))
        .first::<User>(&mut conn)
        .map_err(|_| Custom(Status::Unauthorized, Json(Error { error: "Invalid credentials".to_string() })))?;

    match verify_password(&login_request.password, &user.password) {
        Ok(is_valid) => {
            if !is_valid {
                return Err(Custom(Status::Unauthorized, Json(Error { error: "Invalid credentials for password".to_string() })));
            }
        }
        Err(_) => {
            return Err(Custom(Status::Unauthorized, Json(Error { error: "Invalid credentials for login".to_string() })));
        }
    }

    let token = generat_token(&user);
    diesel::delete(auth_tokens::table.filter(auth_tokens::user_id.eq(user.id)))
        .execute(&mut conn)
        .map_err(|_| Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() })))?;

    let new_auth_token = NewAuthToken {
        user_id: Some(user.id),
        token: &token,
        created_at: Some(Utc::now().naive_utc()),
        expires_at: Utc::now().naive_utc() + chrono::Duration::hours(24),
    };

    diesel::insert_into(auth_tokens::table)
        .values(&new_auth_token)
        .execute(&mut conn)
        .map_err(|_| Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() })))?;

    let mut cookie = Cookie::new("token", token);
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_max_age(time::Duration::hours(24));
    cookies.add_private(cookie);

    Ok(Json(Response { message: "Login successful".to_string() }))

}



/**
* se deconnecter
* @param pool : la connexion à la base de données
* @return statut de la deconnexion
* @throws InternalServerError si la connexion à la base de données ne fonctionne pas
* @see establish_connection
*$ curl -X GET http://localhost:8000/api/v1/logout -H "Cookie: token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.TJVA95OrM7E2cBab30RMHrHDcEfxjoYZgeFONFh7HgQ"
*/
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "User logged out"),
    )
)]
#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>, pool: &State<Pool>, user: AuthenticatedUser) -> Result<Json<Response>, Custom<Json<Error>>> {
    let token_cookie = cookies.get_private("token");

    if let Some(token_cookie) = token_cookie {
        let token_value = token_cookie.value().to_string();
        let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, Json(Error { error: "Service Unavailable".to_string() })))?;

        let auth_token_result = auth_tokens::table
            .filter(auth_tokens::token.eq(&token_value))
            .filter(auth_tokens::user_id.eq(user.id))
            .first::<AuthToken>(&mut conn)
            .optional()
            .map_err(|_| Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() })))?;

        if let Some(_) = auth_token_result {
            diesel::delete(auth_tokens::table.filter(auth_tokens::token.eq(&token_value)))
                .execute(&mut conn)
                .map_err(|_| Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() })))?;

            cookies.remove_private(Cookie::build("token"));
        }
    }

    Ok(Json(Response { message: "User logged out".to_string() }))
}



/**
 * route protegee
 * @param user : l'utilisateur connecté
 * @return le message de bienvenue
 * @throws Unauthorized si l'utilisateur n'est pas connecté
 * @see AuthenticatedUser
 * $ curl -X GET http://localhost:8000/api/v1/protected
 */
#[utoipa::path(
    get,
    path = "/protected",
    tag = "Protected",
    responses(
        (status = 200, description = "Access granted"),
        (status = 401, description = "Unauthorized"),
    )
)]
#[get("/protected")]
pub fn protected_route(user: AuthenticatedUser) -> String {
    format!("Welcome, {}. Your role is {}.", user.email, user.role)
}


/**
    * le trainer creeajoute un student
    * @param pool : la connexion à la base de données
    * @param user : l'utilisateur connecté
    * @param student : le student à ajouter
    * @return le student ajoute
    * @throws Unauthorized si l'utilisateur n'est pas connecté
    * @throws NotFound si le trainer n'existe pas
    * @throws InternalServerError si la connexion à la base de données ne fonctionne pas
    * @see AuthenticatedUser
    * $ curl -X POST http://localhost:8000/api/v1/add_student -H "Content-Type: application/json" -d '{"email": "test", "first_name": "test", "last_name": "test"}' -H "Cookie: token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.TJVA95OrM7E2cBab30RMHrHDcEfxjoYZgeFONFh7HgQ"
    */

#[utoipa::path(
    post,
    path = "/students/create",
    tag = "User",
    request_body(content = CreateStudentRequest),
    responses(
        (status = 201, description = "Student created successfully", body = Response),
        (status = 400, description = "Invalid input", body = Error),
        (status = 500, description = "Internal server error", body = Error),
    )
)]
#[post("/students/create", format = "application/json", data = "<student_request>")]
pub fn create_student(student_request: Json<CreateStudentRequest>, pool: &State<Pool>, user: AuthenticatedUser) -> Result<Json<Response>, Custom<Json<Error>>> {
    if user.role != UserRole::Trainer {
        return Err(Custom(Status::Unauthorized, Json(Error { error: "Only trainers can create students.".to_string() })));
    }

    if student_request.0.email.is_empty() || student_request.0.first_name.is_empty() || student_request.0.last_name.is_empty() {
        return Err(Custom(Status::BadRequest, Json(Error { error: "Invalid input".to_string() })))
    }

    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, Json(Error { error: "Service unavailable".to_string() })))?;
    let existing_user_count = users::table.filter(users::email.eq(&student_request.0.email)).count().get_result::<i64>(&mut conn).expect("Error counting users");
    if existing_user_count > 0 {
        return Err(Custom(Status::Conflict, Json(Error { error: "Un utilisateur existe déjà avec cet Email".to_string() })));
    }

    let user_id = Uuid::new_v4();
    let jwt_secret = thread_rng().sample_iter(&Alphanumeric).take(64).map(char::from).collect::<String>();
    let new_student = NewUser {
        id: user_id,
        email: &student_request.0.email,
        first_name: &student_request.0.first_name,
        last_name: &student_request.0.last_name,
        password: "",
        jwt_secret: &jwt_secret,
        role: UserRole::Student,
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(users::table).values(&new_student).execute(&mut conn).map_err(|e| {
        println!("Error saving new student: {:?}", e);
        Custom(Status::InternalServerError, Json(Error { error: "Internal server error".to_string() }))
    })?;

    Ok(Json(Response { message: "Student created successfully.".to_string() }))
}


#[get("/auth/student_login?<token>")]
pub fn student_login(token: String, pool: &State<Pool>, config: &State<AuthConfig>) -> Result<Json<Response>, Custom<Json<Error>>> {
    let mut conn = pool.get().map_err(|_| Custom(Status::ServiceUnavailable, Json(Error { error: "Service unavailable".to_string() })))?;

    let claims = decode_token(&token, &config.jwt_secret).map_err(|_| Custom(Status::Unauthorized, Json(Error { error: "Invalid or expired token".to_string() })))?;

    let student = users::table
        .filter(users::id.eq(claims.claims.sub))
        .filter(users::role.eq(UserRole::Student))
        .first::<User>(&mut conn)
        .map_err(|_| Custom(Status::Unauthorized, Json(Error { error: "Invalid or expired token".to_string() })))?;

    Ok(Json(Response { message: format!("Student {} authenticated.", student.email) }))
}