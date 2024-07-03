
#![allow(unused)]
#![allow(clippy::all)]

use super::schema::{
     classes, projects, class_users, group_users, evaluations, evaluation_results, notifications, groups, auth_tokens
};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Result, value::RawValue};
use uuid::Uuid;
use diesel::pg::{Pg ,PgValue};
use diesel::serialize::{self, ToSql, Output};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::sql_types::Text;
use diesel::prelude::*;
use diesel::backend::{Backend};
use std::io::Write;
use std::fmt;
use utoipa::ToSchema;

use std::io::Read;


/**
 * UserRole enum
 * enumération des rôles des utilisateurs
 */
#[derive(DbEnum, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Ord, PartialOrd, ToSchema,)]
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    #[db_rename = "trainer"]
    Trainer,  // Formateur
    #[db_rename = "student"]
    Student,  // Étudiant
}

/*
impl FromSql<Text, Pg> for UserRole {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue<'_>>) -> deserialize::Result<Self> {
        match bytes {
            Some(bytes) => {
                let mut str_buf = String::new();
                bytes.as_bytes().read_to_string(&mut str_buf)?;
                match str_buf.as_ref() {
                    "Trainer" => Ok(UserRole::Trainer),
                    "Student" => Ok(UserRole::Student),
                    _ => Err("Unrecognized user role".into()),
                }
            },
            None => Err("Can't read SQL NULL value for UserRole".into()),
        }
    }
}
*/

#[derive(Queryable, Debug, Identifiable , Deserialize, Serialize)]
#[diesel(table_name = auth_tokens)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
pub struct AuthToken {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub token: String,
    pub created_at: Option<NaiveDateTime>,
    pub expires_at: NaiveDateTime,
}

/**
 * ClassUser model
 * la table class_users contient les informations des utilisateurs dans les classes
 */
#[derive(Queryable, Debug, Identifiable, Associations , Deserialize, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Class))]
#[diesel(table_name = class_users)]
#[diesel(primary_key(class_id, user_id))]
pub struct ClassUser {
    pub class_id: Uuid,
    pub user_id: Uuid,
}

/**
 * Class model
 * la table classes contient les informations des classes
 */
#[derive(Queryable, Debug, Identifiable, Deserialize, Serialize)] 
#[diesel(table_name = classes)]
#[diesel(primary_key(id))]
pub struct Class {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/**
 * EvaluationResult model
 * la table evaluation_results contient les informations des résultats des évaluations
 */
#[derive(Queryable, Debug, Identifiable, Associations, Deserialize, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = evaluation_results)]
#[diesel
(primary_key(id))]
pub struct EvaluationResult {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub average_score: f64,
    pub final_score: f64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/**
 * Evaluation model
 * la table evaluations contient les informations des évaluations
 */
#[derive(Queryable, Debug, Identifiable, Associations, Deserialize, Serialize )]
#[diesel(belongs_to(User, foreign_key = evaluator_id, foreign_key = evaluatee_id))]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Project))]
#[diesel(table_name = evaluations)]
#[diesel(primary_key(id))]
pub struct Evaluation {
    pub id: Uuid,
    pub evaluator_id: Option<Uuid>,
    pub evaluatee_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub score: i32,
    pub comments: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/**
 * GroupUser model
 * la table group_users contient les informations des utilisateurs dans les groupes
 */

#[derive(Queryable)]
#[derive(Debug, Identifiable, Associations , Deserialize, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = group_users)]
#[diesel(primary_key(group_id, user_id))]
pub struct GroupUser {
    pub group_id: Uuid,
    pub user_id: Uuid,
}
/**
 * Group model
 * la table groups contient les informations des groupes
 */
#[derive(Queryable, Debug , Identifiable , Deserialize, Serialize)]
#[diesel(table_name = groups)]
#[diesel(primary_key(id))]
pub struct Group {
    pub id: Uuid,
    pub group_name: String,
    pub project_id: Option<Uuid>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/**
 * Notification model
 * la table notifications contient les informations des notifications
 */
#[derive(Queryable, Debug, Identifiable, Associations , Deserialize, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = notifications)]
#[diesel(primary_key(id))]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub message: String,
    pub sent_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/**
 * Project model
 * la table projects contient les informations des projets
 */
#[derive(Queryable, Debug, Identifiable, Associations , Deserialize, Serialize)]
#[diesel(belongs_to(Class))]
#[diesel(table_name = projects)]
#[diesel(primary_key(id))]
pub struct Project {
    pub id: Uuid,
    pub project_name: String,
    pub class_id: Option<Uuid>,
    pub descriptions: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/**
 * User model
 * la table users contient les informations des utilisateurs
 */
use super::schema::users; 
#[derive(Queryable)]
#[derive(Debug, Identifiable , Deserialize, Serialize , Clone , Insertable)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


/**
 * NewUser model
 * la table users contient les informations des utilisateurs
 */
#[derive(Queryable ,Insertable, Deserialize ,Serialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password_hash: &'a str,
    pub role: UserRole,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable ,AsChangeset, Deserialize , Serialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<UserRole>,
    pub updated_at: Option<NaiveDateTime>,
}