use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="bank_client")]
pub struct BankClient {
    pub client_id: i32,
    pub username: String
}

#[derive(Hash, Eq, Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="bank_account")]
pub struct Account {
    pub account_id: i32,
    pub amount_in_cents: i32,
    pub client_id: i32
}

impl PartialEq for Account {
    fn eq(&self, other: &Account) -> bool {
        self.account_id == other.account_id
    }
}