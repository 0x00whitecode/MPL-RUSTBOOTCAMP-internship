use actix_web::{App, HttpServer, web};

mod db;
mod models;
mod handlers;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::user_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
