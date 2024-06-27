
use super::schema::{
    users, classes, projects, class_users, group_users, evaluations, evaluation_results, notifications, groups,
};
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


// use diesel::associations::HasTable;

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum UserRole {
    #[serde(rename = "trainer")]
    Trainer,  // Formateur
    #[serde(rename = "student")]
    Student,  // Étudiant
}

/**
 * UserRole enum
 * enum pour les rôles des utilisateurs
 * trainer: formateur
 * student: étudiant
 * Cette implémentation est nécessaire pour convertir l'énumération UserRole en une valeur textuelle qui peut être stockée dans la base de données. 
 * Diesel utilise cette implémentation pour insérer  et mettre à jour les valeurs UserRole dans les colonnes de type Text. 
 * Diesel utilise également cette implémentation pour lire les valeurs de la base de données et les convertir en valeurs UserRole.
 */
impl<DB> ToSql<Text, DB> for UserRole
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let s = match *self {
            UserRole::Trainer => "trainer",
            UserRole::Student => "student",
        };
        out.write_all(s.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}

// Implémentation pour convertir SQL en UserRole
impl FromSql<Text, Pg> for UserRole {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        match std::str::from_utf8(value.as_bytes())? {
            "trainer" => Ok(UserRole::Trainer),
            "student" => Ok(UserRole::Student),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
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
