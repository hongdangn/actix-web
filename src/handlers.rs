// src/handlers.rs
use crate::models::User;
use actix_web::{web, HttpResponse, Responder};
use rusqlite::{params, Connection};

const DB_PATH: &str = "allusers.db";

pub async fn get_users() -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, age, email FROM users")
        .unwrap();
    let users_iter = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
                email: row.get(3)?,
            })
        })
        .unwrap();

    let users: Vec<User> = users_iter.map(|user| user.unwrap()).collect();
    HttpResponse::Ok().json(users)
}

pub async fn add_user(user: web::Json<User>) -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute(
        "INSERT INTO users (id, name, age, email) VALUES (?1, ?2, ?3, ?4)",
        params![user.id, user.name, user.age, user.email],
    )
    .unwrap();
    HttpResponse::Ok().body("User added successfully")
}

pub async fn update_user(user: web::Json<User>) -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute(
        "UPDATE users SET name = ?1, age = ?2, email = ?3 WHERE id = ?4",
        params![user.name, user.age, user.email, user.id],
    )
    .unwrap();
    HttpResponse::Ok().body("User updated successfully")
}

pub async fn delete_user(user_id: web::Path<i32>) -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute("DELETE FROM users WHERE id = ?1", params![*user_id])
        .unwrap();
    HttpResponse::Ok().body("User deleted successfully")
}
