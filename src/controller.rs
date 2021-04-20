use actix_web::{web, post, get, delete, put,  HttpResponse};
use crate::entities::{BankClient, Account};
use deadpool_postgres::{Client, Pool};
use crate::error::DBError;
use crate::db;

#[get("/clients")]
pub async fn get_clients(db_pool: web::Data<Pool>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let all_clients = db::get_all_clients(&pg_client).await?;
    Ok(HttpResponse::Ok().json(all_clients))
}

#[post("/clients")]
async fn create_client(db_pool: web::Data<Pool>, req_body: web::Json<BankClient>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let new_client = db::create_client(&pg_client, req_body.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_client))
}

#[get("/clients/{client_id}")]
async fn get_client(db_pool: web::Data<Pool>, web::Path((client_id,)): web::Path<(i32,)>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let old_client = db::get_client(&pg_client, client_id).await?;
    Ok(HttpResponse::Ok().json(old_client))
}

#[post("/clients/{client_id}/accounts")]
async fn create_account(db_pool: web::Data<Pool>, web::Path((client_id,)): web::Path<(i32,)>, req_body: web::Json<Account>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let mut acc = req_body.into_inner();
    acc.client_id = client_id;
    acc.account_id = 0;
    let new_account = db::create_account(&pg_client, acc).await?;
    Ok(HttpResponse::Created().json(new_account))
}

#[get("/clients/{client_id}/accounts")]
async fn get_accounts(db_pool: web::Data<Pool>, web::Path((client_id,)): web::Path<(i32,)>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let accounts = db::get_accounts(&pg_client, client_id).await?;
    Ok(HttpResponse::Ok().json(accounts))
}

#[get("/clients/{client_id}/accounts/{account_id}")]
async fn get_account(db_pool: web::Data<Pool>, web::Path((client_id, account_id)): web::Path<(i32, i32)>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let acc = Account {account_id, client_id, amount_in_cents: 0};
    let account = db::get_account(&pg_client, acc).await?;
    Ok(HttpResponse::Ok().json(account))
}

#[delete("/clients/{client_id}")]
async fn delete_client(db_pool: web::Data<Pool>, web::Path((client_id,)): web::Path<(i32,)>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let client = db::delete_client(&pg_client, client_id).await?;
    Ok(HttpResponse::Ok().json(client))
}

#[delete("/clients/{client_id}/accounts/{account_id}")]
async fn delete_account(db_pool: web::Data<Pool>, web::Path((client_id, account_id)): web::Path<(i32, i32)>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let acc = Account {account_id, client_id, amount_in_cents: 0};
    let account = db::delete_account(&pg_client, acc).await?;
    Ok(HttpResponse::Ok().json(account))
}

#[put("/clients/{client_id}")]
async fn update_client(db_pool: web::Data<Pool>, web::Path(client_id) : web::Path<i32>, req_body: web::Json<BankClient>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let mut client = req_body.into_inner();
    client.client_id = client_id;
    let client = db::update_client(&pg_client, client).await?;
    Ok(HttpResponse::Ok().json(client))
}

#[put("/clients/{client_id}/accounts/{account_id}")]
async fn update_account(db_pool: web::Data<Pool>, web::Path((client_id, account_id)) : web::Path<(i32, i32)>, req_body: web::Json<Account>) -> Result<HttpResponse, DBError> {
    let pg_client: Client = db_pool.get().await.map_err(DBError::PoolError)?;
    let mut account = req_body.into_inner();
    account.client_id = client_id;
    account.account_id = account_id;
    let account = db::update_account(&pg_client, account).await?;
    Ok(HttpResponse::Ok().json(account))
}

#[cfg(test)]
mod tests{

}