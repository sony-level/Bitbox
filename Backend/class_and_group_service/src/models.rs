use chrono::NaiveDate;
use diesel::Insertable;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
//use domain::models::*;
use domain::schema::*;


#[derive(Insertable, Serialize, Deserialize , ToSchema)]
#[diesel(table_name = classes)]
pub struct NewClass<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub created_by: Option<Uuid>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = groups)]
pub struct NewGroup<'a> {
    pub group_name: &'a str,
    pub project_id: Option<Uuid>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub project_name: &'a str,
    pub class_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}
