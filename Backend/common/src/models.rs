
#![allow(unused)]
#![allow(clippy::all)]

use super::schema::{
    users, classes, projects, class_users, group_users, evaluations, evaluation_results, notifications, groups,
};
use diesel_derive_enum::DbEnum;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
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



/**
 * UserRole enum
 * enumération des rôles des utilisateurs
 */
#[derive(
    DbEnum, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Ord, PartialOrd, ToSchema,
)]
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    #[db_rename = "trainer"]
    Trainer,  // Formateur
    #[db_rename = "student"]
    Student,  // Étudiant
}

/**
 * EvaluationType enum
 * enumération des types d'évaluations
 */
#[derive(
    DbEnum, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Ord, PartialOrd, ToSchema,
)]
#[ExistingTypePath = "crate::schema::sql_types::EvaluationType"]
pub enum EvaluationType {
    #[db_rename = "peer"]
    Peer,  // Évaluation par les pairs
    #[db_rename = "self_evaluation"]
    SelfEvaluation,  // Auto-évaluation
    #[db_rename = "trainer"]
    Trainer,  // Évaluation par le formateur
}

/**
 * ClassUser model
 * la table class_users contient les informations des utilisateurs dans les classes
 */
#[derive(Queryable, Debug, Identifiable, Associations)]
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
#[derive(Queryable, Debug, Identifiable)]
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
#[derive(Queryable, Debug, Identifiable, Associations)]
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
#[derive(Queryable, Debug, Identifiable, Associations)]
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
#[derive(Queryable, Debug, Identifiable, Associations)]
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
#[derive(Queryable, Debug, Identifiable)]
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
#[derive(Queryable, Debug, Identifiable, Associations)]
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
#[derive(Queryable, Debug, Identifiable, Associations)]
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
#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
