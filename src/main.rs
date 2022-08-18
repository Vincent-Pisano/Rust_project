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

mod menu;
use crate::menu::{enter_password, show_menu, sign_up};

mod database;

fn main() {
    let args: Vec<String> = env::args().collect();

    read_credentials(args);
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
