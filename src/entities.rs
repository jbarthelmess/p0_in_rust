#[derive(Clone)]
pub struct Client {
    pub id: i32,
    pub username: String
}

#[derive(Hash, Eq, Clone)]
pub struct Account {
    pub id: i32,
    pub amount_in_cents: i32,
    pub client_id: i32
}

impl PartialEq for Account {
    fn eq(&self, other: &Account) -> bool {
        self.id == other.id
    }
}