extern crate diesel;
extern crate dotenv;

use std::env;
use chrono::Utc;
use rocket::State;
use diesel::prelude::*;
use domain::models::{User, UserRole};
use rocket::outcome::Outcome;
use rocket::http::{ Status};
use rocket::request::{self, FromRequest, Request};
use uuid::Uuid;
use domain::schema::{auth_tokens, users};
use rocket_sync_db_pools::Error;
use common::db::Pool;
use crate::models::AuthConfig;
use crate::utils::{decode_jwt, decode_jwt_no_secret, decode_token};



pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub role: domain::models::UserRole,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token_cookie = request.cookies().get_private("token");
        if let Some(cookie) = token_cookie {
            let token = cookie.value();
            let pool = request.rocket().state::<Pool>().unwrap();
            let mut conn = pool.get().unwrap();


            let claims = match decode_jwt_no_secret(token) {
                Ok(data) => data,
                Err(_) => return Outcome::Error((Status::Unauthorized, ())),
            };

            let user_result: Result<(String, UserRole, String), _> = users::table
                .filter(users::id.eq(claims.sub))
                .select((users::jwt_secret, users::role, users::email))
                .first(&mut conn);

            match user_result {
                Ok((secret, role, email)) => {
                    match decode_token(token, &secret) {
                        Ok(token_data) => {
                            let now = Utc::now().timestamp() as usize;
                            if token_data.claims.exp < now {
                                diesel::delete(auth_tokens::table.filter(auth_tokens::token.eq(token)))
                                    .execute(&mut conn)
                                    .expect("Error deleting expired token");

                                return Outcome::Error((Status::Unauthorized, ()));
                            }
                            Outcome::Success(AuthenticatedUser {
                                id: claims.sub,
                                email,
                                role,
                            })
                        }
                        Err(_) => Outcome::Error((Status::Unauthorized, ())),
                    }
                }
                Err(_) => Outcome::Error((Status::Unauthorized, ())),
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}