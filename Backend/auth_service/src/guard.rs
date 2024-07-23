extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use rocket::outcome::Outcome;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::State;
use uuid::Uuid;
use domain::models::User;
use domain::schema::users;
use crate::models::ClaimsType;
use crate::utils::{decode_jwt_no_secret, decode_token};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let token = cookies.get_private("token").map(|cookie| cookie.value().to_string());

        if let Some(token) = token {
            let pool = request.guard::<&State<Pool>>().await.succeeded().unwrap();
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => return Outcome::Error((Status::ServiceUnavailable, ())),
            };

            let claims = match decode_jwt_no_secret(&token) {
                Ok(data) => data,
                Err(_) => return Outcome::Error((Status::Unauthorized, ())),
            };

            let user = match users::table
                .filter(users::id.eq(claims.sub))
                .first::<User>(&mut conn) {
                Ok(user) => user,
                Err(_) => return Outcome::Error((Status::Unauthorized, ())),
            };

            match decode_token(&token, &user.jwt_secret) {
                Ok(data) => {
                    // Retour de l'utilisateur authentifié
                    Outcome::Success(AuthenticatedUser {
                        id: user.id,
                        email: user.email,
                        role: user.role.to_string(),
                    })
                },
                Err(_) => Outcome::Error((Status::Unauthorized, ())),
            }

        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}
