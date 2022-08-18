mod file;
mod mongo;

use crate::account::client::*;

pub fn modify_balance(id: u32, new_balance: f32) {
    crate::database::file::modify_balance(id, new_balance);
}

pub fn verify_credentials(username: &str, password: &str) -> Option<Client> {
    crate::database::file::verify_credentials(username, password)
}

pub fn register_new_client(username: String, password: String, balance: String) -> Client {
    crate::database::file::register_new_client(username, password, balance)
}
