use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::*;

pub async fn create_user (pool: web::Data<PgPool>, data:web::Json<CreateUser>,) -> HttpResponse{
    let id = Uuid::new_v4();
    let result = sqlx::query!("INSERT INTO users (id, name, email) VALUES ($1, $2, $3)", id, data.name, data.email)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().json(User {
           id: Some(id),
            name: data.name.clone(),
            email: data.email.clone(),
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


// read single User
pub async fn get_user(pool: web::Data<PgPool>,id: web::Path<Uuid>,) -> HttpResponse {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        *id
    )
    .fetch_optional(pool.get_ref()).await;
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


// udate User 
pub async fn update_user(pool:web::Data<PgPool>, id: web::Path<Uuid>,data: web::Json<UpdateUser>) -> HttpResponse {
    let result = sqlx::query!(
        "UPDATE users SET name = COALESCE($1, name),
        email = COALESCE($2, email) WHERE id = $3",
        data.name,
        data.email,
        *id
    ).execute(pool.get_ref()).await;

    match result {
        Ok(res) if res.rows_affected() == 1 => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


// delete user in the database
pub async fn delete_user(pool: web::Data<PgPool>, id: web::Path<Uuid>,) -> HttpResponse {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        *id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() == 1 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


// handler with pagination 
// read all user here 
pub async fn get_users(pool: web::Data<PgPool>, query: web::Query<Pagination>) -> HttpResponse {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * limit;

    let users = sqlx::query_as!(
        User, 
        "SELECT id, name, email FROM users ORDER BY created_at desc limit $1 OFFSET $2",
        limit,
        offset
        )
        .fetch_all(pool.get_ref())
        .await;
    
    let total = sqlx::query!(
        "SELECT count(*) AS count FROM users"
    )
    .fetch_one(pool.get_ref())
    .await;

    match(users, total) {
        (Ok(data), Ok(total)) => {
            let total = total.count.unwrap_or(0);
            let total_page = (total as f64 / limit as f64).ceil() as i64;

            HttpResponse::Ok().json(PaginationResponse{
                data,
                page,
                limit,
                total,
                total_page
            })
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}