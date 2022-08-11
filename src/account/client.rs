use super::account_trait::*;

pub struct Client {
    pub username: String,
    pub password: String,
}

impl AccountTrait for Client {
    // Constructor
    fn new(username: String, password: String) -> Client {
        Client { username, password }
    }

    fn login(username: String, password: String) -> bool {
        true
    }

    fn to_string(&self) -> String {
        format!("{} {}", self.username, self.password)
    }
}
