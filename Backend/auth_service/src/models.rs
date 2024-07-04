use schemars::JsonSchema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::Uuid as DieselUuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}


#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

