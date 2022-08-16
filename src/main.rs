#![allow(unused)]

use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, Write};
use std::iter::Enumerate;
use std::ops::ControlFlow;
use std::path::Path;
use std::process;

mod account;
use crate::account::client::*;

static PATH: &str = "very_secure_info.txt";

static POS_CLIENT_ID: usize = 0;
static POS_USERNAME: usize = 1;
static POS_PASSWORD: usize = 2;
static POS_BALANCE: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    check_if_login(if is_trying_to_log_in(args.clone()) {
        enter_password(args[2].clone())
    } else {
        None
    })
}

fn enter_password(username: String) -> Option<Client> {
    let mut password = ask_input("password");

    match verify_credentials(&username, &password) {
        None => {
            println!("No account was found with those credentials...\nPlease Sign up");
            None
        }
        Some(optional_client) => Some(optional_client),
    }
}

fn is_trying_to_log_in(args: Vec<String>) -> bool {
    args.len() >= 3 && args[1] == "login"
}

fn check_if_login(optional_client: Option<Client>) {
    let mut client: Client;
    match optional_client {
        None => {
            client = sign_up();
        }
        Some(optional_client) => {
            client = optional_client;
        }
    };
    show_menu(client)
}

fn ask_input(input_name: &str) -> String {
    print!("Enter your {}: \n", input_name);
    let msg = format!("Failed to read the {}", input_name);
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect(&msg);
    input
}

fn sign_up() -> Client {
    create_data_file();
    let username = ask_input("username");
    let password = ask_input("password");

    match verify_credentials(&username, &password) {
        None => {
            let mut balance = ask_input("balance");

            register_new_client(
                String::from(username.trim()),
                String::from(password.trim()),
                String::from(balance.trim()),
            )
        }
        Some(optional_client) => {
            println!("Oops, it seems that username is already taken...");
            sign_up()
        }
    }
}

fn create_data_file() {
    if !Path::new(PATH).exists() {
        let output = File::create(PATH);
        match output {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem creating file : {:?}", error);
            }
        };
    }
}

fn register_new_client(username: String, password: String, balance: String) -> Client {
    let mut new_id = read_lines(PATH).count() + 1;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(PATH)
        .unwrap();

    let data_string: String = format!("{},{},{},{}", new_id, username, password, balance);

    if let Err(e) = writeln!(file, "{}", data_string) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Client::new(new_id as u32, username, password, balance.parse().unwrap())
}

fn verify_credentials(username: &str, password: &str) -> Option<Client> {
    let mut opt_client: Option<Client> = None;

    for (index, line) in read_lines(PATH) {
        if let Ok(client_str) = line {
            let line_vect: Vec<&str> = client_str.split(',').collect();

            let username_file: &str = line_vect[POS_USERNAME];
            let password_file: &str = line_vect[POS_PASSWORD];

            if username_file == username.trim() && password_file == password.trim() {
                let client_id: u32 = line_vect[POS_CLIENT_ID].parse().unwrap();
                let balance: f32 = line_vect[POS_BALANCE].parse().unwrap();

                opt_client = Some(Client::new(
                    client_id,
                    String::from(username),
                    String::from(password),
                    balance,
                ));
                break;
            }
        }
    }
    opt_client
}

fn read_lines<P>(filename: P) -> Enumerate<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().enumerate()
}

fn modify_balance(id: u32, new_balance: f32) {
    let mut new_data = String::new();
    let balance_string = new_balance.to_string() + "\n";

    get_all_clients_as_vector()
        .into_iter()
        .for_each(|mut client| {
            let client_id: u32 = client[POS_CLIENT_ID].clone().parse::<u32>().unwrap();
            if client_id == id {
                client[POS_BALANCE] = &balance_string.as_str();
            }

            let mut data_string = String::new();
            for (index, mut data_str) in client.iter().enumerate() {
                data_string = data_str.to_string();
                if index != client.len() - 1 {
                    data_string.push_str(",");
                }
                new_data.push_str(data_string.as_str());
            }
        });
    save_changes(new_data);
}

//
fn get_all_clients_as_vector() -> Vec<Vec<&'static str>> {
    let mut file_to_read = File::open(PATH).expect("File not found");
    let mut data = String::new();
    file_to_read
        .read_to_string(&mut data)
        .expect("Error while reading file");
    let mut data_vector: Vec<&str> = data.split('\n').collect();
    data_vector.pop();
    let mut data_vector_clients: Vec<Vec<&str>> = Vec::new();
    for client_str in data_vector.iter() {
        let data_client: Vec<&str> = client_str.split(',').collect();
        data_vector_clients.push(data_client.clone());
    }
    data_vector_clients //à corriger
}

fn save_changes(new_data: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(PATH)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", new_data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn show_menu(mut client: Client) {
    //choix à changer en enum ?
    println!("Hello {} !", client.to_string());
    loop {
        println!(
            "\nWhat Would you like to do today?
        1. Check your balance
        2. Make a deposit
        3. Make a withdrawal
        Q. Swallow your tears and leave you poor fuck",
        );
        client = enter_menu_option(client);
    }
}

fn enter_menu_option(mut client: Client) -> Client {
    match ask_input("line").to_ascii_lowercase().as_str().trim() {
        "1" => {
            println!("You currently have {}$", client.balance());
        }
        "2" => {
            println!("Please enter the amount you wish to deposit");
            client = enter_amount_to_modify(client, Operations::Add);
        }
        "3" => {
            println!("Please enter the amount you wish to withdraw");
            client = enter_amount_to_modify(client, Operations::Subtract);
        }
        "q" => {
            println!("Bye");
            process::exit(0);
        }
        _ => {
            println!("That's not a valid choice...");
        }
    };
    client
}

enum Operations {
    Add,
    Subtract,
}

fn enter_amount_to_modify(mut client: Client, operation: Operations) -> Client {
    let amount: f32 = ask_input("amount").trim().parse::<f32>().unwrap_or(-1f32);
    match amount > 0f32 {
        true => match operation {
            Operations::Add => add_amount_to_balance(client, amount),
            Operations::Subtract => remove_amount_from_balance(client, amount),
        },
        false => {
            println!("The entered value was invalid... Please enter a number above 0");
            client //return unchanged client
        }
    }
}

fn add_amount_to_balance(mut client: Client, amount: f32) -> Client {
    modify_balance(*client.id(), *client.balance() + amount);
    client.set_balance(*client.balance() + amount);
    client
}

fn remove_amount_from_balance(mut client: Client, amount: f32) -> Client {
    if *client.balance() >= amount {
        modify_balance(*client.id(), *client.balance() - amount);
        client.set_balance(*client.balance() - amount);
    } else {
        println!(
            "It seems that you do not have the required money...\nYour current balance is {}$",
            *client.balance()
        )
    }
    client
}
