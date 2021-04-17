use actix_web::{get, Responder, HttpResponse};
//use serde::{Serialize};
//use crate::entities::{Account, Client};
//db: &impl super::db::DAO
#[get("/clients")]
async fn get_clients() -> impl Responder {
    //let clients = db.get_all_clients();
    //let clients = serde_json(&clients);
    HttpResponse::Ok().body("Hello World")
}

#[cfg(test)]
mod tests{

}