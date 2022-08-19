mod file;
mod mongo;

use crate::account::client::*;
use crate::account::credentials::*;

pub fn modify_balance(credentials: &Credentials, new_balance: f32) {
    crate::database::mongo::modify_balance(credentials, new_balance);
}

pub fn verify_credentials(credentials: &Credentials) -> Option<Client> {
    crate::database::mongo::verify_credentials(credentials)
}

pub fn register_new_client(credentials: &Credentials, balance: String) -> Client {
    crate::database::mongo::register_new_client(credentials, balance)
}
