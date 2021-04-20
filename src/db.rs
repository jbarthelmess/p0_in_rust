use crate::entities::{Account, BankClient};
use deadpool_postgres::Client;
use crate::error::DBError;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_all_clients(client: &Client) -> Result<Vec<BankClient>, DBError> {
    let stmt = "SELECT * FROM bank_client;";
    let stmt = client.prepare(&stmt).await.unwrap();
    Ok(client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| BankClient::from_row_ref(row).unwrap())
        .collect())
}

pub async fn create_client(client: &Client, new_client: BankClient) -> Result<BankClient, DBError> {
    let stmt = "INSERT INTO bank_client (username) values ($1) RETURNING $table_fields;";
    let stmt = stmt.replace("$table_fields", &BankClient::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();
    client
        .query(&stmt, &[&new_client.username])
        .await?
        .iter()
        .map(|row| BankClient::from_row_ref(row).unwrap())
        .collect::<Vec<BankClient>>()
        .pop()
        .ok_or(DBError::NotFound)
}

pub async fn get_client(client: &Client, id: i32) -> Result<BankClient, DBError> {
    let stmt = "SELECT * FROM bank_client WHERE client_id = $1;";
    let stmt = client.prepare(&stmt).await.unwrap();
    client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| BankClient::from_row_ref(row).unwrap())
        .collect::<Vec<BankClient>>()
        .pop()
        .ok_or(DBError::NotFound)
}

pub async fn create_account(client: &Client, account: Account) -> Result<Account, DBError> {
    let stmt = "INSERT INTO bank_account (amount_in_cents, client_id) values ($1, $2) RETURNING $table_fields;";
    let stmt = stmt.replace("$table_fields", &Account::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();
    client
        .query(&stmt, &[&account.amount_in_cents, &account.client_id])
        .await?
        .iter()
        .map(|row| Account::from_row_ref(row).unwrap())
        .collect::<Vec<Account>>()
        .pop()
        .ok_or(DBError::NotFound)
}

pub async fn get_accounts(client: &Client, id: i32) -> Result<Vec<Account>, DBError> {
    let stmt = "SELECT * FROM bank_account WHERE client_id = $1;";
    let stmt = client.prepare(&stmt).await.unwrap();
    Ok(client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| Account::from_row_ref(row).unwrap())
        .collect::<Vec<Account>>())
}

pub async fn get_account(client: &Client, account: Account) -> Result<Account, DBError> {
    let stmt = "SELECT * FROM bank_account WHERE client_id = $1 AND account_id = $2;";
    let stmt = client.prepare(&stmt).await.unwrap();
    client
        .query(&stmt, &[&account.client_id, &account.account_id])
        .await?
        .iter()
        .map(|row| Account::from_row_ref(row).unwrap())
        .collect::<Vec<Account>>()
        .pop()
        .ok_or(DBError::NotFound)
}