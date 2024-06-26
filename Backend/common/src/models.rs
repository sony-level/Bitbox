use super::schema::{
    users, classes, projects, class_users, group_users, evaluations, evaluation_results, notifications,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::pg::Pg;
use diesel::serialize::{self, ToSql};
use diesel::pg::types::PgValue;
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Text;
use diesel::prelude::*;
// use diesel::associations::HasTable;

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum UserRole {
    #[diesel(rename = "trainer")]
    Trainer,
    #[diesel(rename = "student")]
    Student,
}

impl<DB: diesel::backend::Backend> ToSql<Text, DB> for UserRole {
    fn to_sql<W: std::io::Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        match *self {
            UserRole::Trainer => out.write_all(b"trainer".as_ref())?,
            UserRole::Student => out.write_all(b"student".as_ref())?,
        }
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Pg> for UserRole {
    fn from_sql(value: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"trainer" => Ok(UserRole::Trainer),
            b"student" => Ok(UserRole::Student),
            _ => Err(deserialize::Error::Custom("Unrecognized enum variant".into())),
        }
    }
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Class)]
#[table_name = "class_users"]
#[primary_key(class_id, user_id)]
pub struct ClassUser {
    pub class_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "classes"]
#[primary_key(id)]
pub struct Class {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "evaluation_results"]
#[primary_key(id)]
pub struct EvaluationResult {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub average_score: f64,
    pub final_score: f64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[belongs_to(User, foreign_key = "evaluator_id")]
#[belongs_to(User, foreign_key = "evaluatee_id")]
#[belongs_to(Project)]
#[table_name = "evaluations"]
#[primary_key(id)]
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

#[derive(Queryable, Debug, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Group)]
#[table_name = "group_users"]
#[primary_key(group_id, user_id)]
pub struct GroupUser {
    pub group_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "groups"]
#[primary_key(id)]
pub struct Group {
    pub id: Uuid,
    pub group_name: String,
    pub project_id: Option<Uuid>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "notifications"]
#[primary_key(id)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub message: String,
    pub sent_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[belongs_to(Class)]
#[table_name = "projects"]
#[primary_key(id)]
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

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "users"]
#[primary_key(id)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
