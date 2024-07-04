extern crate common;
extern crate domain;

use schemars::JsonSchema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::Uuid as DieselUuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use domain::models::UserRole;
//use domain::schema::UserRoles;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}


#[derive(Deserialize, Serialize, JsonSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub(crate) sub: Uuid,
    pub(crate) exp: usize,
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, JsonSchema)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub role: UserRole
}

