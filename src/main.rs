#![allow(unused)]

mod account;

use crate::account::{client::*, account_trait::*};

fn main() {
    println!("Hello, world!");

    let client: Client = AccountTrait::new(String::from("Hippie"), "De merde".to_owned());
    println!("{} {}", client.username, client.password);
}
