use crate::account::client::*;

pub fn modify_balance(username: String, new_balance: f32) {
    // sike 
}

pub fn verify_credentials(username: &str, password: &str) -> Option<Client> {
    None
}

pub fn register_new_client(username: String, password: String, balance: String) -> Client {
    Client::new("username".to_string(), "password".to_string(), 1f32)
}
