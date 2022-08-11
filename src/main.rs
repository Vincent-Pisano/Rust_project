#![allow(unused)]

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;

mod account;

use crate::account::{account_trait::*, client::*};

fn main() {
    let args: Vec<String> = env::args().collect();

    check_if_login(if is_trying_to_log_in(args.clone()) {
        client_exist_in_file(args[2].clone(), args[3].clone())
    } else {
        None
    })
}

fn client_exist_in_file(username: String, password: String) -> Option<Client> {
    //checker dans fichier si le client est found

    //if found : //récuper les infos :
    Some(AccountTrait::new(username, password))
    //else :
    //println!("The information entered weren't found...")
    //None
    //None
}

fn check_if_login(optional_client: Option<Client>) {
    match optional_client {
        None => {
            println!("Sign up biche")
            //sign_up
        }
        Some(optional_client) => {
            println!("Hello {}, welcome again !", optional_client.to_string());
            show_menu()
        }
    };
}

fn show_menu() {
    //choix à changer en enum ?
    println!(
        "\nWhat Would you like to do today ?
    1. Check your balance
    2. Make a deposit
    3. Make a withdrawal
    Q. Swallow your tears and leave you poor fuck",
    );

    //faire une boucle do while tant que le input n'est pas "Q"
    enter_option(); //faire en sorte qu'il retourne le input
}

fn is_trying_to_log_in(args: Vec<String>) -> bool {
    args.len() >= 4 && args[1] == "login"
}

fn enter_option() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read the line");

    match input.as_str().trim() {
        "1" => {
            println!("You are a broke mf");
        }
        "2" => {
            println!("Well nice, you finally got a job");
        }
        "3" => {
            println!("I said you are broke");
        }
        "3" => {
            println!("Bye");
        }
        _ => {
            println!("That's not a valid choice");
        }
    }
}
