use actix_web::{web, get, Responder, HttpResponse};
use serde_json;
//use crate::entities::{Client};
use crate::db::DAO;

#[get("/clients")]
async fn get_clients(data: web::Data<crate::AppData>) -> impl Responder {
    let clients = data.db.get_all_clients();
    let json = serde_json::to_string(&clients).unwrap();
    HttpResponse::Ok().body(json)
}

#[cfg(test)]
mod tests{

}