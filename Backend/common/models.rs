use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User, foreign_key = "teacher_id")]
#[table_name = "classes"]
pub struct Class {
    pub class_id: i32,
    pub class_name: String,
    pub description: Option<String>,
    pub teacher_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Class)]
#[table_name = "groups"]
pub struct Group {
    pub group_id: i32,
    pub group_name: String,
    pub class_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Group)]
#[belongs_to(User, foreign_key = "student_id")]
#[table_name = "group_members"]
pub struct GroupMember {
    pub group_member_id: i32,
    pub group_id: i32,
    pub student_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Class)]
#[belongs_to(User, foreign_key = "leader_id")]
#[table_name = "projects"]
pub struct Project {
    pub project_id: i32,
    pub project_name: String,
    pub description: Option<String>,
    pub class_id: i32,
    pub leader_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Class)]
#[table_name = "evaluations"]
pub struct Evaluation {
    pub evaluation_id: i32,
    pub class_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub max_score: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Evaluation)]
#[belongs_to(User, foreign_key = "student_id")]
#[table_name = "evaluation_results"]
pub struct EvaluationResult {
    pub evaluation_result_id: i32,
    pub evaluation_id: i32,
    pub student_id: i32,
    pub score: i32,
    pub feedback: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User, foreign_key = "recipient_id")]
#[belongs_to(User, foreign_key = "sender_id")]
#[table_name = "notifications"]
pub struct Notification {
    pub notification_id: i32,
    pub recipient_id: i32,
    pub sender_id: i32,
    pub message: String,
    pub read: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Class)]
#[table_name = "reports"]
pub struct Report {
    pub report_id: i32,
    pub class_id: i32,
    pub report_title: String,
    pub report_content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Evaluation)]
#[belongs_to(User, foreign_key = "evaluator_id")]
#[belongs_to(User, foreign_key = "evaluatee_id")]
#[table_name = "peer_evaluations"]
pub struct PeerEvaluation {
    pub peer_evaluation_id: i32,
    pub evaluation_id: i32,
    pub evaluator_id: i32,
    pub evaluatee_id: i32,
    pub score: i32,
    pub feedback: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
