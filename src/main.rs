use actix_web::{HttpServer, App};
use dotenv::dotenv;
use tokio_postgres::NoTls;
pub mod entities;
mod db;
mod controller;
mod config;
pub mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    HttpServer::new(move || App::new()
        .data(pool.clone())
        .service(controller::get_clients)
        .service(controller::get_client)
        .service(controller::create_client)
        .service(controller::create_account)
        .service(controller::get_accounts)
        .service(controller::get_account)
        .service(controller::delete_client)
        .service(controller::delete_account)
        .service(controller::update_client)
        .service(controller::update_account)
    )
    .bind(config.server_addr.clone())?
    .run()
    .await
}