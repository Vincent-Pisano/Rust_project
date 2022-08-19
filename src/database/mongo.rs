use crate::account::client::*;
use crate::account::credentials::*;
use mongodb::sync::Collection;
use mongodb::{bson::doc, sync::Client as MongoClient};

pub fn modify_balance(credentials: &Credentials, new_balance: f32) {
    // sike
    let client_update = Client::new(
        credentials.username().to_string(),
        credentials.password().to_string(),
        new_balance,
    );
    let collection = get_collection();
    let result = get_collection().find_one_and_replace(
        doc! { "credentials": { "username" : credentials.username(),   "password": credentials.password()}},
        client_update,
        None);
}

pub fn verify_credentials(credentials: &Credentials) -> Option<Client> {
    let result = get_collection().find_one(
        doc! { "credentials": { "username" : credentials.username(),   "password": credentials.password()}},
        None,
    );
    match result {
        Ok(opt_client) => opt_client,
        Err(e) => None,
    }
}

pub fn register_new_client(credentials: &Credentials, balance: String) -> Client {
    let mongo_client = get_collection();
    let client_new = Client::new(
        credentials.username().to_string(),
        credentials.password().to_string(),
        balance.parse().unwrap(),
    );
    mongo_client.insert_one(client_new.clone(), None);
    client_new
}

fn get_collection() -> Collection<Client> {
    let client = MongoClient::with_uri_str("mongodb://localhost:27017").unwrap();
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .unwrap();

    // Get a handle to a database.
    let db = client.database("rust_project");
    // Get a handle to a collection in the database.
    db.collection::<Client>("client")
}
