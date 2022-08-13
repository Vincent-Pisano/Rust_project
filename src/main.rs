#![allow(unused)]

use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;
use std::process;

use std::io::prelude::*;

mod account;

use crate::account::client::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    modify_balance(0, 420f32);

    check_if_login(if is_trying_to_log_in(args.clone()) {
        enter_password(args[2].clone())
    } else {
        None
    })
}

//not a bug, a feature | "It just works !" Tod Howard
fn enter_password(username: String) -> Option<Client> {
    print!("Enter your password: \n");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .ok()
        .expect("Failed to read the line");

    match verify_credentials(username, password.clone()) {
        None => {
            println!("No account were found withe those credentials...\nPlease Sign up");
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

fn sign_up() -> Client {
    create_data_file();

    print!("Enter your username: \n");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .ok()
        .expect("Failed to read the line");

    print!("Enter your password: \n");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .ok()
        .expect("Failed to read the line");

    match verify_credentials(username.clone(), password.clone()) {
        None => {
            print!("Enter your balance: \n");
            let mut balance = String::new();
            io::stdin()
                .read_line(&mut balance)
                .ok()
                .expect("Failed to read the line");

            write_to_file(
                String::from(username.trim()),
                String::from(password.trim()),
                String::from(balance.trim()),
            )
        }
        Some(optional_client) => {
            println!("Oops, it seems that the username is already taken...");
            sign_up()
        }
    }
}

fn create_data_file() {
    let path = "very_secure_info.txt";

    if !Path::new(path).exists() {
        let output = File::create(path);
        match output {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem creating file : {:?}", error);
            }
        };
    }
}

fn write_to_file(username: String, password: String, balance: String) -> Client {
    let path = "very_secure_info.txt";
    let file_cnt = BufReader::new(File::open(path).expect("Unable to open file"));
    let mut cnt = 0;

    for _ in file_cnt.lines() {
        cnt = cnt + 1;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    let data_string: String = format!("{},{},{},{}", cnt, username, password, balance);

    if let Err(e) = writeln!(file, "{}", data_string) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Client::new(cnt, username, password, balance.parse().unwrap())
}

fn verify_credentials(username: String, password: String) -> Option<Client> {
    let path = "very_secure_info.txt";
    let file_to_read = File::open(path).unwrap();
    let reader = BufReader::new(file_to_read);
    let mut opt_client: Option<Client> = None;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let data_v: Vec<&str> = line.split(',').collect();

        if data_v[1].to_string() == username.trim() && data_v[2].to_string() == password.trim() {
            opt_client = Some(Client::new(
                data_v[0].parse().unwrap(),
                username,
                password,
                data_v[3].parse().unwrap(),
            ));
            break;
        }
    }
    opt_client
}

fn modify_balance(id: u32, new_balance: f32) {
    let path = "very_secure_info.txt";
    let mut file_to_read = File::open(path).expect("File not found");
    //let reader = BufReader::new(file_to_read);

    let mut data = String::new();
    file_to_read
        .read_to_string(&mut data)
        .expect("Error while reading file");

    let mut data_v: Vec<&str> = data.split('\n').collect();
    data_v.pop();

    // changer la reference de data vers new apres la modif
    let mut data_v2: Vec<Vec<&str>> = Vec::new();
    for client_str in data_v.iter() {
        let data: Vec<&str> = client_str.split(',').collect();
        data_v2.push(data.clone());
    }
    println!("{:?}", data_v2);
    let mut new = String::new();
    dbg!(&data, &new);

    for mut client in data_v2 {
        println!("{}", client[0]);
        if client[0].parse::<u32>().unwrap() == id {
            println!("found");
            let balance = new_balance.clone();
            //client[3] = *balance.to_string().as_str();
            println!("{:?}", client);
            break;
        }
    }
}

fn show_menu(client: Client) {
    //choix à changer en enum ?
    println!("Hello {} !", client.to_string());
    loop {
        println!(
            "\nWhat Would you like to do today ?
        1. Check your balance
        2. Make a deposit
        3. Make a withdrawal
        Q. Swallow your tears and leave you poor fuck",
        );
        enter_option(&client);
    }
}

fn enter_option(mut client: &Client) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read the line");

    match input.to_ascii_lowercase().as_str().trim() {
        "1" => {
            println!("You currently have {}$", client.balance());
        }
        "2" => {
            println!("Please enter the amount you wish to deposit");
            input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("Failed to read the line");
            let amount: f32 = input.trim().parse::<f32>().unwrap_or(-1f32);
            if amount > 0f32 {
                modify_balance(*client.id(), *client.balance() - amount)
            } else {
                println!("The entered value was invalid... Please enter a number above 0")
            }
            modify_balance(*client.id(), *client.balance() + amount)
        }
        "3" => {
            println!("Please enter the amount you wish to withdraw");
            input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("Failed to read the line");
            let amount: f32 = input.trim().parse::<f32>().unwrap_or(-1f32);

            if amount > 0f32 {
                if *client.balance() >= amount {
                    modify_balance(*client.id(), *client.balance() - amount)
                } else {
                    println!(
                            "It seems that you do not have the required money...\nYour current balance is {}$",
                            client.balance()
                        )
                }
            } else {
                println!("The entered value was invalid... Please enter a number above 0")
            }
        }
        "q" => {
            println!("Bye");
            process::exit(0);
        }
        _ => {
            println!("That's not a valid choice");
        }
    };
}
