#![allow(unused)]

#[macro_use]
extern crate serde;

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
use crate::account::credentials::*;

mod menu;
use crate::menu::{enter_password, show_menu, sign_up};

mod database;
use mongodb::options::FindOptions;
// dependances for mongo
use mongodb::{bson::doc, sync::Client as MongoClient};
use mongodb::bson::{Document};


fn main() -> mongodb::error::Result<()> {

    let client = MongoClient::with_uri_str(
        "mongodb://localhost:27017",
    )?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Connected successfully.");
    // List the names of the databases in that cluster
    /*for db_name in client.list_database_names(None, None)? {
        println!("{}", db_name);
    }*/

    // Get a handle to a database.
    let db = client.database("rust_project");

    // Get a handle to a collection in the database.
    let collection = db.collection::<Client>("client");

    /*let docs = vec![
        Client::new("test0".to_string(), "pw".to_string(), 69f32 ),
        Client::new("test1".to_string(), "pw".to_string(), 69f32 ),
        Client::new("test2".to_string(), "pw".to_string(), 420f32 ),
    ];*/

    // Insert some books into the "mydb.books" collection.
   // collection.insert_many(docs, None)?;
    let username = "test0";
    //let result = collection.

    /*let filter = doc! { "credentials": { "username" : username, "password": "*"} };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = collection.find(filter, find_options);*/

    let result = collection.find_one(doc! { "credentials": { "username" : username,   "password": "$**"}}, None);
    match result {
        Ok(opt_client) => match opt_client {
            Some(client) => println!("Found : {} !", client.to_string()),
            None => println!("Couldn't find the client : {}", username),
        },
        Err(e) => println!("Error during the connexion with mongo : {}", e),
    }

    let args: Vec<String> = env::args().collect();

    read_credentials(args);
    
    Ok(())
}

// Check Args validity before showing menu

fn read_credentials(args: Vec<String>) {
    check_if_login(if is_trying_to_log_in(&args) {
        enter_password(&args[2])
    } else {
        None
    })
}

fn is_trying_to_log_in(args: &Vec<String>) -> bool {
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
