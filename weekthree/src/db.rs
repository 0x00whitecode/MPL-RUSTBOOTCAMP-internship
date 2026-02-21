// use sqlx::PgPool;
// use dotenvy::dotenv;
// use std::env;


// pub async fn establish_connection() -> PgPool {
//     dotenv().ok();
    
//     let environment = env::var("ENV").unwrap_or_else(|_| "local".to_string());
    
//     let database_url = match environment.as_str() {
//         "prod" => env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set"),
//         "production" => env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set"),
//         _ => env::var("LOCAL_DATABASE_URL").expect("LOCAL_DATABASE_URL must be set"),
//     };
    
//     println!("Connecting to database in {} environment", environment);
    
//     PgPool::connect(&database_url).await.expect("Failed to connect to the database")
// }

use actix_web::{App, HttpServer, web};

mod db;
mod models;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Use Fly.io PORT if present, fallback to PROD_APP_PORT or 8080
    let port: u16 = std::env::var("PORT") // Fly.io sets this automatically
        .or_else(|_| std::env::var("PROD_APP_PORT")) // fallback to your PROD_APP_PORT env
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let host = std::env::var("PROD_APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    println!("Starting server on {}:{}", host, port);

    // Database connection
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::user_routes)
            .route("/health", web::get().to(|| async { "OK" })) // optional health check
    })
    .bind((host.as_str(), port))?  // bind to 0.0.0.0 or env host
    .run()
    .await
}
