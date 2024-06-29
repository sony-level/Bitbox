use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{User, NewUser};
use crate::schema::users::dsl::*;
use crate::DbPool;

pub async fn create_user(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> impl Responder {
    let conn = pool.get().expect("Failed to get DB connection from pool");
    let new_user = NewUser {
        email: user.email.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        role: user.role,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error inserting new user");

    HttpResponse::Ok().json(new_user)
}

pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().expect("Failed to get DB connection from pool");

    match users.find(user_id.into_inner()).first::<User>(&conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

pub async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>, user: web::Json<NewUser>) -> impl Responder {
    let conn = pool.get().expect("Failed to get DB connection from pool");

    match diesel::update(users.find(user_id.into_inner()))
        .set((
            email.eq(&user.email),
            first_name.eq(&user.first_name),
            last_name.eq(&user.last_name),
            role.eq(user.role),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&conn)
    {
        Ok(_) => HttpResponse::Ok().json(user.into_inner()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update user"),
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = pool.get().expect("Failed to get DB connection from pool");

    match diesel::delete(users.find(user_id.into_inner())).execute(&conn) {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete user"),
    }
}
