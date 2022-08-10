use super::account_trait::*;

pub struct Client {pub username: String, pub password: String}

impl AccountTrait for Client{
    // Constructor
    fn new(username: String, password: String) -> Client {
        return Client{username, password};
    }

    fn login(username: String, password: String) -> bool {
        return true;
    }
}