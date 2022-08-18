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

use crate::account::client::*;

static PATH: &str = "very_secure_info.txt";

static POS_CLIENT_ID: usize = 0;
static POS_USERNAME: usize = 1;
static POS_PASSWORD: usize = 2;
static POS_BALANCE: usize = 3;

// START MODIFY BALANCE

pub fn modify_balance(id: u32, new_balance: f32) {
    let mut new_data = String::new();
    let balance_string = new_balance.to_string() + "\n";

    get_all_clients_as_vector()
        .into_iter()
        .for_each(|mut client| {
            let client_id: u32 = client[POS_CLIENT_ID].clone().parse::<u32>().unwrap();
            if client_id == id {
                client[POS_BALANCE] = balance_string.clone();
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

fn get_all_clients_as_vector() -> Vec<Vec<String>> {
    let mut file_to_read = File::open(PATH).expect("File not found");
    let mut data = String::new();
    file_to_read
        .read_to_string(&mut data)
        .expect("Error while reading file");
    let mut data_vector: Vec<&str> = data.split('\n').collect();
    data_vector.pop();
    let mut data_vector_clients: Vec<Vec<String>> = Vec::new();
    for client_str in data_vector.iter() {
        let data_client: Vec<String> = client_str.split(',').map(|s| s.to_string()).collect();
        data_vector_clients.push(data_client.clone());
    }
    data_vector_clients
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

// VERIFY CREDENTIALS

pub fn verify_credentials(username: &str, password: &str) -> Option<Client> {
    create_data_file();

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

// SIGN_UP

pub fn register_new_client(username: String, password: String, balance: String) -> Client {
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

// SUPPLEMENT

fn read_lines<P>(filename: P) -> Enumerate<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().enumerate()
}
