#![allow(unused)]

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;
use std::fs::OpenOptions;
use std::process;

mod account;

use crate::account::{account_trait::*, client::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    check_if_login(
    if is_trying_to_log_in(args.clone()) {
        verify_credentials(args[2].clone(), args[3].clone())
    } else {
        None
    })
}

fn is_trying_to_log_in(args: Vec<String>) -> bool {
    args.len() >= 4 && args[1] == "login"
}

fn check_if_login(optional_client: Option<Client>) {
    let mut client:Client;

    match optional_client {
        None => {
            client = sign_up();
        }
        Some(optional_client) => {
            client = optional_client;
        }
    };
    //passer client en paramètre ?
    show_menu()
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

    print!("Enter your balance: \n");
    let mut balance = String::new();
    io::stdin()
        .read_line(&mut balance)
        .ok()
        .expect("Failed to read the line");

        let credentials = write_to_file(String::from(username.trim()), String::from(password.trim()), String::from(balance.trim()));
        verify_credentials(credentials[0], credentials[1]) //faire en sorte de renvoyer un Client non optionel
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

fn write_to_file(username: String, password: String, balance: String) -> (String, String) {
    let path = "very_secure_info.txt";
    let file_cnt = BufReader::new(File::open(path).expect("Unable to open file"));
    let mut cnt  = 0;

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

    (username, password)
}

fn verify_credentials(username: String, password: String) -> Option<Client>{
    let path = "very_secure_info.txt";
    let file_to_read = File::open(path).unwrap();
    let reader = BufReader::new(file_to_read);
    let mut client: Option<Client> = None;
    
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let data_v: Vec<&str> = line.split(',').collect();

        if data_v[1].to_string() == username && data_v[2].to_string() == password {
            client = Client::new(username, password, data_v[3].parse().unwrap());
            break;
        }
    }
    Some(client)


    // TO DO fix verify_credentials et sing_up poruq ue ca ne retourne pas un option
}

fn read_all_data() {
    let path = "very_secure_info.txt";
    let file_to_read = File::open(path).unwrap();
    let reader = BufReader::new(file_to_read);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        println!("{}", line);
    }
}

fn show_menu() {
    //choix à changer en enum ?
    println!("Hello {} !", optional_client.to_string());
    loop {
        println!(
            "\nWhat Would you like to do today ?
        1. Check your balance
        2. Make a deposit
        3. Make a withdrawal
        Q. Swallow your tears and leave you poor fuck",
        );
        enter_option(); 
    }
}

fn enter_option() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read the line");
    
    match input.to_ascii_lowercase().as_str().trim() {
        "1" => {
            println!("You are a broke mf");
        }
        "2" => {
            println!("Well nice, you finally got a job");
        }
        "3" => {
            println!("I said you are broke");
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
