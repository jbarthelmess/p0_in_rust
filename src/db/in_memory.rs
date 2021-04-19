use std::collections::{HashMap};
use crate::entities::{Account, Client};

/// Struct for an in memory database implementation of the bank api db
/// Holds the clients and accounts in two HashMaps, both keyed on the client_id
pub struct Memory {
    pub client_count: i32,
    pub account_count: i32, 
    pub client_data: HashMap<i32, Client>,
    pub account_data: HashMap<i32, HashMap<i32, Account>>
}

impl Memory {
    pub fn new() -> Self {
        Self {
            client_count: 0,
            account_count: 0,
            client_data: HashMap::new(),
            account_data: HashMap::new()
        }
        
    }
}

impl super::DAO for Memory {
    fn create_client(&mut self, client: Client) -> Client {
        let mut inner_client = client;
        self.client_count += 1;
        inner_client.id = self.client_count;
        let ret = inner_client.clone();
        self.client_data.insert(self.client_count, inner_client);
        self.account_data.insert(self.client_count, HashMap::new());
        ret
    }

    fn create_account(&mut self, account: Account) -> Result<Account, i32> {
        //let client_id = account.client_id;
        let mut inner_account = account;
        match self.account_data.get_mut(&inner_account.client_id) {
            Some(accounts) => {
                self.account_count += 1;
                inner_account.id = self.account_count;
                let ret = inner_account.clone();
                accounts.insert(self.account_count, inner_account);
                return Ok(ret);
            },
            None => Err(-1)
        }
    }

    fn get_all_clients(&self) -> &HashMap<i32, Client> {
        return &self.client_data;
    }

    fn get_client(&self, id: i32) -> Result<&Client, i32> {
        match self.client_data.get(&id) {
            Some(client) => Ok(client),
            None => Err(-1)
        }
    }

    fn get_accounts(&self, id: i32) -> Result<&HashMap<i32, Account>, i32> {
        match self.account_data.get(&id) {
            Some(accounts) => Ok(accounts), 
            None => Err(-1)
        }
    }

    fn update_client(&mut self, client: Client) -> Result<Client, i32> {
        match self.client_data.get(&client.id) {
            Some(_) => {
                let ret = client.clone();
                self.client_data.insert(client.id, client);
                Ok(ret)
            },
            None => Err(-1)
        }
    }

    fn update_account(&mut self, account: Account) -> Result<Account, i32> {
        match self.account_data.get_mut(&account.client_id) {
            Some(accounts) => {
                match accounts.get(&account.id) {
                    Some(_) => {
                        let ret = account.clone();
                        accounts.insert(account.id, account);
                        Ok(ret)
                    },
                    None => Err(-1)
                }
            },
            None => Err(-2)
        }
    }

    fn delete_client(&mut self, client: Client) -> bool {
        match self.client_data.get(&client.id) {
            Some(_) => {
                self.client_data.remove(&client.id);
                self.account_data.remove(&client.id);
                true
            },
            None => false
        }
    }

    fn delete_account(&mut self, account: Account) -> bool {
        match self.account_data.get_mut(&account.client_id) {
            Some(accounts) => {
                match accounts.remove(&account.id) {
                    Some(_) => true,
                    None => false
                }
            }
            None => false
        }
    }
}
