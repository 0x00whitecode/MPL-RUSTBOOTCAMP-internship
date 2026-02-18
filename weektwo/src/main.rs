// use actix_multipart::Multipart;
use actix_web::{web,HttpResponse, App, HttpServer};
// use futures_util::StreamExt;
// use tokio::fs::File;
// use tokio::io::AsyncWriteExt;



async fn handler(auth: web::Header<Authorization<String>>) -> HttpResponse {
    let token = auth.into_inner().as_str();
    HttpResponse::Ok().body(token)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port 8080");
    HttpServer::new(|| {
        App::new()
        .route("/handler", web::get().to(handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}