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
use crate::menu::read_credentials;

mod database;

fn main() {
    let args: Vec<String> = env::args().collect();

    read_credentials(&args);
}
