
#![allow(unused)]
#![allow(clippy::all)]


use std::borrow::Cow;
use super::schema::{
     classes, projects, class_users, group_users, evaluations, evaluation_results, notifications, groups, auth_tokens  , registration_links , email_confirmations
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
use schemars::{JsonSchema , gen::SchemaGenerator , Schema};
use std::io::Read;
use crate::schema::users::jwt_secret;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct LogResponse {
    pub status: &'static str,
   pub  message: String,
}

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

impl From<String> for UserRole {
    fn from(role: String) -> Self {
        match role.as_str() {
            "Student" => UserRole::Student,
            "Trainer" => UserRole::Trainer,
            _ => UserRole::Trainer,
        }
    }
}
impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::Trainer => write!(f, "Trainer"),
            UserRole::Student => write!(f, "Student"),
        }
    }
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

#[derive(Serialize , Deserialize , ToSchema )]
pub struct Response {
    pub message: String,
}

#[derive(Serialize , Deserialize  , ToSchema)]
pub struct Error {
    pub error: String,
}

#[derive(Queryable, Debug, Identifiable , Deserialize, Serialize , Associations)]
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
 * NewAuthToken model
 * la table auth_tokens contient les informations des jetons d'authentification
 */
#[derive(Queryable, Debug, Insertable, Deserialize , Selectable)]
#[diesel(table_name = auth_tokens)]
pub struct NewAuthToken<'a> {
   // pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub token: &'a str,
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
#[derive(Queryable, Debug, Identifiable, Associations , Deserialize, Serialize )]
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

#[derive(Queryable,Debug, Identifiable , Deserialize, Serialize , Clone , Insertable )]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub jwt_secret: String,
    pub role: UserRole,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


/**
 * NewUser model
 * la table users contient les informations des utilisateurs
 */
#[derive(Queryable ,Insertable, Deserialize ,Serialize )]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password: &'a str,
    pub jwt_secret: &'a str,
    pub role: UserRole,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub id : Uuid,

}

#[derive(Queryable ,AsChangeset, Deserialize , Serialize , ToSchema)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
}



/**
 * UserDisplay model
 * la table users contient les informations des utilisateurs
 * il s'agit d'une version simplifiée de l'utilisateur
 * pour l'affichage
 */

#[derive(Queryable, Debug, Identifiable, Deserialize, Serialize , ToSchema)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct UserDisplay {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
impl From<User> for UserDisplay {
    fn from(user: User) -> Self {
        UserDisplay {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Queryable, Debug, Serialize, Deserialize , Selectable , Identifiable, Insertable)]
#[diesel(table_name = email_confirmations)]
#[diesel(check_for_backend(Pg))]
#[diesel(primary_key(id))]
pub struct EmailConfirmation {
    pub id: Uuid,
    pub email: String,
    pub token: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable ,Insertable, Deserialize ,Serialize )]
#[diesel(table_name = email_confirmations)]
pub struct NewEmailConfirmation<'a> {
    pub email: &'a str,
    pub token: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password_hash: &'a str,
    pub created_at: Option<NaiveDateTime>,
    pub expires_at: NaiveDateTime,
    pub id: Uuid,
}


/**
 * RegistrationLink model
 * la table registration_links contient les informations des liens d'inscription
    y compris les informations des utilisateurs
 */
#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = registration_links)]
pub struct RegistrationLink {  // model a revoir
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>,
    pub used_by: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub class_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub email: String,
    pub link: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/**
 * NewRegistrationLink model
 * la table registration_links contient les informations des liens d'inscription

 */
#[derive(Insertable, Deserialize)]
#[diesel(table_name = registration_links)]
pub struct NewRegistrationLink<'a> {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub token: &'a str,
    pub expires_at: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>,
    pub used_by: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub class_id: Option<Uuid>,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub role: UserRole,
    pub email: &'a str,
    pub link: &'a str,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}