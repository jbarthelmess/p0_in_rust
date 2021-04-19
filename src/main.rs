use actix_web::{HttpServer, App};
pub mod entities;
mod db;
mod controller;

pub struct AppData {
    db: db::in_memory::Memory
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new()
        .data(AppData {
            db: db::in_memory::Memory::new()
        })
        .service(controller::get_clients))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
