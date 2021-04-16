use std::collections::{HashMap};
use crate::entities::{Account, Client};
mod in_memory;

#[allow(dead_code)]
fn get_dao() -> impl DAO {
    in_memory::Memory {
        client_count: 0,
        client_data: HashMap::new(),
        account_count: 0,
        account_data: HashMap::new()
    }
}

pub trait DAO {
    // CREATE
    /// Create a client 
    fn create_client(&mut self, client: Client) -> Client;
    /// Create an account 
    fn create_account(&mut self, account: Account) -> Result<Account, i32>;

    // READ
    /// Get Client by Id
    fn get_client(&self, id: i32) -> Result<&Client, i32>;
    /// Get All accounts for a client
    fn get_accounts(&self, id: i32) -> Result<&HashMap<i32, Account>, i32>;

    //UPDATE
    /// Update Client info
    fn update_client(&mut self, client: Client) -> Result<Client, i32>;
    /// Update Account info
    fn update_account(&mut self, account: Account) -> Result<Account, i32>;

    // DELETE
    /// Delete Client
    fn delete_client(&mut self, client: Client) -> bool;
    /// Delete Account
    fn delete_account(&mut self, account: Account) -> bool;
}

///Testing suite for anything that implements the DAO trait
#[cfg(test)]
mod test {
    use super::*;

    fn initialize() -> impl DAO{
        // change this to whatever object implements DAO traits
        // all these tests should pass
        in_memory::Memory{
            client_count: 0,
            client_data: HashMap::new(),
            account_count: 0,
            account_data: HashMap::new()
        }
    }

    #[test]
    fn test_create_client() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        assert_ne!(0, client.id);
    }

    #[test]
    fn test_create_account_no_clients() {
        let mut db = initialize();
        let account = Account {
            id:0,
            amount_in_cents: 1,
            client_id: 1
        };
        match db.create_account(account) {
            Ok(_) => panic!("Not supposed to succeed"),
            Err(_) => println!("success!")
        }
    }

    #[test]
    fn test_create_account_for_client() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        assert_ne!(0, client.id);
        let account = Account {
            id:0,
            amount_in_cents: 1,
            client_id: client.id
        };
        match db.create_account(account) {
            Ok(account) => {
                assert_ne!(0, account.id);
                assert_eq!(client.id, account.client_id);
            },
            Err(_) => panic!("Should not have failed")
        }
    }

    #[test]
    fn test_create_account_for_bad_client_id() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        assert_ne!(0, client.id);
        let account = Account {
            id:0,
            amount_in_cents: 1,
            client_id: client.id+1
        };
        match db.create_account(account) {
            Ok(_) => panic!("Should have failed"),
            Err(_) => println!("success")
        }
    }

    #[test]
    fn test_get_client_bad_id() {
        let db = initialize();
        match db.get_client(1) {
            Ok(_) => panic!("should have failed"),
            Err(_) => println!("success")
        }
    }

    #[test]
    fn test_get_client_good() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        assert_ne!(0, client.id);
        match db.get_client(client.id) {
            Err(_) => panic!("should have succeeded"),
            Ok(saved_client) => {
                assert_eq!(saved_client.username, client.username)
            }
        }
    }

    #[test]
    fn test_get_accounts_bad_client_id() {
        let db = initialize();
        match db.get_accounts(1) {
            Ok(_) => panic!("should not have succeeded"),
            Err(_) => println!("success")
        }
    }

    #[test]
    fn test_get_accounts_good_client_id() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        match db.get_accounts(client.id) {
            Ok(accounts) => {
                assert_eq!(0, accounts.len())
            },
            Err(_) => panic!("should have succeeded")
        }
    }

    #[test]
    fn test_get_accounts_multiple() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        let mut accounts = Vec::new();
        for i in 0..5 {
            accounts.push(db.create_account(Account{id: 0, amount_in_cents: i, client_id: client.id}).unwrap());
        }
        let all_accounts = match db.get_accounts(client.id) {
            Ok(_accs) => _accs,
            Err(_) => panic!("Could not get accounts for client")
        };

        assert_eq!(all_accounts.len(), accounts.len());
        for i in accounts.iter() {
            match all_accounts.get(&i.id) {
                Some(acc) => {
                    assert_eq!(acc.id, i.id);
                    assert_eq!(acc.client_id, i.client_id);
                    assert_eq!(acc.amount_in_cents, i.amount_in_cents);
                },
                None => panic!("should have found account")
            }
        }
    }

    #[test]
    fn test_update_client_no_client() {
        let mut db = initialize();
        match db.update_client(Client{id: 1, username: String::from("hello")}) {
            Ok(_) => panic!("this should not have passed"),
            Err(_) => println!("success")
        }
    }

    #[test]
    fn test_update_client_good() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        match db.update_client(Client{id:client.id, username:String::from("jon_snow")}) {
            Ok(updated_client) => assert_eq!(updated_client.username, "jon_snow"),
            Err(_) => panic!("shouldn't have failed")
        }
    }

    #[test]
    fn test_update_account_no_client() {
        let mut db = initialize();
        match db.update_account(Account{id: 1, amount_in_cents: 4000000, client_id: 1}) {
            Ok(_) => panic!("shouldn't have succeeded"),
            Err(_) => println!("success")
        }
    }

    #[test]
    fn test_update_account_good() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        let mut accounts = Vec::new();
        for i in 0..5 {
            accounts.push(db.create_account(Account{id: 0, amount_in_cents: i, client_id: client.id}).unwrap());
        }
        for i in accounts.iter() {
            match db.update_account(Account{ amount_in_cents: i.amount_in_cents*4000, id:i.id, client_id:i.client_id}) {
                Ok(acc) => {
                    assert_eq!(acc.amount_in_cents, i.amount_in_cents*4000);
                },
                Err(_) => panic!("should've succeeded")
            }
        }
    }
    #[test]
    fn test_delete_client_bad() {
        let mut db = initialize();
        match db.delete_client(Client{id:1, username:String::from("john_doe")}) {
            true => panic!("should've failed"),
            false => println!("success")
        }
    }

    #[test]
    fn test_delete_client_good() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        match db.delete_client(client) {
            true => println!("success"),
            false => panic!("shouldn't have failed")
        }
    }

    #[test]
    fn test_delete_account_bad() {
        let mut db = initialize();
        match db.delete_account(Account{id:0, amount_in_cents: 1, client_id: 1}) {
            true => panic!("should've failed"),
            false => println!("success")
        }
    }

    #[test]
    fn test_delete_account_good() {
        let mut db = initialize();
        let client = Client {
            id: 0,
            username: String::from("john_doe")
        };
        let client = db.create_client(client);
        let mut accounts = Vec::new();
        for i in 0..5 {
            accounts.push(db.create_account(Account{id: 0, amount_in_cents: i, client_id: client.id}).unwrap());
        }
        while accounts.len() > 0 {
            let acc = accounts.pop().unwrap();
            match db.delete_account(acc) {
                true => println!("success"),
                false => panic!("should've succeeded")
            }
        }
    }
}