use actix_web::{HttpServer, App};
pub mod entities;
mod db;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new().service(controller::get_clients))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
