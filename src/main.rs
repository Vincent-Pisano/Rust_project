#![allow(unused)]

use std::io;

mod account;

use crate::account::{client::*, account_trait::*};

fn main() {
    println!("Hello, world!");

    let client: Client = AccountTrait::new(String::from("Hippie"), "De merde".to_owned());
    println!("{} {}", client.username, client.password);

    let mut input = String::new();

    print!("Enter your username:");

    match io::stdin().read_line(&mut input) {
        Ok(__) => {
            println!("Your username is {}", input);
        },
        Err(e) => println!("Something went wrong: {}", e)
    }
}
