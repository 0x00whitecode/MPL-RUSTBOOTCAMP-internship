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
    // Get port from environment variable, fallback to 8080 locally
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Starting server on 0.0.0.0:{}", port);

    // Establish database connection
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::user_routes)
            // Optional: health check for Fly.io
            .route("/health", web::get().to(|| async { "OK" }))
    })
    // Bind to 0.0.0.0 so Fly.io can access it
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
